#!/usr/bin/env python3
import os
import json
import argparse
import re

def parse_yaml_frontmatter(content):
    frontmatter = {}
    body = content
    
    if content.startswith('---'):
        parts = content.split('---', 2)
        if len(parts) >= 3:
            yaml_content = parts[1]
            body = parts[2].strip()
            
            for line in yaml_content.split('\n'):
                line = line.strip()
                if ':' in line:
                    key, val = line.split(':', 1)
                    key = key.strip()
                    val = val.strip().strip('"').strip("'")
                    frontmatter[key] = val
                    
    return frontmatter, body

def main():
    parser = argparse.ArgumentParser(description="Migrate Antigravity skills to a registry JSON.")
    parser.add_argument("--source", required=True, help="Directory containing original SKILL.md files")
    parser.add_argument("--output", required=True, help="Path to save the output skills_registry.json")
    args = parser.parse_args()

    registry = {}
    count = 0

    if not os.path.isdir(args.source):
        print(f"Error: Source directory {args.source} does not exist.")
        return

    # Walk through the directory to find SKILL.md files
    for root, dirs, files in os.walk(args.source):
        for file in files:
            if file.endswith('.md'):
                filepath = os.path.join(root, file)
                try:
                    with open(filepath, 'r') as f:
                        content = f.read()
                        
                    frontmatter, body = parse_yaml_frontmatter(content)
                    
                    # Determine ID from folder name or file name
                    parent_dir = os.path.basename(root)
                    skill_id = parent_dir if parent_dir != "skills" else os.path.splitext(file)[0]
                    
                    name = frontmatter.get('name', skill_id)
                    description = frontmatter.get('description', '')
                    
                    # If the file defines allowed tools, we assume it's a tool-based skill.
                    # For simplicity in the open source version, we treat everything as a prompt
                    # unless a specific schema is provided.
                    skill_type = "prompt"
                    
                    registry[skill_id] = {
                        "name": name,
                        "type": skill_type,
                        "description": description,
                        "instructions": body
                    }
                    count += 1
                except Exception as e:
                    print(f"Failed to parse {filepath}: {e}")

    with open(args.output, 'w') as f:
        json.dump(registry, f, indent=2)

    print(f"Successfully migrated {count} skills into {args.output}")

if __name__ == "__main__":
    main()
