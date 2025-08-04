import { error } from '@sveltejs/kit';
import { readFileSync } from 'fs';
import { join } from 'path';

export async function load({ params }) {
  try {
    const slug = params.slug;
    if (!slug) {
      throw error(404, 'Document not found');
    }

    // Construct the file path
    const filePath = join(process.cwd(), 'src', 'docs', `${slug}.md`);

    // Try to read the markdown file
    const markdown = readFileSync(filePath, 'utf-8');

    // Extract title from first heading
    const titleMatch = markdown.match(/^#\s+(.+)$/m);
    const title = titleMatch ? titleMatch[1] : slug.split('/').pop() || 'Documentation';

    // Generate table of contents
    const toc = generateTOC(markdown);

    return {
      markdown,
      title,
      toc,
      slug,
    };
  } catch (err) {
    console.error('Error loading document:', err);
    throw error(404, 'Document not found');
  }
}

function generateTOC(markdown) {
  const toc = [];
  const lines = markdown.split('\n');

  lines.forEach((line) => {
    const headingMatch = line.match(/^(#{1,6})\s+(.+)$/);
    if (headingMatch) {
      const level = headingMatch[1].length;
      const text = headingMatch[2];
      const id = text.toLowerCase().replace(/[^a-z0-9]+/g, '-');
      toc.push({ id, text, level });
    }
  });

  return toc;
}
