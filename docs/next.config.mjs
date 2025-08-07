import nextra from 'nextra'

const withNextra = nextra({
  latex: true,
  search: {
    codeblocks: false
  },
  contentDirBasePath: process.env.NODE_ENV === 'production' ? '/rhema' : '/',
})

export default withNextra({
  output: 'export',
  experimental: {
    optimizePackageImports: false,
  },
})