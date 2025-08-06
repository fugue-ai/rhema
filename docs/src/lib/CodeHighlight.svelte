<script>
import { onMount } from 'svelte';
import Prism from 'prismjs';
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-javascript';
import 'prismjs/components/prism-typescript';
import 'prismjs/components/prism-json';
import 'prismjs/components/prism-yaml';
import 'prismjs/components/prism-toml';
import 'prismjs/components/prism-rust';
import 'prismjs/components/prism-markdown';
import 'prismjs/components/prism-css';
import 'prismjs/components/prism-html';
import 'prismjs/components/prism-xml';
import 'prismjs/components/prism-docker';
import 'prismjs/components/prism-git';
import 'prismjs/components/prism-shell-session';
import 'prismjs/plugins/line-numbers/prism-line-numbers';
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard';
import 'prismjs/plugins/toolbar/prism-toolbar';

export const code = '';
export const language = 'text';
export const filename = '';
export const showLineNumbers = false;
export const showCopyButton = true;

let codeElement;
let highlighted = false;

onMount(() => {
  if (codeElement && !highlighted) {
    highlightCode();
  }
});

function highlightCode() {
  if (!codeElement) return;

  // Set the language class
  codeElement.className = `language-${language}`;

  // Highlight the code
  Prism.highlightElement(codeElement);
  highlighted = true;
}

$: if (code && codeElement && !highlighted) {
  highlightCode();
}

function copyToClipboard() {
  navigator.clipboard.writeText(code).then(() => {
    // Show a brief success message
    const button = codeElement?.parentElement?.querySelector('.copy-button');
    if (button) {
      const originalText = button.textContent;
      button.textContent = 'Copied!';
      button.classList.add('copied');
      setTimeout(() => {
        button.textContent = originalText;
        button.classList.remove('copied');
      }, 2000);
    }
  });
}
</script>

<div class="code-block" class:line-numbers={showLineNumbers}>
	{#if filename}
		<div class="code-header">
			<span class="filename">{filename}</span>
			{#if showCopyButton}
				<button class="copy-button" on:click={copyToClipboard} title="Copy to clipboard">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
						<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
					</svg>
				</button>
			{/if}
		</div>
	{/if}
	
	<pre class="code-container" class:line-numbers={showLineNumbers}>
		<code bind:this={codeElement}>{code}</code>
	</pre>
</div>

<style>
	.code-block {
		position: relative;
		margin: 1.5rem 0;
		border-radius: 0.5rem;
		overflow: hidden;
		background: var(--code-bg);
		border: 1px solid var(--border-color);
	}

	.code-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--code-header-bg);
		border-bottom: 1px solid var(--border-color);
		font-size: 0.875rem;
		font-weight: 500;
	}

	.filename {
		color: var(--text-color);
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
	}

	.copy-button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		background: none;
		border: none;
		color: var(--text-muted);
		cursor: pointer;
		border-radius: 0.25rem;
		transition: all 0.2s ease;
	}

	.copy-button:hover {
		background: var(--hover-bg);
		color: var(--text-color);
	}

	.copy-button.copied {
		color: var(--success-color);
	}

	.code-container {
		margin: 0;
		padding: 1rem;
		background: transparent;
		border: none;
		border-radius: 0;
		overflow-x: auto;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.code-container :global(code) {
		background: none;
		padding: 0;
		border: none;
		border-radius: 0;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: inherit;
		line-height: inherit;
	}

	/* Prism.js Theme Customization */
	.code-container :global(.token.comment) {
		color: var(--code-comment);
	}

	.code-container :global(.token.keyword) {
		color: var(--code-keyword);
	}

	.code-container :global(.token.string) {
		color: var(--code-string);
	}

	.code-container :global(.token.number) {
		color: var(--code-number);
	}

	.code-container :global(.token.function) {
		color: var(--code-function);
	}

	.code-container :global(.token.class-name) {
		color: var(--code-class);
	}

	.code-container :global(.token.operator) {
		color: var(--code-operator);
	}

	.code-container :global(.token.punctuation) {
		color: var(--code-punctuation);
	}

	/* Line Numbers */
	.code-container.line-numbers {
		padding-left: 3.5rem;
	}

	.code-container.line-numbers :global(.line-numbers-rows) {
		border-right: 1px solid var(--border-color);
		background: var(--code-line-numbers-bg);
	}

	.code-container.line-numbers :global(.line-numbers-rows > span) {
		color: var(--text-muted);
		font-size: 0.75rem;
	}

	/* Responsive Design */
	@media (max-width: 768px) {
		.code-container {
			font-size: 0.8rem;
			padding: 0.75rem;
		}

		.code-header {
			padding: 0.5rem 0.75rem;
			font-size: 0.8rem;
		}
	}

	/* Print Styles */
	@media print {
		.code-block {
			break-inside: avoid;
			border: 1px solid #ccc;
		}

		.code-header {
			background: #f5f5f5;
			border-bottom: 1px solid #ccc;
		}

		.copy-button {
			display: none;
		}
	}
</style> 