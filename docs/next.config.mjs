import nextra from 'nextra'

const basePath = process.env.NODE_ENV === 'production' ? '/rhema' : '/'

const withNextra = nextra({
  latex: true,
  search: {
    codeblocks: false
  },
  contentDirBasePath: basePath,
})

export default withNextra({
  basePath,
  output: 'export',
  experimental: {
    optimizePackageImports: false,
  },
  images: {
    unoptimized: true,
  },
})