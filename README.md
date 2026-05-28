# Antigravity Skill Injector

The **Antigravity Skill Injector** is a dynamic "Zero Context Bloat" skill injection architecture for the Antigravity Desktop App. 

## The Problem
Antigravity natively loads all your skills (plugins) into the system prompt at the start of every conversation. If you have dozens of skills, this instantly consumes thousands of tokens, bloats your context window, and slows down your agent.

## The Solution
This tool provides a **Rust-based MCP Server** combined with **native UI stubs**. 
1. **Native UI Integration**: Your skills still show up in the Antigravity UI with their real names and descriptions, but they consume virtually zero context.
2. **Dynamic Payload Loading**: When the agent decides to use a skill, it calls out to the local Rust MCP server to fetch the massive prompt payload *only* when it's actually needed.

## Setup Instructions

1. **Clone the repo**
   ```bash
   git clone https://github.com/yourusername/antigravity-skill-injector.git
   cd antigravity-skill-injector
   ```

2. **Run the Installer**
   ```bash
   ./install.sh
   ```
   The installer will:
   - Compile the ultra-fast Rust MCP server.
   - Ask for the path to your existing Antigravity skills.
   - Migrate them to a static JSON registry.
   - Inject minified "stub" files into your Antigravity plugins directory.

3. **Configure Antigravity**
   At the end of the script, it will provide a JSON block. Open your `mcp_config.json` (typically located in `~/.gemini/config/mcp_config.json`) and paste the block inside your `mcpServers` object.

4. **Restart**
   Restart your Antigravity Desktop App. You will now see all your skills natively in the UI with zero token bloat!

## Architecture Details
- **`mcp-server/`**: A low-resource compiled Rust binary that acts as the JSON-RPC interface for Antigravity.
- **`scripts/migrate_skills.py`**: Parses heavy `SKILL.md` YAML files into a structured `skills_registry.json`.
- **`scripts/generate_stubs.py`**: Generates the ~20 token stub files that populate the Antigravity UI.
