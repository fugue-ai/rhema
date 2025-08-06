<script lang="ts">
import { onNavigate } from '$app/navigation';
import { base } from '$app/paths';
import { writable } from 'svelte/store';
import 'kitdocs/sb.css';
import 'kitdocs/md.css';
import '../app.css';
// import favicon from '$lib/assets/favicon.svg';
import appData from '../app.json';
import Search from '$lib/Search.svelte';
import { searchIndex } from '$lib/searchIndex.js';

const { children } = $props();

// Initialize stores with our app.json data
const appJsonDataStore = writable(appData);
const appStore = writable({
  theme: 'light',
  isTopNavLinksOpen: false,
  isSideNavOpen: false,
  isSearchOpen: false,
  scrollY: 0,
});
const metaTagsStore = writable({
  appName: appData.projectName || 'KitDocs',
  title: 'Rhema Documentation',
  description: 'Comprehensive documentation for the Rhema project',
  url: '',
  image: '',
  ogType: 'website' as 'website' | 'article',
});

const isDarkMode = $derived($appStore.theme === 'dark');
let appDiv: HTMLDivElement;

const onThemeChange = async (newTheme: string) => {
  appStore.update((data) => {
    data.theme = newTheme;
    return data;
  });
};

onNavigate((data) => {
  const stopFunc = data.from?.url.href === data.to?.url.href;
  if (stopFunc) return;
  appDiv?.scrollTo({ top: 0, behavior: 'smooth' });
});

function handleScroll() {
  appStore.update((data) => {
    data.scrollY = appDiv?.scrollTop || 0;
    return data;
  });
}
</script>

<svelte:head>
	<!-- <link rel="icon" href={favicon} /> -->
	<title>{$metaTagsStore.title}</title>
	<meta name="title" content={$metaTagsStore.title} />
	<meta name="description" content={$metaTagsStore.description} />
	<meta property="og:type" content={$metaTagsStore.ogType} />
	<meta property="og:url" content={$metaTagsStore.url} />
	<meta property="og:title" content={$metaTagsStore.title} />
	<meta property="og:description" content={$metaTagsStore.description} />
	<meta property="og:image" content={$metaTagsStore.image} />
	<meta property="twitter:card" content="summary_large_image" />
	<meta property="twitter:url" content={$metaTagsStore.url} />
	<meta property="twitter:title" content={$metaTagsStore.title} />
	<meta property="twitter:description" content={$metaTagsStore.description} />
	<meta property="twitter:image" content={$metaTagsStore.image} />
</svelte:head>

<div class="app" class:dark={isDarkMode} bind:this={appDiv} onscroll={handleScroll}>
	<!-- Skip to content link for accessibility -->
	<a href="#main-content" class="skip-link sr-only">Skip to main content</a>
	
	<!-- Top Navigation -->
	<header class="topNav">
		<div class="navContent">
			<div class="logo">
				<a href={base || "/"} aria-label="Rhema Documentation Home">Rhema</a>
			</div>
			<div class="navActions">
				<Search searchIndex={searchIndex} on:select={(event) => {
					const result = event.detail;
					window.location.href = (base || "") + `/docs/${result.path}`;
				}} />
				<button 
					class="themeToggle" 
					onclick={() => onThemeChange(isDarkMode ? 'light' : 'dark')}
					aria-label="Toggle {isDarkMode ? 'light' : 'dark'} mode"
				>
					{#if isDarkMode}
						☀️
					{:else}
						🌙
					{/if}
				</button>
			</div>
		</div>
	</header>

	<div class="appContent">
		<!-- Side Navigation -->
		<aside class="sideNav" aria-label="Documentation navigation">
			<nav>
				{#each Object.entries($appJsonDataStore.kitDocs || {}) as [section, items]}
					<div class="navSection">
						<h3 class="sectionTitle">{section.replace('-', ' ').replace(/\b\w/g, l => l.toUpperCase())}</h3>
						<ul class="navItems">
							{#each items as item}
								<li>
									<a href={(base || "") + item.href} class="navLink">
										{item.title}
									</a>
								</li>
							{/each}
						</ul>
					</div>
				{/each}
			</nav>
		</aside>

		<!-- Main Content -->
		<main class="mainContent" id="main-content">
			{@render children?.()}
		</main>
	</div>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		background: var(--sb-background);
		color: var(--sb-text-color);
		min-height: 100vh;
	}

	.topNav {
		background: var(--sb-foreground);
		border-bottom: 1px solid var(--sb-border-color);
		padding: 1rem 0;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.navContent {
		max-width: var(--sb-max-width);
		margin: 0 auto;
		padding: 0 1rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.logo a {
		font-size: 1.5rem;
		font-weight: bold;
		color: var(--sb-header-color);
		text-decoration: none;
	}

	.themeToggle {
		background: none;
		border: none;
		font-size: 1.2rem;
		cursor: pointer;
		padding: 0.5rem;
		border-radius: 0.25rem;
		transition: background-color 0.2s;
	}

	.themeToggle:hover {
		background: var(--sb-focus-color);
	}

	.appContent {
		max-width: var(--sb-max-width);
		width: 95%;
		margin: 0 auto;
		display: flex;
		gap: 2rem;
		flex: 1;
	}

	.sideNav {
		width: 280px;
		padding: 2rem 0;
		overflow-y: auto;
	}

	.navSection {
		margin-bottom: 2rem;
	}

	.sectionTitle {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--sb-text-color);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	.navItems {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.navItems li {
		margin-bottom: 0.25rem;
	}

	.navLink {
		display: block;
		padding: 0.5rem 0.75rem;
		color: var(--sb-text-color);
		text-decoration: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.navLink:hover {
		background: var(--sb-focus-color);
		color: var(--sb-header-color);
	}

	.mainContent {
		flex: 1;
		padding: 2rem 0;
	}

	@media (max-width: 768px) {
		.appContent {
			flex-direction: column;
			gap: 1rem;
		}

		.sideNav {
			width: 100%;
			padding: 1rem 0;
		}
	}
</style>
