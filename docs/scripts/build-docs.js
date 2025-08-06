import { readFileSync, writeFileSync, mkdirSync, existsSync, readdirSync, statSync } from 'fs';
import { join, dirname, relative } from 'path';
import { marked } from 'marked';

// Configure marked for GitHub Pages compatibility
marked.setOptions({
  gfm: true,
  breaks: true,
  headerIds: true,
  mangle: false
});

function convertMarkdownToHtml(markdownPath, outputPath) {
  try {
    const markdown = readFileSync(markdownPath, 'utf-8');
    const html = marked(markdown);
    
    // Create directory if it doesn't exist
    const outputDir = dirname(outputPath);
    if (!existsSync(outputDir)) {
      mkdirSync(outputDir, { recursive: true });
    }
    
    // Calculate relative path to build root for assets
    const relativePath = relative(outputDir, join(process.cwd(), 'build'));
    const assetPrefix = relativePath === '' ? '.' : relativePath;
    
    // Create HTML page with proper structure
    const fullHtml = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Rhema Documentation</title>
    <link rel="stylesheet" href="${assetPrefix}/_app/immutable/assets/0.CUkH5Dtm.css">
    <script type="module" data-sveltekit-hydrate="1" src="${assetPrefix}/_app/immutable/entry/start.Dh5_5E8I.js"></script>
</head>
<body data-sveltekit-preload-data="hover">
    <div style="display: contents">
        <div class="docs-page">
            <div class="docs-content">
                <article class="markdown-content">
                    ${html}
                </article>
            </div>
        </div>
    </div>
</body>
</html>`;
    
    writeFileSync(outputPath, fullHtml);
    console.log(`Generated: ${outputPath}`);
  } catch (err) {
    console.error(`Error converting ${markdownPath}:`, err);
  }
}

function processDirectory(inputDir, outputDir) {
  function walkDir(dir) {
    const files = readdirSync(dir);
    
    files.forEach(file => {
      const filePath = join(dir, file);
      const stat = statSync(filePath);
      
      if (stat.isDirectory()) {
        walkDir(filePath);
      } else if (file.endsWith('.mdx')) {
        const relativePath = relative(inputDir, filePath);
        const htmlPath = relativePath.replace(/\.mdx$/, '.html');
        const outputPath = join(outputDir, htmlPath);
        
        convertMarkdownToHtml(filePath, outputPath);
      }
    });
  }
  
  walkDir(inputDir);
}

// Convert all markdown files
const docsDir = join(process.cwd(), 'src', 'docs');
const buildDir = join(process.cwd(), 'build');

console.log('Converting markdown files to HTML...');
processDirectory(docsDir, buildDir);
console.log('Done!'); 