#!/usr/bin/env bash
set -e

echo "=========================================="
echo " Antigravity Skill Injector Setup"
echo "=========================================="

# 1. Compile the Rust MCP Server
echo "[1/4] Compiling the Rust MCP Server..."
cd src/mcp-server
cargo build --release
SERVER_BIN="$(pwd)/target/release/mcp-skill-injector"
cd ../..

echo "[2/4] Setting up directories..."
ANTIGRAVITY_DIR="$HOME/.gemini/antigravity"
CONFIG_DIR="$HOME/.gemini/config"
DATA_DIR="$ANTIGRAVITY_DIR/skill_injector_data"
mkdir -p "$DATA_DIR"

# Ensure Python scripts are executable
chmod +x scripts/migrate_skills.py scripts/generate_stubs.py

echo "[3/4] Migrating Skills..."
read -p "Enter the full path to your existing skills directory (or press Enter to skip migration): " SKILLS_PATH

if [ -n "$SKILLS_PATH" ]; then
    REGISTRY_JSON="$DATA_DIR/skills_registry.json"
    echo "Running migration script..."
    python3 scripts/migrate_skills.py --source "$SKILLS_PATH" --output "$REGISTRY_JSON"
    
    echo "Generating native UI stubs..."
    python3 scripts/generate_stubs.py --registry "$REGISTRY_JSON" --output-dir "$CONFIG_DIR/plugins" --plugin-name "dynamic-skills"
else
    echo "Skipping skill migration."
    REGISTRY_JSON="$DATA_DIR/skills_registry.json"
    echo "{}" > "$REGISTRY_JSON"
fi

# We must update the Rust binary to look for the registry at the dynamic location
# Or we can pass it via env var or command line args.
# Since the Rust code currently hardcodes the path, let's patch it or just inform the user.
# Wait, let's update the Rust code to use an environment variable or argument.

echo "[4/4] Configuration"
echo ""
echo "Setup is almost complete! Due to security boundaries, you must manually"
echo "add the MCP server to your config file."
echo ""
echo "Please add the following block to your MCP config (usually ~/.gemini/config/mcp_config.json):"
echo ""
echo "    \"skill-injector\": {"
echo "      \"command\": \"$SERVER_BIN\","
echo "      \"args\": [\"$REGISTRY_JSON\"],"
echo "      \"disabled\": false,"
echo "      \"autoApprove\": []"
echo "    }"
echo ""
echo "Once added, fully restart your Antigravity Desktop app."
echo "Done!"
