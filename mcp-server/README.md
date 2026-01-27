# MCP Server Example

A Model Context Protocol (MCP) server implementation for AI agent integration with RustAPI.

> 📖 **Cookbook**: [Crates → rustapi-toon](https://tuntii.github.io/RustAPI/) · [MCP Specification](https://modelcontextprotocol.io/)

## Prerequisites

- Rust 1.70+
- Understanding of [toon-api](../toon-api/) example
- Interest in AI/LLM development

## Overview

## Features

- ✅ **Tool Listing** - Expose available tools to AI assistants
- ✅ **Tool Execution** - Execute tools with type-safe arguments
- ✅ **Resource Listing** - Provide access to documents and resources
- ✅ **TOON Format** - 50-58% token savings for LLM communication
- ✅ **Token Counting** - Automatic token usage headers
- ✅ **Content Negotiation** - Automatic JSON/TOON format selection

## Running

```bash
cargo run --example mcp-server
```

The server will start on `http://localhost:8080`

## Endpoints

### MCP Server Info
```bash
curl http://localhost:8080/mcp
```

### List Tools (JSON)
```bash
curl http://localhost:8080/mcp/tools
```

### List Tools (TOON - for LLMs)
```bash
curl -H "Accept: application/toon" http://localhost:8080/mcp/tools
```

### Execute Tool
```bash
curl -X POST http://localhost:8080/mcp/tools/execute \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "calculate",
    "arguments": {
      "operation": "add",
      "a": 5,
      "b": 3
    }
  }'
```

### List Resources
```bash
curl http://localhost:8080/mcp/resources
```

## Available Tools

### calculate
Performs basic arithmetic operations (add, subtract, multiply, divide).

**Arguments:**
- `operation` (string, required): One of "add", "subtract", "multiply", "divide"
- `a` (number, required): First operand
- `b` (number, required): Second operand

**Example:**
```bash
curl -X POST http://localhost:8080/mcp/tools/execute \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "calculate",
    "arguments": {
      "operation": "multiply",
      "a": 7,
      "b": 6
    }
  }'
```

### get_weather
Gets current weather for a location (mock implementation).

**Arguments:**
- `location` (string, required): City name or coordinates
- `units` (string, optional): "celsius" or "fahrenheit" (default: celsius)

**Example:**
```bash
curl -X POST http://localhost:8080/mcp/tools/execute \
  -H "Content-Type: application/json" \
  -d '{
    "tool": "get_weather",
    "arguments": {
      "location": "Istanbul",
      "units": "celsius"
    }
  }'
```

## Token Savings Example

**JSON Response (148 bytes, ~40 tokens):**
```json
{
  "tools": [
    {
      "name": "calculate",
      "description": "Perform basic arithmetic operations"
    }
  ]
}
```

**TOON Response (87 bytes, ~25 tokens) - 37% savings:**
```
tools[1]{name,description}:
  calculate,Perform basic arithmetic operations
```

## Headers

All MCP responses include token counting headers:

- `X-Token-Count-JSON` - Token count if JSON format
- `X-Token-Count-TOON` - Token count if TOON format
- `X-Token-Savings` - Percentage saved with TOON
- `X-Format-Used` - Format actually used ("json" or "toon")

## Integration with AI Assistants

This MCP server can be used with AI assistants like Claude, GPT-4, or any MCP-compatible client. Simply point the client to `http://localhost:8080/mcp` and it will discover available tools and resources.

## Extending

To add more tools:

1. Add tool definition to `list_tools()` function
2. Add execution logic to `execute_tool()` function
3. Update the match statement with your tool name

Example:
```rust
Tool {
    name: "my_new_tool".to_string(),
    description: "Does something useful".to_string(),
    input_schema: ToolSchema {
        // Define schema here
    },
}
```
