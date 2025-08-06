export default {
  // Source directory for markdown files
  docs: './src/docs',
  
  // Output directory for generated pages
  out: './build',
  
  // Generate routes for all markdown files
  routes: true,
  
  // Enable static generation
  static: true,
  
  // Base path for deployment
  base: process.env.NODE_ENV === 'production' ? '/rhema' : '',
}; 