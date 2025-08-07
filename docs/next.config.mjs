import nextra from 'nextra'

const withNextra = nextra({
  contentDirBasePath: process.env.NODE_ENV === 'production' ? '/rhema' : '/',
})

export default withNextra({
  turbopack: {
    resolveAlias: {
      // Path to your `mdx-components` file with extension
      'next-mdx-import-source-file': './src/mdx-components.tsx'
    }
  }
})