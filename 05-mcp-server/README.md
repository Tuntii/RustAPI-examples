# 05-mcp-server — MCP Tools Example

This is a complete, standalone example of exposing your RustAPI endpoints as **MCP tools** for LLMs and AI agents (Claude, Cursor, Continue, custom agents, etc.).

## Features Demonstrated

- `RustApi::auto()` for zero-boilerplate routing + OpenAPI
- Tagging routes with `#[rustapi_rs::tag("agent")]` to selectively expose them
- `McpServer::from_rustapi(...)` with `InvocationMode::InProcess` (zero network overhead, full pipeline respect)
- Side-by-side HTTP API (port 8080) + MCP server (port 9090)
- Graceful shutdown via `run_rustapi_and_mcp_with_shutdown`
- Security: untagged routes (e.g. `/admin/secret`) are **never** visible to agents

## Run

```bash
# From the rustapi-rs-examples root
cargo run -p mcp-server
```

Output:
```
🚀 HTTP API:  http://127.0.0.1:8080
🧠 MCP tools: http://127.0.0.1:9090
```

## Usage

### Normal HTTP (for humans / browsers)
- `GET http://127.0.0.1:8080/weather/Istanbul`
- `POST http://127.0.0.1:8080/calc/sum` with `{"a": 40, "b": 2}`

### MCP / Agent endpoint (port 9090)
Point any MCP client at `http://127.0.0.1:9090` (HTTP + SSE transport).

Manual test with curl (JSON-RPC):

```bash
# initialize
curl -X POST http://127.0.0.1:9090 \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"initialize"}'

# list tools (only the ones with "agent" tag)
curl -X POST http://127.0.0.1:9090 \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'

# call a tool
curl -X POST http://127.0.0.1:9090 \
  -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"get_weather","arguments":{"city":"Istanbul"}}}'
```

## How It Works

Routes annotated with `#[rustapi_rs::tag("agent")]` are automatically turned into MCP tools using the OpenAPI schema RustAPI already generates.

`InvocationMode::InProcess` means:
- Tool calls bypass the network entirely
- Still go through validation, extractors, middleware, error handlers, etc.
- Extremely fast (~microseconds)

See source in `src/main.rs`.

## See Also

- Main RustAPI repo examples: `crates/rustapi-rs/examples/mcp_tools.rs` (quick in-tree demo)
- Cookbook: [MCP Integration](https://tuntii.github.io/RustAPI/recipes/mcp_integration.html)
- Other MCP recipes in the cookbook (in-process, OpenAPI CLI, stdio)
- The full [rustapi-rs-examples](https://github.com/Tuntii/rustapi-rs-examples) repository

## Dependencies (in this example)

```toml
rustapi-rs = { version = "0.1.507", features = ["protocol-mcp", "swagger-ui"] }
```

## License

Same as the main RustAPI project (MIT OR Apache-2.0).
