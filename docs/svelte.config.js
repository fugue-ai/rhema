import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import kitdocs from 'kitdocs/plugin';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://svelte.dev/docs/kit/integrations
  // for more information about preprocessors
  preprocess: [vitePreprocess(), kitdocs()],
  extensions: ['.svelte', '.md'],

  kit: {
    // Use static adapter for GitHub Pages deployment
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html',
      precompress: false,
      strict: false,
    }),

    // KitDocs configuration
    alias: {
      $docs: './src/docs',
      $lib: './src/lib',
    },
  },
};

export default config;
