import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

export function buildSearchIndex(docsPath = 'src/docs') {
  const searchIndex = [];

  function processDirectory(dirPath, basePath = '') {
    const items = readdirSync(dirPath);

    for (const item of items) {
      const fullPath = join(dirPath, item);
      const relativePath = join(basePath, item);
      const stats = statSync(fullPath);

      if (stats.isDirectory()) {
        // Recursively process subdirectories
        processDirectory(fullPath, relativePath);
      } else if (item.endsWith('.md')) {
        // Process markdown files
        try {
          const content = readFileSync(fullPath, 'utf-8');
          const title = extractTitle(content, item);
          const excerpt = extractExcerpt(content);

          searchIndex.push({
            title,
            path: relativePath.replace(/\.md$/, ''),
            content: stripMarkdown(content),
            excerpt,
          });
        } catch (error) {
          console.warn(`Failed to process ${fullPath}:`, error.message);
        }
      }
    }
  }

  try {
    processDirectory(docsPath);
    return searchIndex;
  } catch (error) {
    console.error('Error building search index:', error);
    return [];
  }
}

function extractTitle(content, filename) {
  // Try to extract title from first heading
  const titleMatch = content.match(/^#\s+(.+)$/m);
  if (titleMatch) {
    return titleMatch[1].trim();
  }

  // Fallback to filename
  return filename.replace(/\.md$/, '').replace(/[-_]/g, ' ');
}

function extractExcerpt(content, maxLength = 150) {
  // Remove markdown formatting and get first paragraph
  const plainText = stripMarkdown(content);
  const firstParagraph = plainText.split('\n\n')[0];

  if (firstParagraph.length <= maxLength) {
    return firstParagraph;
  }

  return firstParagraph.substring(0, maxLength) + '...';
}

function stripMarkdown(content) {
  return (
    content
      // Remove headers
      .replace(/^#{1,6}\s+.+$/gm, '')
      // Remove code blocks
      .replace(/```[\s\S]*?```/g, '')
      // Remove inline code
      .replace(/`([^`]+)`/g, '$1')
      // Remove links but keep text
      .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
      // Remove bold/italic
      .replace(/\*\*([^*]+)\*\*/g, '$1')
      .replace(/\*([^*]+)\*/g, '$1')
      // Remove HTML tags
      .replace(/<[^>]+>/g, '')
      // Clean up whitespace
      .replace(/\n+/g, '\n')
      .trim()
  );
}

// Build the search index when this module is imported
export const searchIndex = buildSearchIndex();
