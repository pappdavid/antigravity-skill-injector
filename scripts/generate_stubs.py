#!/usr/bin/env python3
import json
import os
import argparse

def main():
    parser = argparse.ArgumentParser(description="Generate lightweight native stubs for Antigravity.")
    parser.add_argument("--registry", required=True, help="Path to skills_registry.json")
    parser.add_argument("--output-dir", required=True, help="Antigravity plugins directory to write stubs to")
    parser.add_argument("--plugin-name", default="mcp-injected-skills", help="Name of the plugin folder")
    args = parser.parse_args()

    with open(args.registry, "r") as f:
        skills = json.load(f)

    plugin_dir = os.path.join(args.output_dir, args.plugin_name)
    skills_dir = os.path.join(plugin_dir, "skills")
    
    os.makedirs(skills_dir, exist_ok=True)

    # Create plugin.json
    plugin_json = {
        "name": args.plugin_name,
        "description": "Dynamically loaded skills via MCP to prevent context bloat."
    }
    with open(os.path.join(plugin_dir, "plugin.json"), "w") as f:
        json.dump(plugin_json, f, indent=2)

    count = 0
    for skill_id, skill_data in skills.items():
        name = skill_data.get("name", skill_id)
        
        # Minify description to prevent context bloat
        full_desc = skill_data.get("description", "")
        desc = full_desc.split('.')[0] if '.' in full_desc else full_desc
        if len(desc) > 80:
            desc = desc[:77] + "..."
            
        skill_folder = os.path.join(skills_dir, skill_id)
        os.makedirs(skill_folder, exist_ok=True)
        
        # Ultra-minimal stub
        skill_content = f"""---
name: {name}
description: "{desc}"
---
Run MCP `load_skill` with `"{skill_id}"` for instructions.
"""
        with open(os.path.join(skill_folder, "SKILL.md"), "w") as f:
            f.write(skill_content)
        count += 1

    print(f"Successfully generated {count} native stub skills in {plugin_dir}")

if __name__ == "__main__":
    main()
