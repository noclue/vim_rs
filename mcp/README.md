# vim_rs MCP Server

A Model Context Protocol (MCP) server that gives AI coding assistants comprehensive access to the **vSphere API for Rust** — enabling semantic search, property exploration, and intelligent code generation for [vim_rs](https://github.com/noclue/vim_rs).

The embedded API database is built from the **vSphere 9.1.0.0** VI/JSON OpenAPI specification (`vim_build/data/vi_json_openapi_specification_v9_1_0_0.json`), aligned with `vim_rs` **0.6.0**.

## Quick Usage

Once configured, your AI assistant can:

```
You: "How do I power on a virtual machine with vim_rs?"

AI: [calls search("power on virtual machine")]
    → Found VirtualMachine::power_on_vm_task method
    
    [calls get("VirtualMachine::power_on_vm_task")]
    → Returns full method signature, documentation, and example code
```

```
You: "What properties can I retrieve from a VirtualMachine?"

AI: [calls get_property_tree("VirtualMachine")]
    → Returns navigable tree of all VM properties with Rust types
```

---

## Installation

### Download Pre-built Binary

TBD

### Build from Source

```bash
git clone https://github.com/noclue/vim_rs.git
cd vim_rs/mcp

# Build API database and embeddings
cargo run -p data-transformer --release

# Build release binary (includes embedded model + API database)
cargo build --release -p vim_mcp_server

# Binary location:
# Windows: target/release/vim_mcp_server.exe
# Linux/macOS: target/release/vim_mcp_server
```

**Optional:** Enable CUDA GPU acceleration (requires CUDA 12.x + cuDNN 9.x):

```bash
# Build embeddings with GPU acceleration
cargo run -p data-transformer --release --features cuda

# Build server with GPU acceleration
cargo build --release -p vim_mcp_server --features cuda
```

---

## Configure Your AI Tool

### Claude Code (CLI)

Add the MCP server using the Claude CLI:

```bash
# Windows
claude mcp add vim-rs -- C:\path\to\vim_mcp_server.exe

# Linux/macOS
claude mcp add vim-rs -- /path/to/vim_mcp_server
```

Or edit your Claude config file directly:

**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`  
**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "vim-rs": {
      "command": "C:\\path\\to\\vim_mcp_server.exe"
    }
  }
}
```

### Cursor

Open **Settings** → **MCP** → **Add Server**, or edit `.cursor/mcp.json` in your project:

```json
{
  "mcpServers": {
    "vim-rs": {
      "command": "C:\\path\\to\\vim_mcp_server.exe"
    }
  }
}
```

For global configuration, edit `~/.cursor/mcp.json`.

### Windsurf / Codeium

Add to your MCP configuration:

```json
{
  "mcpServers": {
    "vim-rs": {
      "command": "/path/to/vim_mcp_server"
    }
  }
}
```

### VS Code + Continue

Add to your Continue configuration (`~/.continue/config.json`):

```json
{
  "experimental": {
    "modelContextProtocolServers": [
      {
        "name": "vim-rs",
        "transport": {
          "type": "stdio",
          "command": "/path/to/vim_mcp_server"
        }
      }
    ]
  }
}
```

---

## Available Tools

| Tool | Description |
|------|-------------|
| `get_starter_guide` | **Start here!** Complete vim_rs patterns, connection setup, PropertyCollector usage |
| `search` | Semantic search across all API items using natural language |
| `get` | Get detailed info for any item by ID |
| `list_property_collector_root_types` | List all managed object types for PropertyCollector |
| `get_property_path` | Explore property paths and their Rust types |
| `get_property_tree` | Visual tree view of all properties for a managed object |

### ID Format Examples

```
Managed Object:  VirtualMachine
Method:          VirtualMachine::power_on_vm_task
Structure:       VirtualDevice
Field:           VirtualHardware::device
Enum:            ManagedEntityStatus
Trait:           VirtualDeviceTrait
Example:         example::connection_basic
```

---

## Web UI (Interactive Mode)

The server includes a built-in web interface for testing and exploring tools without an MCP client.

```bash
# Run server in web mode (default port 8080)
./target/release/vim_mcp_server --web

# Custom port and bind address
./target/release/vim_mcp_server --web --port 3000 --bind 0.0.0.0
```

Open `http://localhost:8080` to access the interactive tool explorer.

**Features:**
- Auto-discovery of all tools
- Dynamic form generation
- Markdown rendering of results
- Dark/Light theme support
- **In-process execution** (no JSON-RPC overhead)

---

### Setup

1. Configure the MCP server path in `test_client/mcp.json`:

```json
{
  "mcpServers": {
    "vim": {
      "command": "C:\\path\\to\\vim_mcp_server.exe"
    }
  }
}
```

2. Build and run:

```bash
cd test_client
cargo run --release
```

3. Open http://127.0.0.1:8080 in your browser

### Features

- **Auto-discovery** — Dynamically discovers all available tools
- **Dynamic Forms** — Generates input forms from JSON Schema
- **Markdown Rendering** — Formats tool output as HTML
- **Result History** — Shows all invocations with timestamps

---

## Architecture

```
mcp/
├── server/                 # MCP server implementation
│   └── src/
│       ├── main.rs         # Server entry point, tool handlers
│       ├── model.rs        # Embedded API DB and guide
│       ├── property_collector.rs  # Property path navigation
│       ├── field_data.rs   # Generated field path reference data. See vim_build
│       └── embedded_model.rs      # Embedded ML model
├── api_database/           # Shared API data types
├── data/                   # Generated data (gitignored)
│   ├── api_database.bin    # Compiled API database with embeddings
│   └── model_cache/        # Cached embedding model
├── data_processing/        # Build-time data generators
│   ├── data_transformer/         # All-in-one orchestrator
│   ├── build_api_definitions/    # OpenAPI → JSON
│   ├── build_embeddings/         # Generate vector embeddings
│   └── build_examples/           # Index code examples
└── test_client/            # Web UI for testing
```

### How It Works

1. **Embedded Database** — The API database (~5000 items) with pre-computed embeddings is compiled directly into the binary
2. **Semantic Search** — Uses BGE-small-en-v1.5 embeddings for natural language queries
3. **Property Navigation** — Statically analyzed type information enables property path exploration
4. **MCP Protocol** — Communicates via JSON-RPC 2.0 over stdio

---

## Development

### Prerequisites

- Rust 1.85+
- ~4GB disk space (for Rust build ML stuff)

### Building

First you need to (re)generate the API database and embeddings:

```bash
# All-in-one: API definitions + examples + embeddings
cargo run -p data-transformer --release

# With GPU acceleration
cargo run -p data-transformer --release --features cuda
```

Once you have API database (data/api_database.bin) then you can build the main executable:
```bash
# Debug build (faster compile, slower runtime)
cargo build -p vim_mcp_server

# Release build (slower compile, optimized runtime)
cargo build --release -p vim_mcp_server

# Run tests
cargo test -p vim_mcp_server
```
Again `--features cuda` will try to build accelerated version provided you have the NVidia
dependencies.


## Troubleshooting

### "Semantic search is not available"

The embedding model failed to load. Check:
- Sufficient disk space (~500MB for model cache)
- Model files aren't corrupted (delete `data/model_cache` and restart)

### "Item not found"

Use `search` to find valid IDs first, then `get` to retrieve details.

### Server not connecting

- Verify the binary path in your MCP configuration
- Check the binary has execute permissions (`chmod +x` on Linux/macOS)
- Look for error messages in the AI tool's logs

### CUDA errors (GPU builds only)

- Ensure CUDA 12.x and cuDNN 9.x are installed and their DLL files are on the PATH or library path.
- See `docs/GPU_SETUP.md` - pain to read and may help
- Verify GPU drivers are up to date
- Server falls back to CPU if CUDA fails

---

## License

Same as vim_rs project — Apache 2.0 license.
