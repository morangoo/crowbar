import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  markdown: {
    theme: {
      light: "catppuccin-latte",
      dark: "catppuccin-mocha",
    },
  },
  outDir: '.vitepress/dist',
  title: "Crowbar API",
  description: "API Documentation",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Documentation', link: '/docs' }
    ],

    logo: 'https://raw.githubusercontent.com/morangoo/crowbar/master/public/crowbar.png',

    sidebar: [
      {
        text: 'Crowbar API',
        items: [
          { text: 'Getting Started', link: '/api-info/getting-started' },
        ]
      },
      {
        text: 'Steam',
        items: [
          { text: 'Steam Market', link: '/api-docs/steam-market' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/morangoo/crowbar' },
      { icon: 'discord', link: '' },
      { icon: 'patreon', link: '' },
      { icon: 'steam', link: 'https://steamcommunity.com' }
    ]
  }
})
