import { defineConfig } from 'vitepress'

export default defineConfig({
  base: '/WinIsland/',
  title: "WinIsland",
  description: "A sleek, functional dynamic island for Windows",
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide' },
      { text: 'Download', link: '/download' }
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'What is WinIsland?', link: '/guide' },
          { text: 'Getting Started', link: '/getting-started' }
        ]
      },
      {
        text: 'Download',
        items: [
          { text: 'Latest Nightly', link: '/download' }
        ]
      }
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/Eatgrapes/WinIsland' }
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright æ¼?2026-present WinIsland'
    }
  }
})

