# Rust CLI Coding Agent - Specification

## Overview

Build a CLI coding agent in Rust that connects to Ollama (with extension to other providers), featuring a TUI with markdown rendering for user interaction.

## Architecture

```
User Input (TUI) → Agent Loop → LLM (Ollama) → Tool Execution → Render Output (TUI)
```

### Components

| Component | Responsibility |
|-----------|---------------|
| `main.rs` | Entry point, TUI setup, event loop |
| `config.rs` | Config loading from `.agent.toml` and env vars |
| `agent.rs` | Core agent loop (LLM → tools → repeat) |
| `provider/mod.rs` | `LLMProvider` trait for abstraction |
| `provider/ollama.rs` | Ollama implementation |
| `tools/mod.rs` | Tool registry and execution |
| `tools/file.rs` | read/write/edit files |
| `tools/bash.rs` | Shell execution |
| `ui/mod.rs` | TUI layout with ratatui |
| `ui/widgets.rs` | Output/input/custom widgets |

## Dependencies

```toml
[dependencies]
ratatui = "0.28"
crossterm = "0.28"
ollama-client = "0.2"
async-trait = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
pulldown-cmark = "0.10"  # Markdown rendering
```

## Tools Schema

The agent has access to these tools (JSON schema sent to LLM):

| Tool | Parameters | Description |
|------|------------|-------------|
| `read_file` | `path: string` | Read file contents |
| `write_file` | `path: string, content: string` | Write/create file |
| `edit_file` | `path: string, old: string, new: string` | Replace string in file |
| `bash` | `command: string` | Execute shell command |

### Tool Security (Bash)

- **Allowed**: Most shell commands
- **Blocked**: `rm -rf /`, `>: /dev/sdX`, `mkfs.*`, `dd if=/dev/zero of=/dev/sdX`

## Agent Loop

1. User types prompt in TUI input
2. Send system prompt + user message + tool schema to LLM
3. LLM responds with:
   - **Text response** → Display in output (markdown rendered)
   - **Tool call** → Execute tool, append result, repeat from step 2
4. Display LLM response + full tool trace in output area
5. Max iterations: 10 (configurable)

### System Prompt

```
You are a CLI coding agent. You have tools to read, write, edit files and run shell commands.
Available tools: read_file, write_file, edit_file, bash.
When you need to act on the filesystem or run commands, use the available tools.
After each tool execution, assess if the task is complete. If not, continue using tools.
Respond with text when the task is done, or when no tool is needed.
```

## TUI Layout

```
┌─────────────────────────────────────────────────┐
│                                                 │
│  ┌───────────────────────────────────────────┐  │
│  │Output Area (scrollable)                  │  │
│  │                                           │  │
│  │  ── Tool Trace ──                         │  │
│  │  > read_file(path="src/main.rs")          │  │
│  │  < File contents...                       │  │
│  │                                           │  │
│  │  ── LLM Response ──                       │  │
│  │  Here's the analysis...                   │  │
│  │  ```rust                                 │  │
│  │  fn main() { ... }                       │  │
│  │  ```                                     │  │
│  │                                           │  │
│  └───────────────────────────────────────────┘  │
│  ─────────────────────────────────────────────  │
│  Status: Ollama | qwen2.5:coder | Tools: 4     │
│  ─────────────────────────────────────────────  │
│  > Fix the bug in src/main.rs                  │
│  (Enter to send, Ctrl+C to quit)               │
└─────────────────────────────────────────────────┘
```

### Interaction

| Key | Action |
|-----|--------|
| Enter | Send prompt to agent |
| Ctrl+C | Quit |
| Up/Down | Navigate input history (V1.1) |

## Configuration

### `.agent.toml`

```toml
[agent]
provider = "ollama"
model = "qwen2.5:coder"
max_iterations = 10
system_prompt = "You are..."

[agent.tool_restrictions]
allow_bash = true
blocked_patterns = ["rm -rf /", ">: /dev/sdX"]
```

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `OLLAMA_BASE_URL` | Ollama server URL (default: `http://localhost:11434`) |
| `AGENT_MODEL` | Model to use |
| `AGENT_MAX_ITERATIONS` | Max tool calls per prompt |

## Provider Extension

The `LLMProvider` trait enables adding more providers:

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, messages: Vec<Message>, tools: Option<ToolSchema>) -> Result<ChatResponse>;
    fn name(&self) -> &'static str;
}
```

### Future Providers

| Provider | Crate | Priority |
|----------|-------|----------|
| OpenAI | `openai-oxide` or raw API | High |
| Anthropic | `anthropic-sdk-rust` | High |
| OpenRouter | Multi-provider crate | Medium |

## Markdown Rendering

Use `pulldown-cmark` to parse markdown and render in TUI:
- Headers → Bold + underline
- Code blocks → Fixed-width, background
- Bold → Bold text
- Lists → Bullets
- Links → Show URL (no clickable in V1)

## Implementation Phases

### Phase 1: Minimal Core
- [ ] `Cargo.toml` setup
- [ ] Config loading
- [ ] `LLMProvider` trait + Ollama impl
- [ ] Tool registry with file/bash tools
- [ ] Basic agent loop (no TUI yet)
- [ ] Test with CLI input/output

### Phase 2: TUI Integration
- [ ] Ratatui setup
- [ ] Output area widget (scrollable)
- [ ] Input widget
- [ ] Status bar
- [ ] Tool trace display

### Phase 3: Polish
- [ ] Markdown rendering in output
- [ ] Input history
- [ ] Config file support
- [ ] Proper error handling
- [ ] Help / commands

## Default Values

| Setting | Default |
|---------|---------|
| Provider | Ollama |
| Model | `qwen2.5:coder` |
| Ollama URL | `http://localhost:11434` |
| Max Iterations | 10 |
| Bash Blocklist | `rm -rf /`, `>: /dev/sdX`, `mkfs`, `dd if=/dev/zero of=/dev/` |

## Usage

```bash
# Run with defaults (Ollama localhost)
cargo run

# Run with custom model
AGENT_MODEL=codellama:7b cargo run

# Run with custom Ollama URL
OLLAMA_BASE_URL=http://192.168.1.100:11434 cargo run
```
