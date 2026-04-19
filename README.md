# Rust CLI Coding Agent

A CLI coding agent in Rust that connects to Ollama (with extension to other providers), featuring a TUI with markdown rendering for user interaction.

## Quick Start

```bash
# Run with defaults (Ollama localhost:11434)
cargo run

# Run with custom model
AGENT_MODEL=codellama:7b cargo run

# Run with custom Ollama URL
OLLAMA_BASE_URL=http://192.168.1.100:11434 cargo run
```

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
pulldown-cmark = "0.10"
```

## Tools

| Tool | Parameters | Description |
|------|------------|-------------|
| `read_file` | `path: string` | Read file contents |
| `write_file` | `path: string, content: string` | Write/create file |
| `edit_file` | `path: string, old: string, new: string` | Replace string in file |
| `bash` | `command: string` | Execute shell command |

### Tool Security (Bash)

- **Allowed**: Most shell commands
- **Blocked**: `rm -rf /`, `>: /dev/sdX`, `mkfs.*`, `dd if=/dev/zero of=/dev/sdX`

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

| Variable | Purpose | Default |
|----------|---------|---------|
| `OLLAMA_BASE_URL` | Ollama server URL | `http://localhost:11434` |
| `AGENT_MODEL` | Model to use | `qwen2.5:coder` |
| `AGENT_MAX_ITERATIONS` | Max tool calls per prompt | `10` |

## TUI Layout

```
┌─────────────────────────────────────────────────┐
│  ┌───────────────────────────────────────────┐  │
│  │Output Area (scrollable)                  │  │
│  │                                           │  │
│  │  ── Tool Trace ──                         │  │
│  │  > read_file(path="src/main.rs")         │  │
│  │  < File contents...                      │  │
│  │                                           │  │
│  │  ── LLM Response ──                      │  │
│  │  Here's the analysis...                  │  │
│  └───────────────────────────────────────────┘  │
│  ─────────────────────────────────────────────  │
│  Status: Ollama | qwen2.5:coder | Tools: 4     │
│  ─────────────────────────────────────────────  │
│  > Fix the bug in src/main.rs                  │
│  (Enter to send, Ctrl+C to quit)               │
└─────────────────────────────────────────────────┘
```

## Usage

### Key Bindings

| Key | Action |
|-----|--------|
| Enter | Send prompt to agent |
| Ctrl+C | Quit |

## Provider Extension

The `LLMProvider` trait enables adding more providers:

```rust
#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, messages: Vec<Message>, tools: Option<ToolSchema>) -> Result<ChatResponse>;
    fn name(&self) -> &'static str;
}
```

## License

MIT