#!/bin/bash
# Entrypoint for ivy-amass tool wrapper

# Read JSON from stdin
input=$(cat)
target=$(echo "$input" | jq -r '.target')
mode=$(echo "$input" | jq -r '.mode // "passive"')

if [ -z "$target" ] || [ "$target" == "null" ]; then
    echo '{"error": "Missing target parameter"}' >&2
    exit 1
fi

mode_flag="-passive"
if [ "$mode" == "active" ]; then
    mode_flag="-active"
fi

# Run amass
echo "Running amass on target: $target in $mode mode..." >&2
amass enum $mode_flag -d "$target" -json /app/output.json > /app/raw.txt 2>&1

# Parse output into MCP format
echo "Parsing output..." >&2

jq -c '{
  tool_id: "ivy_amass",
  status: "completed",
  findings: [
    inputs | select(.name != null) | {
      type: "Subdomain",
      data: { name: .name, source: .source },
      confidence: 0.9,
      relationships: [
        { type: "HAS_SUBDOMAIN", from_type: "Domain", from_key: .domain }
      ]
    }
  ],
  raw_output_file: "/app/raw.txt"
}' /app/output.json

# If no json output was created, return empty findings
if [ ! -f /app/output.json ]; then
  cat <<EOF
{
  "tool_id": "ivy_amass",
  "status": "completed",
  "findings": [],
  "raw_output_file": "/app/raw.txt"
}
EOF
fi
