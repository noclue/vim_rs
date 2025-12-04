# Quick Start Guide

## Setup

1. **Configure the MCP server** in `mcp.json`:
   ```json
   {
     "mcpServers": {
       "vim": {
         "command": "C:\\Users\\karaa\\projects\\rust\\vim-rs\\mcp\\target\\release\\vim_mcp_server.exe"
       }
     }
   }
   ```
   
   Update the path to point to your MCP server executable.

2. **Build the vim_mcp_server** (if not already built):
   ```bash
   cd ../server
   cargo build --release
   ```

## Running the Test Client

```bash
cd test_client
cargo run --release
```

The server will start at `http://127.0.0.1:8080`

## Using the Web UI

1. Open your browser to `http://127.0.0.1:8080`
2. Select a tool from the dropdown
3. Fill in the parameters (required fields marked with *)
4. Click "Invoke Tool"
5. View the results below

## Features

- **Auto-discovery**: All tools are discovered dynamically from the MCP server
- **Dynamic Forms**: Input forms are generated automatically from JSON Schema
- **Markdown Rendering**: Tool output is converted to formatted HTML
- **Result History**: All invocations are shown with timestamps
- **Error Handling**: Errors are clearly displayed

## Troubleshooting

### "Failed to spawn MCP server"
- Check that the path in `mcp.json` is correct
- Verify the executable exists
- Ensure you have permissions to execute it

### "No tools available"
- The MCP server may have failed to start
- Check the console for error messages
- Try running the MCP server standalone to verify it works

### Tools not appearing
- The server is still initializing
- Refresh the page
- Check browser console for JavaScript errors

