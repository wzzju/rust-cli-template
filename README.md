# Rust CLI Template / Rust CLI 模板

[English](#english) | [中文](#chinese)

---

<a id="english"></a>
## English

### Introduction
This project is a Rust CLI application template generated using AI assistance and specialized development skills. It demonstrates a modern workflow combining large language models with structured engineering practices to build high-quality software efficiently.

### AI & Skills Usage
This project was built using a specific set of AI "skills" to ensure quality, maintainability, and adherence to best practices throughout the development lifecycle:

- **using-superpowers**: Used at the very beginning to establish the capabilities and protocols for the session, ensuring efficient tool usage.
- **brainstorming**: Applied to explore user intent, analyze requirements, and design the initial feature set before any code was written.
- **rust10x**: Provided expert guidance on Rust-specific best practices, project structure, error handling patterns, and idiomatic code styles.
- **writing-plans**: Utilized to create a comprehensive, step-by-step implementation plan (saved in `docs/plans`), ensuring a clear roadmap for development.
- **executing-plans**: Managed the systematic execution of the implementation plan, breaking down complex tasks into manageable batches.
- **using-git-worktrees**: Employed to set up an isolated development environment in `.worktrees/`, allowing for safe parallel development without affecting the main repository.
- **finishing-a-development-branch**: Utilized to verify the final project state, ensuring all tests pass, the codebase is clean, and the branch is ready for merging.
- **suggest-git-commit**: Used to generate semantic and standardized Git commit messages, ensuring a clean and readable project history.

### Project Structure
- `crates/core`: Contains the core search logic, domain models, and rendering logic.
- `crates/cli`: Handles command-line argument parsing and execution flow.
- `docs/plans`: Contains the implementation plan used to guide the development.

### Usage

#### Installation

Ensure you have Rust installed. Clone the repository and run:

```bash
# Build the project using the Cargo.lock file to ensure reproducible builds
cargo build --release --locked

# Install the binary globally (overwriting if exists) using the lockfile
cargo install --path crates/cli --force --locked
```

To manage installed binaries:

```bash
# List all installed cargo binaries
cargo install --list

# Uninstall the binary (use the package name 'rg-cli')
cargo uninstall rg-cli
```

The binary will be available in `target/release/rg` (if built) or in your cargo bin path (if installed).

#### Commands

This CLI tool supports the following commands:

**1. Search**

Search for a pattern in a file or standard input.

```bash
# Search in a file
rg search "pattern" file.txt

# Search with regex
rg search -r "^\d+" file.txt

# Case-insensitive search
rg search -i "pattern" file.txt

# Search from stdin
echo "content" | rg search "pattern"
```

**Options:**
- `-i, --ignore-case`: Enable case-insensitive matching.
- `-r, --regex`: Treat the pattern as a regular expression.

#### Documentation

To generate and view the project documentation locally:

```bash
cargo doc --open --no-deps --workspace
```

---

<a id="chinese"></a>
## Chinese (中文)

### 简介
本项目是一个使用 AI 辅助和特定开发技能生成的 Rust CLI 应用程序模板。它展示了一种结合大语言模型与结构化工程实践的现代工作流，旨在高效构建高质量软件。

### AI 与技能使用
本项目在开发周期的各个阶段使用了以下 AI "技能" (Skills)，以确保代码质量、可维护性和最佳实践的落地：

- **using-superpowers**: 在会话开始时使用，用于确立能力范围和工具使用协议，确保高效的协作。
- **brainstorming**: 用于探索用户意图、分析需求，并在编写任何代码之前设计初始功能集。
- **rust10x**: 提供 Rust 特定的最佳实践指导，包括项目结构、错误处理模式和地道的代码风格。
- **writing-plans**: 用于制定全面、分步的实施计划（保存在 `docs/plans` 中），确保开发过程有清晰的路线图。
- **executing-plans**: 管理实施计划的系统化执行，将复杂的任务分解为可管理的批次进行处理。
- **using-git-worktrees**: 用于在 `.worktrees/` 中设置隔离的开发环境，允许在不影响主仓库的情况下进行安全的并行开发。
- **finishing-a-development-branch**: 用于验证最终项目状态，确保所有测试通过、代码库整洁，并准备好合并分支。
- **suggest-git-commit**: 用于生成语义化且标准化的 Git 提交信息，确保项目历史清晰可读。

### 项目结构
- `crates/core`: 包含核心搜索逻辑、领域模型和渲染逻辑。
- `crates/cli`: 处理命令行参数解析和执行流程。
- `docs/plans`: 包含用于指导开发的实施计划文档。

### 使用说明

#### 安装

确保已安装 Rust 环境。克隆仓库并运行：

```bash
# 使用 Cargo.lock 文件构建项目，确保构建的可重现性
cargo build --release --locked

# 全局安装二进制文件（如果存在则覆盖），使用锁定文件
cargo install --path crates/cli --force --locked
```

管理已安装的二进制文件：

```bash
# 列出所有已安装的 cargo 二进制文件
cargo install --list

# 卸载二进制文件（使用包名 'rg-cli'）
cargo uninstall rg-cli
```

二进制文件将生成在 `target/release/rg`（如果已构建）或您的 cargo bin 路径中（如果已安装）。

#### 命令

本 CLI 工具支持以下命令：

**1. Search (搜索)**

在文件或标准输入中搜索模式。

```bash
# 在文件中搜索
rg search "pattern" file.txt

# 使用正则表达式搜索
rg search -r "^\d+" file.txt

# 忽略大小写搜索
rg search -i "pattern" file.txt

# 从标准输入搜索
echo "content" | rg search "pattern"
```

**选项:**
- `-i, --ignore-case`: 启用不区分大小写的匹配。
- `-r, --regex`: 将模式视为正则表达式。

#### 文档

要在本地生成并查看项目文档：

```bash
cargo doc --open --no-deps --workspace
```
