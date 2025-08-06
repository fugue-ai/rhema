<script>
import { onMount, onDestroy } from 'svelte';

export const formula = '';
export const display = false; // true for block math, false for inline
export const config = {};

let container;
let mathJaxLoaded = false;

onMount(() => {
  loadMathJax();
});

onDestroy(() => {
  // Clean up MathJax if needed
  if (window.MathJax) {
    window.MathJax.Hub?.Queue?.clear?.();
  }
});

async function loadMathJax() {
  if (mathJaxLoaded) return;

  try {
    // Load MathJax dynamically
    const script = document.createElement('script');
    script.src = 'https://polyfill.io/v3/polyfill.min.js?features=es6';
    document.head.appendChild(script);

    await new Promise((resolve) => {
      script.onload = resolve;
    });

    const mathJaxScript = document.createElement('script');
    mathJaxScript.id = 'MathJax-script';
    mathJaxScript.src = 'https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js';
    mathJaxScript.async = true;
    document.head.appendChild(mathJaxScript);

    await new Promise((resolve) => {
      mathJaxScript.onload = resolve;
    });

    // Configure MathJax
    window.MathJax = {
      tex: {
        inlineMath: [
          ['$', '$'],
          ['\\(', '\\)'],
        ],
        displayMath: [
          ['$$', '$$'],
          ['\\[', '\\]'],
        ],
        processEscapes: true,
        processEnvironments: true,
      },
      options: {
        ignoreHtmlClass: 'tex2jax_ignore',
        processHtmlClass: 'tex2jax_process',
      },
      svg: {
        fontCache: 'global',
      },
      ...config,
    };

    mathJaxLoaded = true;
    renderMath();
  } catch (error) {
    console.error('Failed to load MathJax:', error);
  }
}

function renderMath() {
  if (!container || !mathJaxLoaded) return;

  // Set the content
  if (display) {
    container.innerHTML = `$$${formula}$$`;
  } else {
    container.innerHTML = `$${formula}$`;
  }

  // Render with MathJax
  if (window.MathJax?.typesetPromise) {
    window.MathJax.typesetPromise([container]).catch((error) => {
      console.error('MathJax rendering error:', error);
    });
  }
}

$: if (formula && container && mathJaxLoaded) {
  renderMath();
}
</script>

<div 
	bind:this={container} 
	class="math-container" 
	class:display-math={display}
	class:inline-math={!display}
	role="math"
	aria-label="Mathematical formula"
>
	{#if !mathJaxLoaded}
		<span class="math-placeholder">
			{display ? '$$' : '$'}{formula}{display ? '$$' : '$'}
		</span>
	{/if}
</div>

<style>
	.math-container {
		display: inline;
	}

	.math-container.display-math {
		display: block;
		text-align: center;
		margin: 1rem 0;
	}

	.math-container.inline-math {
		display: inline;
	}

	.math-placeholder {
		font-family: 'Times New Roman', serif;
		font-style: italic;
		color: var(--text-muted);
		background: var(--code-bg);
		padding: 0.2rem 0.4rem;
		border-radius: 0.25rem;
		border: 1px dashed var(--border-color);
	}

	/* MathJax styling overrides */
	.math-container :global(.MathJax) {
		outline: none;
	}

	.math-container :global(.MathJax_Display) {
		margin: 1rem 0;
	}

	/* Print styles */
	@media print {
		.math-container {
			break-inside: avoid;
		}

		.math-placeholder {
			background: #f5f5f5;
			border: 1px solid #ccc;
		}
	}
</style> 