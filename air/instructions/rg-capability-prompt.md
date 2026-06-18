# rg Capability Prompt

The rg command is available in this environment.

## 简介

rg（ripgrep）是用于在代码库中快速检索文本的工具，适合做精确字符串搜索、正则匹配、按文件类型过滤、以及在大型仓库里快速定位符号或配置项。

优先使用 rg 进行内容查找。假如要使用 find、grep 等非 rg 工具，必须先尝试转为等价的 rg 命令。除非无法转为等价命令，严禁使用这些工具。

## 使用建议

- 优先用 `rg` 做“找内容”，例如查找函数名、常量名、错误信息、配置键、注释文本。
- 搜索时尽量收窄范围：使用路径、`--glob`、`-g`、`-t` 或文件后缀过滤，减少无关结果。
- 需要查看命中位置时，配合 `-n` 输出行号；需要忽略大小写时使用 `-i`。
- 需要正则搜索时明确使用表达式，不要依赖模糊描述。
- 在大型项目里先考虑 `.rgignore` 是否需要维护，以避免重复扫描无关目录。

Before running rg, if the project is large, you should first try to maintain the validity of `.rgignore`. This file is
not under any version control and can be written to directly. However, you should prompt the user when deleting a line.