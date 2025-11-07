#!/bin/bash
# Simple test script for the MCP server

set -e

echo "Building vim_mcp_server..."
cargo build --quiet

echo ""
echo "=== Testing MCP Server ==="
echo ""

# Test 1: Initialize
echo "1. Initialize"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | cargo run --quiet 2>/dev/null
echo ""

# Test 2: List tools
echo "2. List tools"
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | cargo run --quiet 2>/dev/null | jq '.result.tools[] | {name, description}'
echo ""

# Test 3: Call hello tool
echo "3. Call hello tool"
echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"hello","arguments":{"name":"vSphere Developer"}}}' | cargo run --quiet 2>/dev/null | jq '.result.content[0].text'
echo ""

# Test 4: Call stats tool
echo "4. Call stats tool"
echo '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"stats","arguments":{}}}' | cargo run --quiet 2>/dev/null | jq '.result.content[0].text'
echo ""

# Test 5: List resources
echo "5. List resources"
echo '{"jsonrpc":"2.0","id":5,"method":"resources/list"}' | cargo run --quiet 2>/dev/null | jq '.result.resources[] | {uri, name}'
echo ""

# Test 6: Read metadata resource
echo "6. Read metadata resource"
echo '{"jsonrpc":"2.0","id":6,"method":"resources/read","params":{"uri":"vim://metadata"}}' | cargo run --quiet 2>/dev/null | jq '.result.contents[0].text | fromjson'
echo ""

echo "=== All tests passed! ==="
