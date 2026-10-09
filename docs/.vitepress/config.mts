import { defineConfig } from 'vitepress';

export default defineConfig({
  title: 'Pumpkin-Duels',
  description: 'Production-ready 1v1 practice duel plugin for the Pumpkin Minecraft server',
  lang: 'en-US',
  base: '/',
  cleanUrls: true,
  themeConfig: {
    siteTitle: 'Pumpkin-Duels',
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Commands', link: '/commands' },
      { text: 'Configuration', link: '/configuration' },
      { text: 'Arenas', link: '/arenas' },
      { text: 'Kits', link: '/kits' },
      { text: 'Architecture', link: '/architecture' },
    ],
    sidebar: [
      {
        text: 'Overview',
        items: [
          { text: 'Introduction', link: '/' },
          { text: 'Getting Started', link: '/getting-started' },
        ],
      },
      {
        text: 'User Guide',
        items: [
          { text: 'Commands and Permissions', link: '/commands' },
          { text: 'Configuration', link: '/configuration' },
          { text: 'Arena Management', link: '/arenas' },
          { text: 'Kit Setup', link: '/kits' },
        ],
      },
      {
        text: 'Developer Guide',
        items: [
          { text: 'Architecture and Internals', link: '/architecture' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/synkfr/Pumpkin-Duels' },
    ],
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright (c) 2026 synkfr',
    },
    search: {
      provider: 'local',
    },
  },
});
