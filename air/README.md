# AI-Resources Department | MCV

Dpt.AIR (AI-Resources Department) is responsible for the maintenance and rapid deployment of AI tool-flow assets.

AI tool-flow assets include all files and scripts applicable to adjusting, limiting, and optimizing AI tools.

The AI tools targeted primarily include Codex, Copilot, and Claude Code.

## [Skill Section](./skills)

Skill 资产具有 `Public` 与 `Private` 两种可见性。可见性是资产本身的分发属性，用于区分哪些资产默认可共享，哪些资产仅用于特定使用者、环境或项目。

- `Public` 资产是仓库默认可见的共享资产，可作为公开分发、组合和部署的基础内容。
- `Private` 资产是非公开共享资产，通常作为对 `Public` 资产的补充、覆盖或定制内容存在。

可见性与 harness、提示词结构或工作流编排无直接关系；它仅描述资产的存储与分发边界。

## [Deploy Section](./deploy)

## Best Practice Guide

### 优先使用稳定工具执行重复动作

相较于确定性算法在成本与质量上的优势，现阶段 LLM 尚无法与之匹敌。稳定工具的输出可预测、行为可复现、失败模式相对清晰。

应该将确定性强、重复性高、且已有成熟算法解决的问题从模型推理中剥离出来。

```prompt
- 优先使用稳定工具执行重复动作

使用如下策略对待存在现成工具可解决的重复动作，以减少模型在低价值重复任务上的参与：

- 若存在该工具方案在用户环境可用，则使用它。
- 若不可用，则进行不超过一次的工具部署建议。
- 若不可用且已进行过工具部署建议
  - 若高度怀疑该动作可由用户执行（非命令行方式，如 IDE），且输出不重要，则除非用户明确要求交给 agent 执行，直接跳过该动作。
  - 若后续动作依赖该输出，则视情况选用：尝试工具执行、提示用户执行或提交 token 由 agent 执行。

尽可能压缩稳定工具的反馈，且应该在构造命令行时即考虑压缩。其目的在于：

- 降低 token 消耗
- 提高结果稳定性

用例：

- 格式化代码
  - 以 rust 语言为例，可使用 rustfmt。
  - 若环境无法找到任何格式化程序，则跳过格式化动作。因为用户必然能够找到可用的格式化程序，可能由编辑器或 CI 执行。
- 静态检查
  - 以 rust 语言为例，可使用 clippy 或 cargo check。但假如对错误已有预期，则在执行 check 前就构造特定错误的压缩命令。
  - 静态检查的结果在许多场合会影响后续步骤，若环境无法找到任何工具，则视情况自行判断。
- schema 校验
- 代码生成（proto 编译等）
```

### 将重复流程沉淀为可复用资产

反复出现、可动态加载的流程应优先沉淀为脚本、Skill、Prompt layer、Template 或配置文件。

```prompt
- 将重复流程沉淀为可复用资产

当用户发起的流程显然具备可重复性、上下文可动态加载或可模板化，则应提示用户对流程进行沉淀，如脚本化、生成 Skill、生成 Prompt layer、生成模板或配置文件。
```


