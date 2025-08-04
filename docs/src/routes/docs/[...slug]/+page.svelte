<script>
import { onMount } from 'svelte';
import {
  processMarkdown,
  initializeMathJax,
  initializeCodeHighlighting,
} from '$lib/markdownProcessor.js';

export let data;

let content = '';
const title = data.title;
const toc = data.toc;

onMount(() => {
  // Process markdown with enhanced features
  content = processMarkdown(data.markdown);

  // Initialize enhanced features after content is rendered
  setTimeout(() => {
    initializeCodeHighlighting();
    initializeMathJax();
  }, 100);
});

function scrollToHeading(id) {
  const element = document.getElementById(id);
  if (element) {
    element.scrollIntoView({ behavior: 'smooth' });
  }
}
</script>

<svelte:head>
	<title>{title} - Rhema Documentation</title>
	<meta name="description" content="Rhema documentation for {title}" />
</svelte:head>

<div class="docs-page">
	<div class="docs-layout">
		{#if toc.length > 0}
			<aside class="toc-sidebar">
				<h3>Table of Contents</h3>
				<nav class="toc-nav">
					{#each toc as item}
						<a 
							href="#{item.id}" 
							class="toc-link toc-level-{item.level}"
							on:click|preventDefault={() => scrollToHeading(item.id)}
						>
							{item.text}
						</a>
					{/each}
				</nav>
			</aside>
		{/if}

		<main class="docs-content">
			<header class="docs-header">
				<h1>{title}</h1>
				<div class="breadcrumb">
					<a href="/">Home</a>
					<span class="separator">/</span>
					<a href="/docs">Documentation</a>
					<span class="separator">/</span>
					<span class="current">{title}</span>
				</div>
			</header>

			<article class="docs-article">
				{@html content}
			</article>
		</main>
	</div>
</div>

<style>
	.docs-page {
		min-height: 100vh;
		background: var(--background-color);
		color: var(--text-color);
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 50vh;
		gap: 1rem;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid var(--border-color);
		border-top: 4px solid var(--primary-color);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}

	.error {
		text-align: center;
		padding: 2rem;
		max-width: 600px;
		margin: 0 auto;
	}

	.error h1 {
		color: var(--error-color);
		margin-bottom: 1rem;
	}

	.btn {
		display: inline-block;
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		text-decoration: none;
		font-weight: 500;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: var(--primary-color);
		color: white;
	}

	.btn-primary:hover {
		background: var(--primary-hover);
	}

	.docs-layout {
		display: grid;
		grid-template-columns: 1fr 300px;
		gap: 2rem;
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}

	.docs-content {
		min-width: 0;
	}

	.docs-header {
		margin-bottom: 2rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-color);
	}

	.docs-header h1 {
		margin: 0 0 1rem 0;
		font-size: 2.5rem;
		font-weight: 700;
		color: var(--heading-color);
	}

	.breadcrumb {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.9rem;
		color: var(--text-muted);
	}

	.breadcrumb a {
		color: var(--link-color);
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.breadcrumb .separator {
		color: var(--text-muted);
	}

	.breadcrumb .current {
		color: var(--text-color);
		font-weight: 500;
	}

	.docs-article {
		line-height: 1.6;
	}

	.docs-article :global(h1),
	.docs-article :global(h2),
	.docs-article :global(h3),
	.docs-article :global(h4),
	.docs-article :global(h5),
	.docs-article :global(h6) {
		margin-top: 2rem;
		margin-bottom: 1rem;
		color: var(--heading-color);
	}

	.docs-article :global(h1) {
		font-size: 2rem;
		font-weight: 700;
	}

	.docs-article :global(h2) {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.docs-article :global(h3) {
		font-size: 1.25rem;
		font-weight: 600;
	}

	.docs-article :global(p) {
		margin-bottom: 1rem;
	}

	.docs-article :global(code) {
		background: var(--code-bg);
		padding: 0.2rem 0.4rem;
		border-radius: 0.25rem;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.9em;
	}

	.docs-article :global(pre) {
		background: var(--code-bg);
		padding: 1rem;
		border-radius: 0.5rem;
		overflow-x: auto;
		margin: 1rem 0;
	}

	.docs-article :global(pre code) {
		background: none;
		padding: 0;
	}

	.docs-article :global(blockquote) {
		border-left: 4px solid var(--primary-color);
		padding-left: 1rem;
		margin: 1rem 0;
		color: var(--text-muted);
	}

	.docs-article :global(ul),
	.docs-article :global(ol) {
		margin: 1rem 0;
		padding-left: 2rem;
	}

	.docs-article :global(li) {
		margin-bottom: 0.5rem;
	}

	.docs-article :global(table) {
		width: 100%;
		border-collapse: collapse;
		margin: 1rem 0;
	}

	.docs-article :global(th),
	.docs-article :global(td) {
		padding: 0.75rem;
		border: 1px solid var(--border-color);
		text-align: left;
	}

	.docs-article :global(th) {
		background: var(--table-header-bg);
		font-weight: 600;
	}

	.toc-sidebar {
		position: sticky;
		top: 2rem;
		height: fit-content;
		background: var(--sidebar-bg);
		border: 1px solid var(--border-color);
		border-radius: 0.5rem;
		padding: 1.5rem;
	}

	.toc-sidebar h3 {
		margin: 0 0 1rem 0;
		font-size: 1.1rem;
		font-weight: 600;
		color: var(--heading-color);
	}

	.toc-nav {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.toc-link {
		display: block;
		padding: 0.5rem;
		border-radius: 0.25rem;
		text-decoration: none;
		color: var(--text-color);
		transition: all 0.2s ease;
		font-size: 0.9rem;
		line-height: 1.4;
	}

	.toc-link:hover {
		background: var(--hover-bg);
		color: var(--primary-color);
	}

	.toc-level-1 { padding-left: 0.5rem; }
	.toc-level-2 { padding-left: 1rem; }
	.toc-level-3 { padding-left: 1.5rem; }
	.toc-level-4 { padding-left: 2rem; }
	.toc-level-5 { padding-left: 2.5rem; }
	.toc-level-6 { padding-left: 3rem; }

	@media (max-width: 768px) {
		.docs-layout {
			grid-template-columns: 1fr;
			padding: 1rem;
		}

		.toc-sidebar {
			position: static;
			margin-top: 2rem;
		}

		.docs-header h1 {
			font-size: 2rem;
		}

		.breadcrumb {
			flex-wrap: wrap;
		}
	}
</style> 