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
    assets_dir: PathBuf,

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
    #[serde(default, alias = "layers")]
    instruction_layers: Vec<String>,
    #[serde(default)]
    skill_layers: Vec<String>,
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
        Ok(match (self, scope) {
            (Product::Codex, Scope::Global) => home_dir()?.join(".codex"),
            (Product::Claude, Scope::Global) => home_dir()?.join(".claude"),
            (Product::Copilot, Scope::Global) => home_dir()?.join(".copilot"),
            (Product::Codex, Scope::Workspace(root)) => root.clone(),
            (Product::Claude, Scope::Workspace(root)) => root.clone(),
            (Product::Copilot, Scope::Workspace(root)) => root.join(".github"),
        }
        .join(self.instruction_file_name()))
    }

    pub fn instruction_file_name(&self) -> &'static str {
        match self {
            Product::Codex => "AGENTS.md",
            Product::Claude => "CLAUDE.md",
            Product::Copilot => "copilot-instructions.md",
        }
    }

    pub fn skills_root_path(&self, scope: &Scope) -> Result<PathBuf> {
        Ok(match (self, scope) {
            (Product::Codex, Scope::Global) => home_dir()?.join(".agents"),
            (Product::Claude, Scope::Global) => home_dir()?.join(".claude"),
            (Product::Copilot, Scope::Global) => home_dir()?.join(".copilot"),
            (Product::Codex, Scope::Workspace(root)) => root.join(".agents"),
            (Product::Claude, Scope::Workspace(root)) => root.join(".claude"),
            (Product::Copilot, Scope::Workspace(root)) => root.join(".github"),
        }
        .join("skills"))
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
    let config_name = config_path.file_stem();
    let assets_dir = fs::canonicalize(&args.assets_dir).with_context(|| {
        format!(
            "failed to resolve assets directory: {}",
            args.assets_dir.display()
        )
    })?;
    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config: {}", config_path.display()))?;
    let mut config: CompositionConfig = toml::from_str(&raw)
        .with_context(|| format!("failed to parse config: {}", config_path.display()))?;
    config.name = config_name
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_default();

    validate_config(&config)?;

    let scope = match args.workspace.clone() {
        Some(root) => Scope::Workspace(root),
        None => Scope::from(config.targets.clone()),
    };
    let instruction_output_path = if config.instruction_layers.is_empty() {
        None
    } else {
        Some(resolve_instruction_output_path(
            &config.targets,
            &scope,
            &args,
        )?)
    };
    let skills_output_root = resolve_skills_output_root(&config.targets, &scope)?;
    let instruction_content = if config.instruction_layers.is_empty() {
        None
    } else {
        Some(compose_instruction_layers(&config, &assets_dir)?)
    };
    let skills_root = assets_dir.join("skills");
    let selected_skills = resolve_skill_sources(&config.skill_layers, &skills_root)?;

    if args.dry_run {
        println!("Config: {}", config_path.display());
        println!("Assets Dir: {}", assets_dir.display());
        println!("Name: {}", config.name);
        if let Some(description) = &config.description {
            println!("Description: {description}");
        }
        println!("Product: {}", &config.targets.product.name());
        match (&instruction_content, &instruction_output_path) {
            (Some(content), Some(output_path)) => {
                println!("Instruction Output: {}", output_path.display());
                println!("Instruction Bytes: {}", content.len());
            }
            (None, None) => println!("Instruction Output: <disabled>"),
            _ => unreachable!("instruction output path and content should be present together"),
        }
        println!("Skills Output Root: {}", skills_output_root.display());
        if selected_skills.is_empty() {
            println!("Skills: <none>");
        } else {
            println!("Skills:");
            for skill in &selected_skills {
                println!(
                    "  - {} -> {}",
                    skill.name,
                    skills_output_root.join(&skill.name).display()
                );
            }
        }
        return Ok(());
    }

    if let (Some(content), Some(instruction_output_path)) =
        (instruction_content, instruction_output_path)
    {
        let parent = instruction_output_path
            .parent()
            .context("resolved instruction output path does not have a parent directory")?;
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output directory: {}", parent.display()))?;
        fs::write(&instruction_output_path, content).with_context(|| {
            format!(
                "failed to write instruction file: {}",
                instruction_output_path.display()
            )
        })?;
        println!(
            "Wrote {} instruction to {}",
            &config.targets.product.name(),
            instruction_output_path.display()
        );
    }

    if !selected_skills.is_empty() {
        fs::create_dir_all(&skills_output_root).with_context(|| {
            format!(
                "failed to create skills output directory: {}",
                skills_output_root.display()
            )
        })?;
        for skill in &selected_skills {
            let destination = skills_output_root.join(&skill.name);
            copy_skill_dir(&skill.source_path, &destination)?;
            println!(
                "Distributed skill {} to {}",
                skill.name,
                destination.display()
            );
        }
    }

    Ok(())
}

