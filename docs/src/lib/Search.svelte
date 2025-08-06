<script>
import { createEventDispatcher } from 'svelte';
import { onMount } from 'svelte';

export const placeholder = 'Search documentation...';
export const searchIndex = [];

const dispatch = createEventDispatcher();

let searchTerm = '';
let results = [];
let isOpen = false;
let searchInput;

onMount(() => {
  // Focus search input when opened
  if (searchInput) {
    searchInput.focus();
  }
});

function handleSearch() {
  if (!searchTerm.trim()) {
    results = [];
    return;
  }

  const term = searchTerm.toLowerCase();
  results = searchIndex
    .filter((item) => {
      return (
        item.title.toLowerCase().includes(term) ||
        item.content.toLowerCase().includes(term) ||
        item.path.toLowerCase().includes(term)
      );
    })
    .slice(0, 10); // Limit to 10 results
}

function handleKeydown(event) {
  if (event.key === 'Escape') {
    closeSearch();
  }
}

function openSearch() {
  isOpen = true;
  setTimeout(() => {
    if (searchInput) {
      searchInput.focus();
    }
  }, 100);
}

function closeSearch() {
  isOpen = false;
  searchTerm = '';
  results = [];
}

function selectResult(result) {
  dispatch('select', result);
  closeSearch();
}

$: if (searchTerm) {
  handleSearch();
}
</script>

<div class="search-container">
	<!-- Search Button -->
	<button class="search-button" on:click={openSearch} aria-label="Search">
		<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			<circle cx="11" cy="11" r="8"/>
			<path d="m21 21-4.35-4.35"/>
		</svg>
		<span class="search-text">Search</span>
	</button>

	<!-- Search Modal -->
	{#if isOpen}
		<div class="search-overlay" on:click={closeSearch} on:keydown={(e) => e.key === 'Escape' && closeSearch()} role="dialog" aria-modal="true" aria-label="Search" tabindex="-1">
			<div class="search-modal" role="document">
				<div class="search-header">
					<div class="search-input-wrapper">
						<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="11" cy="11" r="8"/>
							<path d="m21 21-4.35-4.35"/>
						</svg>
						<input
							bind:this={searchInput}
							bind:value={searchTerm}
							type="text"
							placeholder={placeholder}
							class="search-input"
							on:keydown={handleKeydown}
						/>
					</div>
					<button class="close-button" on:click={closeSearch} aria-label="Close search">
						<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18"/>
							<line x1="6" y1="6" x2="18" y2="18"/>
						</svg>
					</button>
				</div>

				<div class="search-results">
					{#if searchTerm && results.length === 0}
						<div class="no-results">
							<p>No results found for "{searchTerm}"</p>
						</div>
					{:else if results.length > 0}
						{#each results as result}
							<button class="result-item" on:click={() => selectResult(result)}>
								<div class="result-title">{result.title}</div>
								<div class="result-path">{result.path}</div>
								{#if result.excerpt}
									<div class="result-excerpt">{result.excerpt}</div>
								{/if}
							</button>
						{/each}
					{:else if searchTerm}
						<div class="searching">
							<div class="spinner"></div>
							<p>Searching...</p>
						</div>
					{:else}
						<div class="search-tips">
							<h3>Search Tips</h3>
							<ul>
								<li>Try searching for specific terms or concepts</li>
								<li>Use quotes for exact phrase matching</li>
								<li>Search by file path or section name</li>
							</ul>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.search-container {
		position: relative;
	}

	.search-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: var(--button-bg);
		border: 1px solid var(--border-color);
		border-radius: 0.5rem;
		color: var(--text-color);
		cursor: pointer;
		transition: all 0.2s ease;
		font-size: 0.9rem;
	}

	.search-button:hover {
		background: var(--button-hover);
		border-color: var(--primary-color);
	}

	.search-text {
		display: none;
	}

	@media (min-width: 768px) {
		.search-text {
			display: inline;
		}
	}

	.search-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		z-index: 1000;
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 2rem;
	}

	.search-modal {
		background: var(--background-color);
		border: 1px solid var(--border-color);
		border-radius: 0.75rem;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
		width: 100%;
		max-width: 600px;
		max-height: 80vh;
		overflow: hidden;
	}

	.search-header {
		display: flex;
		align-items: center;
		padding: 1rem;
		border-bottom: 1px solid var(--border-color);
	}

	.search-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex: 1;
	}

	.search-input {
		flex: 1;
		padding: 0.75rem;
		border: none;
		background: transparent;
		color: var(--text-color);
		font-size: 1rem;
		outline: none;
	}

	.search-input::placeholder {
		color: var(--text-muted);
	}

	.close-button {
		padding: 0.5rem;
		background: none;
		border: none;
		color: var(--text-color);
		cursor: pointer;
		border-radius: 0.25rem;
		transition: background 0.2s ease;
	}

	.close-button:hover {
		background: var(--hover-bg);
	}

	.search-results {
		max-height: 60vh;
		overflow-y: auto;
	}

	.result-item {
		display: block;
		width: 100%;
		padding: 1rem;
		border: none;
		background: none;
		text-align: left;
		cursor: pointer;
		border-bottom: 1px solid var(--border-color);
		transition: background 0.2s ease;
	}

	.result-item:hover {
		background: var(--hover-bg);
	}

	.result-item:last-child {
		border-bottom: none;
	}

	.result-title {
		font-weight: 600;
		color: var(--text-color);
		margin-bottom: 0.25rem;
	}

	.result-path {
		font-size: 0.8rem;
		color: var(--text-muted);
		margin-bottom: 0.5rem;
	}

	.result-excerpt {
		font-size: 0.9rem;
		color: var(--text-color);
		line-height: 1.4;
	}

	.no-results,
	.searching,
	.search-tips {
		padding: 2rem;
		text-align: center;
		color: var(--text-muted);
	}

	.searching {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.spinner {
		width: 24px;
		height: 24px;
		border: 2px solid var(--border-color);
		border-top: 2px solid var(--primary-color);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}

	.search-tips h3 {
		margin-bottom: 1rem;
		color: var(--text-color);
	}

	.search-tips ul {
		text-align: left;
		max-width: 400px;
		margin: 0 auto;
	}

	.search-tips li {
		margin-bottom: 0.5rem;
	}

	@media (max-width: 768px) {
		.search-overlay {
			padding: 1rem;
		}

		.search-modal {
			max-height: 90vh;
		}
	}
</style> 