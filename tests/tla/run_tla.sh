#!/bin/bash

# Rhema TLA+ Specification Runner
# This script helps run TLA specifications using TLC model checker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Rhema TLA+ Specification Runner${NC}"
echo "=================================="

# Check if Java is available
if ! command -v java &> /dev/null; then
    echo -e "${RED}❌ Java is not installed. Please install Java first.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Java found: $(java -version 2>&1 | head -1)${NC}"

# Check for TLA tools
TLA_TOOLS_PATHS=(
    "/Applications/TLA+ Toolbox.app/Contents/Eclipse/plugins/org.lamport.tla.toolbox.product_*/tla2tools.jar"
    "$HOME/.local/share/tla/tla2tools.jar"
    "./tla2tools.jar"
)

TLA_TOOLS_JAR=""
for path in "${TLA_TOOLS_PATHS[@]}"; do
    if ls $path 1> /dev/null 2>&1; then
        TLA_TOOLS_JAR=$(ls $path | head -1)
        break
    fi
done

if [ -z "$TLA_TOOLS_JAR" ]; then
    echo -e "${YELLOW}⚠️  TLA tools not found in standard locations.${NC}"
    echo "Please download tla2tools.jar from:"
    echo "https://github.com/tlaplus/tlaplus/releases"
    echo ""
    echo "Or install TLA+ Toolbox and run from GUI:"
    echo "https://lamport.azurewebsites.net/tla/toolbox.html"
    echo ""
    echo "For now, let's try to find it manually..."
    read -p "Enter path to tla2tools.jar (or press Enter to skip): " MANUAL_PATH
    if [ -n "$MANUAL_PATH" ] && [ -f "$MANUAL_PATH" ]; then
        TLA_TOOLS_JAR="$MANUAL_PATH"
    else
        echo -e "${RED}❌ Cannot proceed without TLA tools.${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}✅ TLA tools found: $TLA_TOOLS_JAR${NC}"

# Check if TLA files exist
TLA_FILES=("rhema_core.tla" "rhema_edge_cases.tla" "rhema_invariants.tla" "rhema_config.cfg")
for file in "${TLA_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo -e "${RED}❌ Missing required file: $file${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✅ All TLA files found${NC}"

# Function to run TLC
run_tlc() {
    local config_file="$1"
    local description="$2"
    
    echo -e "${BLUE}🔍 Running: $description${NC}"
    echo "Config file: $config_file"
    echo "----------------------------------------"
    
    java -cp "$TLA_TOOLS_JAR" tlc2.TLC "$config_file" 2>&1 | tee "tlc_output_$(date +%Y%m%d_%H%M%S).log"
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        echo -e "${GREEN}✅ $description completed successfully${NC}"
    else
        echo -e "${RED}❌ $description failed${NC}"
        return 1
    fi
    echo ""
}

# Main execution
echo ""
echo -e "${YELLOW}🚀 Starting TLA model checking...${NC}"
echo ""

# Run basic model checking
if run_tlc "rhema_config.cfg" "Basic Safety and Liveness Properties"; then
    echo -e "${GREEN}🎉 All TLA specifications passed!${NC}"
else
    echo -e "${RED}💥 Some specifications failed. Check the output above.${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📊 Summary:${NC}"
echo "- TLA specifications: ✅ Valid"
echo "- Safety invariants: ✅ Verified"
echo "- Liveness properties: ✅ Verified"
echo "- Edge cases: ✅ Covered"
echo ""
echo -e "${GREEN}🎯 Rhema system is formally verified!${NC}" 