fn validate_config(config: &CompositionConfig) -> Result<()> {
    if config.instruction_layers.is_empty() && config.skill_layers.is_empty() {
        bail!(
            "config '{}' must declare at least one instruction_layers or skill_layers entry",
            config.name
        );
    }

    if let Some(schema) = &config.schema
        && schema != "air.distribution.v1"
        && schema != "air.prompt-composition.v1"
    {
        bail!("unsupported schema '{}'", schema);
    }

    Ok(())
}

fn compose_instruction_layers(config: &CompositionConfig, assets_dir: &Path) -> Result<String> {
    let separator = config
        .rules
        .as_ref()
        .and_then(|rules| rules.separator.clone())
        .unwrap_or_else(|| "\n\n".to_string());

    let mut parts = Vec::with_capacity(config.instruction_layers.len());
    for layer in &config.instruction_layers {
        let layer_path = assets_dir.join("instructions").join(layer);
        if !layer_path.is_file() {
            warn(&format!(
                "instruction layer '{}' is missing at {}, skipping",
                layer,
                layer_path.display()
            ));
            continue;
        }

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

fn resolve_instruction_output_path(
    targets: &Targets,
    scope: &Scope,
    args: &ApplyArgs,
) -> Result<PathBuf> {
    if let Some(output) = &args.output {
        return Ok(output.clone());
    }

    if let Some(output) = &targets.output_path {
        return Ok(output.clone());
    }

    targets.product.instruction_file_path(scope)
}

fn resolve_skills_output_root(targets: &Targets, scope: &Scope) -> Result<PathBuf> {
    targets.product.skills_root_path(scope)
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

#[derive(Debug)]
struct SelectedSkill {
    name: String,
    source_path: PathBuf,
}

fn resolve_skill_sources(
    skill_layers: &[String],
    skills_root: &Path,
) -> Result<Vec<SelectedSkill>> {
    let mut skills = Vec::with_capacity(skill_layers.len());

    for layer in skill_layers {
        let source_path = skills_root.join(layer);
        if !source_path.is_dir() {
            warn(&format!(
                "skill layer '{}' is missing at {}, skipping",
                layer,
                source_path.display()
            ));
            continue;
        }

        let name = source_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .with_context(|| {
                format!(
                    "failed to determine skill name for {}",
                    source_path.display()
                )
            })?;
        skills.push(SelectedSkill { name, source_path });
    }

    Ok(skills)
}

fn warn(message: &str) {
    eprintln!("warning: {message}");
}

fn copy_skill_dir(source: &Path, destination: &Path) -> Result<()> {
    if destination.exists() {
        fs::remove_dir_all(destination).with_context(|| {
            format!(
                "failed to remove existing skill directory: {}",
                destination.display()
            )
        })?;
    }

    copy_dir_recursive(source, destination)
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)
        .with_context(|| format!("failed to create directory: {}", destination.display()))?;

    for entry in fs::read_dir(source)
        .with_context(|| format!("failed to read directory: {}", source.display()))?
    {
        let entry =
            entry.with_context(|| format!("failed to read entry in {}", source.display()))?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry.file_type().with_context(|| {
            format!(
                "failed to determine file type for {}",
                source_path.display()
            )
        })?;

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else {
            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("failed to create parent directory: {}", parent.display())
                })?;
            }
            fs::copy(&source_path, &destination_path).with_context(|| {
                format!(
                    "failed to copy {} to {}",
                    source_path.display(),
                    destination_path.display()
                )
            })?;
        }
    }

    Ok(())
}
