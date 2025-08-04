<script lang="ts">
import { onMount } from 'svelte';
import { marked } from 'marked';

let content = $state('');

onMount(async () => {
  try {
    const response = await fetch('/docs/index.md');
    if (response.ok) {
      const markdown = await response.text();
      content = await marked(markdown);
    }
  } catch (error) {
    console.error('Failed to load documentation:', error);
    content = await marked('# Rhema Documentation\n\nWelcome to the Rhema documentation.');
  }
});
</script>

<div class="home-content">
	<div class="hero">
		<h1>Rhema Documentation</h1>
		<p class="subtitle">Comprehensive documentation for the Rhema project</p>
		<div class="hero-actions">
			<a href="/docs/getting-started/quick-start" class="btn btn-primary">Get Started</a>
			<a href="/docs/user-guide/cli-command-reference" class="btn btn-secondary">CLI Reference</a>
		</div>
	</div>
	
	<div class="content">
		{#if content}
			{@html content}
		{:else}
			<div class="loading">Loading documentation...</div>
		{/if}
	</div>
	
	<div class="quick-links">
		<h2>Quick Navigation</h2>
		<div class="link-grid">
			<div class="link-card">
				<h3>Getting Started</h3>
				<ul>
					<li><a href="/docs/getting-started/quick-start">Quick Start</a></li>
					<li><a href="/docs/getting-started/workspace-quick-start">Workspace Setup</a></li>
					<li><a href="/docs/getting-started/refactoring-to-workspace">Refactoring Guide</a></li>
				</ul>
			</div>
			
			<div class="link-card">
				<h3>User Guide</h3>
				<ul>
					<li><a href="/docs/user-guide/cli-command-reference">CLI Commands</a></li>
					<li><a href="/docs/user-guide/configuration-management">Configuration</a></li>
					<li><a href="/docs/user-guide/batch-operations">Batch Operations</a></li>
				</ul>
			</div>
			
			<div class="link-card">
				<h3>Core Features</h3>
				<ul>
					<li><a href="/docs/core-features/lock-configuration-system">Lock Configuration</a></li>
					<li><a href="/docs/core-features/lock-file-cache-system">Cache System</a></li>
					<li><a href="/docs/core-features/lock-file-ai-integration">AI Integration</a></li>
				</ul>
			</div>
			
			<div class="link-card">
				<h3>Development</h3>
				<ul>
					<li><a href="/docs/development-setup/development">Setup Guide</a></li>
					<li><a href="/docs/architecture/">Architecture</a></li>
					<li><a href="/docs/examples/">Examples</a></li>
				</ul>
			</div>
		</div>
	</div>
</div>

<style>
	.home-content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}
	
	.hero {
		text-align: center;
		padding: 4rem 0;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		color: white;
		border-radius: 1rem;
		margin-bottom: 3rem;
	}
	
	.hero h1 {
		font-size: 3rem;
		font-weight: bold;
		margin-bottom: 1rem;
	}
	
	.subtitle {
		font-size: 1.25rem;
		margin-bottom: 2rem;
		opacity: 0.9;
	}
	
	.hero-actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}
	
	.btn {
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		text-decoration: none;
		font-weight: 600;
		transition: all 0.2s;
	}
	
	.btn-primary {
		background: white;
		color: #667eea;
	}
	
	.btn-primary:hover {
		background: #f8fafc;
		transform: translateY(-2px);
	}
	
	.btn-secondary {
		background: transparent;
		color: white;
		border: 2px solid white;
	}
	
	.btn-secondary:hover {
		background: white;
		color: #667eea;
		transform: translateY(-2px);
	}
	
	.content {
		margin-bottom: 3rem;
		line-height: 1.6;
	}
	
	.content :global(h1) {
		font-size: 2.5rem;
		font-weight: bold;
		margin-bottom: 1rem;
		color: #1e293b;
	}
	
	.content :global(h2) {
		font-size: 1.8rem;
		font-weight: bold;
		margin: 2rem 0 1rem 0;
		color: #334155;
	}
	
	.content :global(h3) {
		font-size: 1.4rem;
		font-weight: bold;
		margin: 1.5rem 0 0.5rem 0;
		color: #475569;
	}
	
	.content :global(p) {
		margin-bottom: 1rem;
	}
	
	.content :global(ul), .content :global(ol) {
		margin: 1rem 0;
		padding-left: 2rem;
	}
	
	.content :global(li) {
		margin-bottom: 0.5rem;
	}
	
	.content :global(code) {
		background: #f1f5f9;
		padding: 0.2rem 0.4rem;
		border-radius: 0.25rem;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 0.9em;
	}
	
	.content :global(pre) {
		background: #1e293b;
		color: #e2e8f0;
		padding: 1rem;
		border-radius: 0.5rem;
		overflow-x: auto;
		margin: 1rem 0;
	}
	
	.content :global(pre code) {
		background: none;
		padding: 0;
		color: inherit;
	}
	
	.content :global(blockquote) {
		border-left: 4px solid #3b82f6;
		padding-left: 1rem;
		margin: 1rem 0;
		color: #64748b;
	}
	
	.content :global(a) {
		color: #3b82f6;
		text-decoration: none;
	}
	
	.content :global(a:hover) {
		text-decoration: underline;
	}
	
	.loading {
		text-align: center;
		padding: 2rem;
		color: #64748b;
	}
	
	.quick-links {
		margin-top: 3rem;
	}
	
	.quick-links h2 {
		text-align: center;
		font-size: 2rem;
		margin-bottom: 2rem;
		color: #1e293b;
	}
	
	.link-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 2rem;
	}
	
	.link-card {
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 0.75rem;
		padding: 1.5rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		transition: all 0.2s;
	}
	
	.link-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}
	
	.link-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #1e293b;
	}
	
	.link-card ul {
		list-style: none;
		padding: 0;
		margin: 0;
	}
	
	.link-card li {
		margin-bottom: 0.5rem;
	}
	
	.link-card a {
		color: #3b82f6;
		text-decoration: none;
		font-size: 0.875rem;
		transition: color 0.2s;
	}
	
	.link-card a:hover {
		color: #1d4ed8;
		text-decoration: underline;
	}
	
	@media (max-width: 768px) {
		.hero h1 {
			font-size: 2rem;
		}
		
		.hero-actions {
			flex-direction: column;
			align-items: center;
		}
		
		.link-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
