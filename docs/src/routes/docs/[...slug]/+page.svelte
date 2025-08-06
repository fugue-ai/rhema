<script>
import { marked } from 'marked';
import { onMount } from 'svelte';

export let data;

let content = '';
let title = '';
let loading = true;
let error = false;

onMount(async () => {
  try {
    const slug = data.slug;
    if (!slug) {
      error = true;
      return;
    }

    // Try to load the markdown file
    const response = await fetch(`/docs/${slug}.md`);
    if (!response.ok) {
      error = true;
      return;
    }

    const markdown = await response.text();
    
    // Extract title from first heading
    const titleMatch = markdown.match(/^#\s+(.+)$/m);
    title = titleMatch ? titleMatch[1] : slug.split('/').pop() || 'Documentation';
    
    // Process markdown
    content = marked(markdown);
    loading = false;
  } catch (err) {
    console.error('Error loading document:', err);
    error = true;
  }
});
</script>

<svelte:head>
  <title>{title || 'Documentation'} - Rhema Documentation</title>
  <meta name="description" content="Rhema documentation for {title || 'Documentation'}" />
</svelte:head>

<div class="docs-page">
  <div class="docs-content">
    <header class="docs-header">
      <h1>{title || 'Loading...'}</h1>
      <div class="breadcrumb">
        <a href="/">Home</a>
        <span class="separator">/</span>
        <a href="/docs">Documentation</a>
        <span class="separator">/</span>
        <span class="current">{title || 'Loading...'}</span>
      </div>
    </header>

    {#if loading}
      <div class="loading">
        <p>Loading documentation...</p>
      </div>
    {:else if error}
      <div class="error">
        <h2>Document Not Found</h2>
        <p>The requested documentation page could not be found.</p>
        <a href="/docs">Return to Documentation</a>
      </div>
    {:else}
      <article class="markdown-content">
        {@html content}
      </article>
    {/if}
  </div>
</div>

<style>
  .docs-page {
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  .docs-header {
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--sb-border-color);
  }

  .docs-header h1 {
    color: var(--sb-header-color);
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  .breadcrumb {
    font-size: 0.875rem;
    color: var(--sb-text-muted);
  }

  .breadcrumb a {
    color: var(--sb-primary-color);
    text-decoration: none;
  }

  .breadcrumb a:hover {
    text-decoration: underline;
  }

  .separator {
    margin: 0 0.5rem;
  }

  .current {
    color: var(--sb-text-muted);
  }

  .docs-content {
    line-height: 1.6;
  }

  .markdown-content {
    color: var(--sb-text-color);
  }

  .markdown-content h1,
  .markdown-content h2,
  .markdown-content h3,
  .markdown-content h4,
  .markdown-content h5,
  .markdown-content h6 {
    color: var(--sb-header-color);
    margin-top: 2rem;
    margin-bottom: 1rem;
  }

  .markdown-content h1 {
    font-size: 2.5rem;
    border-bottom: 2px solid var(--sb-border-color);
    padding-bottom: 0.5rem;
  }

  .markdown-content h2 {
    font-size: 2rem;
    border-bottom: 1px solid var(--sb-border-color);
    padding-bottom: 0.25rem;
  }

  .markdown-content h3 {
    font-size: 1.5rem;
  }

  .markdown-content p {
    margin-bottom: 1rem;
  }

  .markdown-content ul,
  .markdown-content ol {
    margin-bottom: 1rem;
    padding-left: 2rem;
  }

  .markdown-content li {
    margin-bottom: 0.5rem;
  }

  .markdown-content code {
    background: var(--sb-code-background);
    color: var(--sb-code-color);
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 0.875rem;
  }

  .markdown-content pre {
    background: var(--sb-code-background);
    color: var(--sb-code-color);
    padding: 1rem;
    border-radius: 0.5rem;
    overflow-x: auto;
    margin: 1rem 0;
  }

  .markdown-content pre code {
    background: none;
    padding: 0;
  }

  .markdown-content blockquote {
    border-left: 4px solid var(--sb-primary-color);
    padding-left: 1rem;
    margin: 1rem 0;
    font-style: italic;
    color: var(--sb-text-muted);
  }

  .markdown-content table {
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
  }

  .markdown-content th,
  .markdown-content td {
    border: 1px solid var(--sb-border-color);
    padding: 0.5rem;
    text-align: left;
  }

  .markdown-content th {
    background: var(--sb-background-muted);
    font-weight: 600;
  }
</style> 