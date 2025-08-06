import { marked } from 'marked';

// Configure marked with custom renderer
const renderer = new marked.Renderer();

// Custom code block rendering for syntax highlighting
renderer.code = (code, language, isEscaped) => {
  const lang = language || 'text';
  const escapedCode = isEscaped ? code : escapeHtml(code);

  return `
		<div class="code-block-wrapper">
			<div class="code-header">
				<span class="language-label">${lang}</span>
				<button class="copy-button" onclick="copyCode(this)" title="Copy to clipboard">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
					</svg>
				</button>
			</div>
			<pre class="language-${lang}"><code class="language-${lang}">${escapedCode}</code></pre>
		</div>
	`;
};

// Custom heading rendering with IDs for TOC
renderer.heading = (text, level) => {
  const id = text.toLowerCase().replace(/[^a-z0-9]+/g, '-');
  return `<h${level} id="${id}">${text}</h${level}>`;
};

// Custom link rendering with external link indicators
renderer.link = (href, title, text) => {
  const isExternal = href.startsWith('http');
  const target = isExternal ? ' target="_blank" rel="noopener noreferrer"' : '';
  const titleAttr = title ? ` title="${title}"` : '';
  const externalIcon = isExternal ? ' <span class="external-link-icon">↗</span>' : '';

  return `<a href="${href}"${target}${titleAttr}>${text}${externalIcon}</a>`;
};

// Custom table rendering for better styling
renderer.table = (header, body) => {
  return `
		<div class="table-wrapper">
			<table>
				<thead>${header}</thead>
				<tbody>${body}</tbody>
			</table>
		</div>
	`;
};

// Configure marked options
marked.setOptions({
  renderer: renderer,
  breaks: true,
  gfm: true,
  headerIds: true,
  mangle: false,
});

// Helper function to escape HTML
function escapeHtml(text) {
  const map = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;',
    '"': '&quot;',
    "'": '&#39;',
  };
  return text.replace(/[&<>"']/g, (m) => map[m]);
}

// Process markdown with enhanced features
export function processMarkdown(markdown) {
  // Pre-process math expressions
  const processedMarkdown = preprocessMath(markdown);

  // Convert markdown to HTML
  let html = marked(processedMarkdown);

  // Post-process for MathJax and code highlighting
  html = postprocessMath(html);

  return html;
}

// Pre-process math expressions to protect them from markdown processing
function preprocessMath(markdown) {
  // Replace inline math with placeholders
  markdown = markdown.replace(/\$([^$\n]+?)\$/g, (match, formula) => {
    return `MATH_INLINE_${btoa(formula).replace(/[^a-zA-Z0-9]/g, '')}`;
  });

  // Replace block math with placeholders
  markdown = markdown.replace(/\$\$([\s\S]+?)\$\$/g, (match, formula) => {
    return `MATH_BLOCK_${btoa(formula).replace(/[^a-zA-Z0-9]/g, '')}`;
  });

  return markdown;
}

// Post-process HTML to restore math expressions
function postprocessMath(html) {
  // Restore inline math
  html = html.replace(/MATH_INLINE_([a-zA-Z0-9]+)/g, (match, encoded) => {
    const formula = atob(encoded);
    return `<span class="math-inline" data-formula="${escapeHtml(formula)}">$${formula}$</span>`;
  });

  // Restore block math
  html = html.replace(/MATH_BLOCK_([a-zA-Z0-9]+)/g, (match, encoded) => {
    const formula = atob(encoded);
    return `<div class="math-block" data-formula="${escapeHtml(formula)}">$$${formula}$$</div>`;
  });

  return html;
}

// Initialize MathJax after content is loaded
export function initializeMathJax() {
  if (typeof window !== 'undefined' && window.MathJax) {
    window.MathJax.typesetPromise?.();
  }
}

// Initialize code highlighting after content is loaded
export function initializeCodeHighlighting() {
  if (typeof window !== 'undefined' && window.Prism) {
    window.Prism.highlightAll();
  }
}

// Copy code function for code blocks
if (typeof window !== 'undefined') {
  window.copyCode = (button) => {
    const codeBlock = button.closest('.code-block-wrapper').querySelector('code');
    const text = codeBlock.textContent;

    navigator.clipboard.writeText(text).then(() => {
      const originalHTML = button.innerHTML;
      button.innerHTML = '<span>Copied!</span>';
      button.classList.add('copied');

      setTimeout(() => {
        button.innerHTML = originalHTML;
        button.classList.remove('copied');
      }, 2000);
    });
  };
}
