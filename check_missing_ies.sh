#!/bin/bash

# Extract IE types from the enum
echo "Extracting IE types from enum..."
grep -E "^\s+[A-Z][a-zA-Z0-9_]+ = [0-9]+," src/ie/mod.rs | sed 's/^\s*//' | sed 's/ = .*//' > /tmp/enum_ies.txt

# Extract implemented IE modules
echo "Extracting implemented IE modules..."
ls src/ie/*.rs | grep -v mod.rs | sed 's|src/ie/||' | sed 's|\.rs$||' | sort > /tmp/implemented_ies.txt

# Convert enum names to snake_case for comparison
echo "Converting enum names to snake_case..."
python3 << 'EOF'
import re

def camel_to_snake(name):
    # Handle special cases
    name = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    name = re.sub('([a-z0-9])([A-Z])', r'\1_\2', name)
    return name.lower()

with open('/tmp/enum_ies.txt', 'r') as f:
    enum_ies = [line.strip() for line in f if line.strip()]

snake_case_ies = []
for ie in enum_ies:
    snake_ie = camel_to_snake(ie)
    snake_case_ies.append(snake_ie)

with open('/tmp/snake_case_ies.txt', 'w') as f:
    for ie in sorted(snake_case_ies):
        f.write(ie + '\n')
EOF

# Find missing implementations
echo "Finding missing implementations..."
comm -23 /tmp/snake_case_ies.txt /tmp/implemented_ies.txt > /tmp/missing_ies.txt

echo "Missing IE implementations:"
cat /tmp/missing_ies.txt

echo ""
echo "Total enum IEs: $(wc -l < /tmp/enum_ies.txt)"
echo "Total implemented: $(wc -l < /tmp/implemented_ies.txt)"
echo "Total missing: $(wc -l < /tmp/missing_ies.txt)"
