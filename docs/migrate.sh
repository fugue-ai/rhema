#!/bin/bash

# Rhema Documentation Migration Script
# This script helps migrate the documentation from MkDocs to KitDocs

set -e

echo "🚀 Starting Rhema Documentation Migration..."

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: package.json not found. Please run this script from the kitdocs-docs directory."
    exit 1
fi

# Install dependencies if not already installed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Copy documentation files if they don't exist
if [ ! -d "src/docs" ]; then
    echo "📁 Copying documentation files..."
    cp -r ../docs/* src/docs/
fi

# Copy to static directory for serving
if [ ! -d "static/docs" ]; then
    echo "📁 Copying documentation to static directory..."
    cp -r src/docs static/
fi

# Build the project
echo "🔨 Building the documentation site..."
npm run build

echo "✅ Migration setup complete!"
echo ""
echo "Next steps:"
echo "1. Run 'npm run dev' to start the development server"
echo "2. Visit http://localhost:5173 to see the documentation"
echo "3. Fix any remaining issues with the KitDocs integration"
echo ""
echo "Current issues to address:"
echo "- KitDocs plugin configuration"
echo "- Layout component integration"
echo "- Script tag syntax in Svelte 5"
echo "- Dynamic routing for documentation pages"
echo ""
echo "See README.md for more details on the migration process." 