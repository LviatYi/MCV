use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Deserialize;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Air(command) => match command {
            AirCommand::Apply(args) => apply(args)?,
        },
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "mcv")]
#[command(about = "MCV command line interface.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Air(AirCommand),
}

#[derive(Debug, Subcommand)]
enum AirCommand {
    #[command(about = "Compose and distribute AIR assets for supported agent products.")]
    Apply(ApplyArgs),
}

#[derive(Debug, Parser)]
struct ApplyArgs {
    #[arg(short, long)]
    config: PathBuf,

    #[arg(long)]
    workspace: Option<PathBuf>,

    #[arg(long)]
    output: Option<PathBuf>,

    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Deserialize)]
struct CompositionConfig {
    schema: Option<String>,
    name: String,
    description: Option<String>,
    layers: Vec<String>,
    targets: Targets,
    rules: Option<Rules>,
}

#[derive(Clone, Debug, Deserialize)]
struct Targets {
    product: Product,
    scope: Option<ScopeInArg>,
    output_path: Option<PathBuf>,
    workspace_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum Product {
    Codex,
    Claude,
    Copilot,
}

impl Product {
    pub fn name(&self) -> &'static str {
        match self {
            Product::Codex => "Codex",
            Product::Claude => "Claude Code",
            Product::Copilot => "GitHub Copilot",
        }
    }

    pub fn instruction_file_path(&self, scope: &Scope) -> Result<PathBuf> {
        match (self, scope) {
            (Product::Codex, Scope::Global) => Ok(home_dir()?
                .join(".codex")
                .join(self.instruction_file_name())),
            (Product::Claude, Scope::Global) => Ok(home_dir()?
                .join(".claude")
                .join(self.instruction_file_name())),
            (Product::Copilot, Scope::Global) => Ok(home_dir()?
                .join(".copilot")
                .join(self.instruction_file_name())),
            (Product::Codex, Scope::Workspace(root)) => Ok(root.join(self.instruction_file_name())),
            (Product::Claude, Scope::Workspace(root)) => {
                Ok(root.join(self.instruction_file_name()))
            }
            (Product::Copilot, Scope::Workspace(root)) => {
                Ok(root.join(".github").join(self.instruction_file_name()))
            }
        }
    }

    pub fn instruction_file_name(&self) -> &'static str {
        match self {
            Product::Codex => "AGENTS.md",
            Product::Claude => "CLAUDE.md",
            Product::Copilot => "copilot-instructions.md",
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
enum ScopeInArg {
    #[default]
    Global,
    Workspace,
}

#[derive(Clone, Debug, Default)]
enum Scope {
    #[default]
    Global,
    Workspace(PathBuf),
}

impl From<Targets> for Scope {
    fn from(value: Targets) -> Self {
        match value.scope {
            Some(ScopeInArg::Global) | None => Scope::Global,
            Some(ScopeInArg::Workspace) => {
                let workspace_path = value.workspace_path.unwrap_or_else(default_working_dir);
                Scope::Workspace(workspace_path)
            }
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct Rules {
    separator: Option<String>,
}

fn apply(args: ApplyArgs) -> Result<()> {
    let config_path = fs::canonicalize(&args.config)
        .with_context(|| format!("failed to resolve config path: {}", args.config.display()))?;
    let config_dir = config_path
        .parent()
        .context("config path does not have a parent directory")?;
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config: {}", config_path.display()))?;
    let config: CompositionConfig = toml::from_str(&raw)
        .with_context(|| format!("failed to parse config: {}", config_path.display()))?;

    validate_config(&config)?;

    let content = compose_layers(&config, config_dir)?;
    let output_path = resolve_output_path(&config.targets, &args)?;

    if args.dry_run {
        println!("Config: {}", config_path.display());
        println!("Name: {}", config.name);
        if let Some(description) = &config.description {
            println!("Description: {description}");
        }
        println!("Product: {}", &config.targets.product.name());
        println!("Output: {}", output_path.display());
        println!("Bytes: {}", content.len());
        return Ok(());
    }

    let parent = output_path
        .parent()
        .context("resolved output path does not have a parent directory")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create output directory: {}", parent.display()))?;
    fs::write(&output_path, content)
        .with_context(|| format!("failed to write output file: {}", output_path.display()))?;

    println!(
        "Wrote {} prompt to {}",
        &config.targets.product.name(),
        output_path.display()
    );

    Ok(())
}

fn validate_config(config: &CompositionConfig) -> Result<()> {
    if config.layers.is_empty() {
        bail!("config '{}' must declare at least one layer", config.name);
    }

    if let Some(schema) = &config.schema
        && schema != "air.prompt-composition.v1"
    {
        bail!("unsupported schema '{}'", schema);
    }

    Ok(())
}

fn compose_layers(config: &CompositionConfig, config_dir: &Path) -> Result<String> {
    let separator = config
        .rules
        .as_ref()
        .and_then(|rules| rules.separator.clone())
        .unwrap_or_else(|| "\n\n".to_string());

    let mut parts = Vec::with_capacity(config.layers.len());
    for layer in &config.layers {
        let layer_path = config_dir.join(layer);
        let content = fs::read_to_string(&layer_path)
            .with_context(|| format!("failed to read layer: {}", layer_path.display()))?;
        parts.push(indent_markdown_headings(content.trim()));
    }

    let mut combined = format!("# {}", config.name);
    if !parts.is_empty() {
        combined.push_str(&separator);
        combined.push_str(&parts.join(&separator));
    }

    if !combined.ends_with('\n') {
        combined.push('\n');
    }

    Ok(combined)
}

fn indent_markdown_headings(input: &str) -> String {
    let mut lines = Vec::new();

    for line in input.lines() {
        if line.starts_with('#') {
            lines.push(format!("#{line}"));
        } else {
            lines.push(line.to_string());
        }
    }

    lines.join("\n")
}

fn resolve_output_path(targets: &Targets, args: &ApplyArgs) -> Result<PathBuf> {
    if let Some(output) = &args.output {
        return Ok(output.clone());
    }

    if let Some(output) = &targets.output_path {
        return Ok(output.clone());
    }

    let scope = match args.workspace.clone() {
        Some(root) => Scope::Workspace(root),
        None => Scope::from(targets.clone()),
    };

    targets.product.instruction_file_path(&scope)
}

fn home_dir() -> Result<PathBuf> {
    if let Some(user_profile) = env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(user_profile));
    }

    if let Some(home) = env::var_os("HOME") {
        return Ok(PathBuf::from(home));
    }

    bail!("failed to determine user home directory")
}

fn default_working_dir() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
