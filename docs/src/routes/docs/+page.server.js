import { error } from '@sveltejs/kit';
import { readFileSync } from 'fs';
import { join } from 'path';

export async function load() {
  try {
    // Load the docs index file
    let filePath;
    let markdown;

    // Try the current working directory first (for development)
    filePath = join(process.cwd(), 'src', 'docs', 'index.md');
    try {
      markdown = readFileSync(filePath, 'utf-8');
    } catch {
      // If that fails, try the docs directory relative to the project root
      filePath = join(process.cwd(), 'docs', 'src', 'docs', 'index.md');
      markdown = readFileSync(filePath, 'utf-8');
    }

    // Extract title from first heading
    const titleMatch = markdown.match(/^#\s+(.+)$/m);
    const title = titleMatch ? titleMatch[1] : 'Documentation';

    // Generate table of contents
    const toc = generateTOC(markdown);

    return {
      markdown,
      title,
      toc,
      slug: 'index',
    };
  } catch (err) {
    console.error('Error loading document:', err);
    throw error(404, 'Document not found');
  }
}

function generateTOC(markdown) {
  const lines = markdown.split('\n');
  const toc = [];
  
  for (const line of lines) {
    const headingMatch = line.match(/^(#{2,4})\s+(.+)$/);
    if (headingMatch) {
      const level = headingMatch[1].length;
      const title = headingMatch[2].trim();
      const id = title.toLowerCase().replace(/[^a-z0-9]+/g, '-');
      
      toc.push({
        level,
        title,
        id,
      });
    }
  }
  
  return toc;
} 