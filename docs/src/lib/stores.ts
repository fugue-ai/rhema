import { writable } from 'svelte/store';
import appData from '../app.json';

// Initialize the appJsonDataStore with our app.json data
export const appJsonDataStore = writable(appData);

// Initialize the appStore with default values
export const appStore = writable({
  theme: 'light',
  isTopNavLinksOpen: false,
  isSideNavOpen: false,
  isSearchOpen: false,
  scrollY: 0,
});

// Initialize the metaTagsStore
export const metaTagsStore = writable({
  appName: appData.projectName || 'KitDocs',
});
