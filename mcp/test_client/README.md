# MCP Test Client

A generic web-based test client for validating MCP (Model Context Protocol) servers. This tool dynamically discovers available tools from any MCP server and provides a clean UI for testing them.

## Features

- **Generic & Dynamic**: Works with any MCP server without modification
- **Auto-discovery**: Automatically discovers and displays all available tools
- **Dynamic Forms**: Generates input forms based on JSON Schema definitions
- **Markdown Support**: Converts markdown output to formatted HTML
- **Clean UI**: Modern, responsive design with scrollable results
- **Real-time Testing**: Invoke tools and see results immediately

## Quick Start

### Prerequisites

- Rust 1.70 or later
- An MCP server executable (e.g., `vim_mcp_server.exe`)

### Configuration

1. Create or edit `mcp.json` in the `test_client` directory:

```json
{
  "mcpServers": {
    "vim": {
      "command": "C:\\Users\\karaa\\projects\\rust\\vim-rs\\mcp\\target\\release\\vim_mcp_server.exe"
    }
  }
}
```

Update the `command` path to point to your MCP server executable.

### Running the Client

1. Build and run the test client:

```bash
cd test_client
cargo run --release
```

2. Open your browser and navigate to:

```
http://127.0.0.1:8080
```

3. The client will:
   - Spawn the MCP server as a subprocess
   - Connect via stdio
   - Discover all available tools
   - Display them in a dropdown

### Using the UI

1. **Select a Tool**: Choose a tool from the dropdown menu
2. **Fill Parameters**: A form will appear with inputs based on the tool's schema
   - Required fields are marked with `*`
   - Descriptions provide context for each parameter
   - Arrays and objects can be entered as JSON
3. **Invoke**: Click "Invoke Tool" to execute
4. **View Results**: Results appear below with:
   - Timestamp
   - Success/Error status
   - Input parameters (formatted JSON)
   - Output (rendered markdown)
5. **Clear Results**: Use "Clear Results" to reset the display

## Architecture

### Components

- **MCP Client Manager** (`src/mcp_client.rs`): Spawns and manages the MCP server subprocess
- **Form Generator** (`src/form_generator.rs`): Dynamically generates HTML forms from JSON Schema
- **Markdown Renderer** (`src/markdown.rs`): Converts markdown to HTML
- **Web Server** (`src/main.rs`): Actix-web server with routes and state management
- **Config Parser** (`src/config.rs`): Reads and parses `mcp.json`

### Routes

- `GET /` - Main page with tool selector
- `GET /api/tool/{tool_name}` - Get form HTML for a specific tool
- `POST /api/invoke` - Invoke a tool with parameters

## Supported Parameter Types

The form generator handles all JSON Schema types:

- **string**: Text input or dropdown (for enums)
- **number/integer**: Number input
- **boolean**: Checkbox
- **array**: JSON textarea
- **object**: Nested fieldset with sub-fields

## Development

### Project Structure

```
test_client/
├── Cargo.toml           # Dependencies
├── mcp.json             # MCP server configuration
├── src/
│   ├── main.rs          # Web server and routes
│   ├── mcp_client.rs    # MCP client management
│   ├── form_generator.rs # Dynamic form generation
│   ├── markdown.rs      # Markdown conversion
│   └── config.rs        # Configuration parser
├── templates/
│   ├── index.html       # Main page template
│   └── tool_form.html   # Form fragment template
└── static/
    └── style.css        # Styling
```

### Building

```bash
# Debug build
cargo build

# Release build (recommended)
cargo build --release

# Run tests
cargo test
```

## Troubleshooting

### "Failed to spawn MCP server"

- Verify the path in `mcp.json` is correct
- Ensure the MCP server executable exists and is executable
- Check that the server runs standalone

### "No tools available"

- Check that the MCP server implements the `tools/list` endpoint
- Look at the server's stderr output for errors
- Verify the server is MCP protocol compliant

### "Failed to list tools"

- The MCP server may have crashed or exited
- Check server logs (stderr is passed through)
- Verify JSON-RPC communication is working

## License

This test client is part of the vim-rs project.

