<div align="center">

# Antigravity Skill Injector

**Zero-context-bloat dynamic skill loading for the Antigravity Desktop App**

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-CE422B?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/Python-3.9%2B-3776AB?logo=python&logoColor=white)](https://www.python.org)
[![MCP](https://img.shields.io/badge/Protocol-MCP-FF6B35)](https://modelcontextprotocol.io)
[![Antigravity](https://img.shields.io/badge/App-Antigravity-8B5CF6)](https://antigravity.so)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

---

## The Problem

Antigravity loads every skill into the system prompt at conversation start. With dozens of skills, that's **thousands of wasted tokens** before you write a single word — context bloat, slower responses, and higher cost on every turn.

## The Solution

Replace heavy `SKILL.md` files with **20-token stubs**. Skills still appear in the Antigravity UI with real names and descriptions. When the agent actually needs a skill, it calls the local Rust MCP server to fetch the full payload **on demand**.

---

## How It Works

```mermaid
flowchart TB
    subgraph Before["❌ Before — Context Bloat"]
        SP1[System Prompt] -->|"Skill A: 500 tokens\nSkill B: 800 tokens\nSkill C: 600 tokens\n... 30 more skills"| CTX1[😩 Context Window Saturated]
    end

    subgraph After["✅ After — Zero Bloat"]
        SP2[System Prompt] -->|"Skill A: 20 tokens\nSkill B: 20 tokens\nSkill C: 20 tokens"| CTX2[Context Window Free]
        CTX2 -->|Agent needs Skill B| MCP[Rust MCP Server]
        MCP -->|fetch_skill payload| REG[skills_registry.json]
        REG -->|"Full 800-token payload\n(only when needed)"| CTX2
    end
```

---

## Architecture

```mermaid
graph LR
    A[Antigravity UI] -->|Renders stub files| B[~/.gemini/plugins/]
    B -->|20-token stub per skill| C[Agent Context]
    C -->|mcp.fetch_skill call| D[Rust MCP Server\nlocal binary]
    D -->|reads| E[skills_registry.json]
    E -->|full SKILL.md payload| D
    D -->|dynamic injection| C

    F[migrate_skills.py] -->|parses SKILL.md files| E
    G[generate_stubs.py] -->|writes stub files| B
```

---

## Setup

### 1. Clone and install

```bash
git clone https://github.com/pappdavid/antigravity-skill-injector.git
cd antigravity-skill-injector
./install.sh
```

The installer will:
- Compile the Rust MCP server (requires Rust 1.75+)
- Ask for your existing Antigravity skills directory
- Migrate all `SKILL.md` files to `skills_registry.json`
- Generate 20-token stub files in your Antigravity plugins directory

### 2. Configure Antigravity

The installer prints an MCP config block. Add it to `~/.gemini/config/mcp_config.json`:

```json
{
  "mcpServers": {
    "skill-injector": {
      "command": "/path/to/antigravity-skill-injector/src/mcp-server/target/release/skill-injector-mcp",
      "args": ["--registry", "/path/to/skills_registry.json"]
    }
  }
}
```

### 3. Restart Antigravity

All your skills appear in the UI with zero token overhead. The agent fetches full payloads on demand.

---

## Components

| Component | Language | Description |
|---|---|---|
| `src/mcp-server/` | **Rust** | Ultra-fast local MCP server — JSON-RPC over stdio |
| `scripts/migrate_skills.py` | **Python** | Parses `SKILL.md` YAML frontmatter → `skills_registry.json` |
| `scripts/generate_stubs.py` | **Python** | Generates 20-token stub files for the Antigravity UI |
| `install.sh` | **Bash** | End-to-end installer: compile → migrate → inject → configure |

---

## Token Savings

```
Typical setup: 30 skills × 600 tokens average = 18,000 tokens/conversation
After injection: 30 skills × 20 tokens = 600 tokens/conversation

Savings: ~17,400 tokens per conversation (~97% reduction)
```

---

## Tech Stack

<div align="center">

![Rust](https://skillicons.dev/icons?i=rust)&nbsp;
![Python](https://skillicons.dev/icons?i=python)&nbsp;
![Bash](https://skillicons.dev/icons?i=bash)

</div>
