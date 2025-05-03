# Context7 MCP Server for Zed

This extension integrates [Context7](https://context7.com/) as a Model Context Protocol (MCP) server for Zed's Assistant, providing up-to-date documentation for any prompt.

## What is Context7?

Context7 pulls up-to-date, version-specific documentation and code examples straight from the source and places them directly into your prompt context.

### ❌ Without Context7

LLMs rely on outdated or generic information about the libraries you use. You get:
- ❌ Code examples are outdated and based on year-old training data
- ❌ Hallucinated APIs that don't even exist
- ❌ Generic answers for old package versions

### ✅ With Context7

Context7 fetches up-to-date, version-specific documentation and code examples straight from the source — and places them directly into your prompt.

Add `use context7` to your question in Zed Assistant:

```
How do I use the new Next.js `after` function? use context7
```

```
How do I invalidate a query in React Query? use context7
```

```
How do I protect a route with NextAuth? use context7
```

## How It Works

- 1️⃣ Ask your question naturally
- 2️⃣ Tell the LLM to `use context7`
- 3️⃣ Get working code answers

No tab-switching, no hallucinated APIs that don't exist, no outdated code generations.

## Installation

This extension can be installed from the Zed extension.

## Agent Mode Configuration

If you're using Zed's agent mode, you need to enable this context server for your assistant:

1. Open Zed's assistant settings
2. Enable the Context7 MCP server. If you see that the status of the tool is a red dot, make sure you toggle it so that becomes green.
3. Enable the Context7 MCP Server in the active assistant profile. In the chat section, click on the `Write | Ask` button, then click on `tools`, then enable the Context7 MCP Server.

## Environment Variables (Optional)

- `DEFAULT_MINIMUM_TOKENS`: Set the minimum token count for documentation retrieval (default: 10000).

Examples:

```json
{
  "context_server": {
    "mcp-server-context7": {
      "settings": {
        "default_minimum_tokens": "10000"
      }
    }
  }
}
```

## Available Tools

The Context7 MCP Server provides these tools to the LLM:

- `resolve-library-id`: Resolves a general library name into a Context7-compatible library ID.
  - `libraryName` (optional): Search and rerank results

- `get-library-docs`: Fetches documentation for a library using a Context7-compatible library ID.
  - `context7CompatibleLibraryID` (required)
  - `topic` (optional): Focus the docs on a specific topic (e.g., "routing", "hooks")
  - `tokens` (optional, default 5000): Max number of tokens to return

## Development

Clone the project and install dependencies:

```
cargo build
```

## License

MIT
