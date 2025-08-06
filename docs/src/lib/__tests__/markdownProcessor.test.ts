import { describe, it, expect } from 'vitest';
import { processMarkdown } from '../markdownProcessor.js';

describe('markdownProcessor', () => {
  describe('processMarkdown', () => {
    it('should process basic markdown', () => {
      const input = '# Hello World\n\nThis is a **test**.';
      const result = processMarkdown(input);

      expect(result).toContain('<h1 id="hello-world">Hello World</h1>');
      expect(result).toContain('<p>This is a <strong>test</strong>.</p>');
    });

    it('should handle code blocks', () => {
      const input = '```javascript\nconsole.log("hello");\n```';
      const result = processMarkdown(input);

      expect(result).toContain('class="language-javascript"');
      expect(result).toContain('console.log("hello");');
    });

    it('should handle inline math', () => {
      const input = 'The formula is $E = mc^2$.';
      const result = processMarkdown(input);

      expect(result).toContain('class="math-inline"');
      expect(result).toContain('$E = mc^2$');
    });

    it('should handle block math', () => {
      const input = '$$\n\\int_{-\\infty}^{\\infty} e^{-x^2} dx = \\sqrt{\\pi}\n$$';
      const result = processMarkdown(input);

      expect(result).toContain('class="math-block"');
      expect(result).toContain('$$');
    });

    it('should handle external links', () => {
      const input = '[GitHub](https://github.com)';
      const result = processMarkdown(input);

      expect(result).toContain('target="_blank"');
      expect(result).toContain('rel="noopener noreferrer"');
      expect(result).toContain('class="external-link-icon"');
    });
  });
});
