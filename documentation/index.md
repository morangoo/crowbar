---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Crowbar API"
  text: "Documentation"
  tagline: Let steam flow!
  actions:
    - theme: brand
      text: Getting Started
      link: /api-info/getting-started
    - theme: alt
      text: Documentation
      link: /docs


features:
  - title: "🛒 Steam Market Integration"
    details: Instantly access and analyze Steam Market data with powerful endpoints.
  - title: "👤 User Profiles & Inventories"
    details: Fetch Steam user profiles, inventories, and player-specific stats.
  - title: "🎮 Store, Games & Events"
    details: Get info on games, promotions, store listings, and community events.
  - title: "🛠️ Open Source & Extensible"
    details: 100% open source, easy to extend, and built for the community.

---



## Why Crowbar?

- 🚀 **Fast & Modern**: Built with Rust and Rocket for blazing-fast performance.
- 🧩 **Super Complete**: Not just Steam data—get in-game info too, like a player's CSGO rank and more.
- 🔑 **No Steam API Key Needed**: Use all endpoints without requiring an official Steam API key.
- 🔌 **Extensible**: Open source, modular, and easy to contribute.


## Sponsors

<div align="center">
  <a href="https://github.com/sponsors"><img src="https://github.githubassets.com/images/modules/site/sponsors/sponsors-mona.svg" width="48" alt="GitHub Sponsors"/></a>
  <br>
  <em>Want to support Crowbar? <a href="https://github.com/morangoo/crowbar">Become a sponsor!</a></em>
</div>


## Changelog

- <b>2025-07-25:</b> Documentation improvements, new homepage sections, and more endpoints coming soon!
- <b>2025-07-20:</b> Added new documentation and improved market search endpoints.
- <b>2025-07-10:</b> Project started.



## Team

<div align="center">
  <a href="https://github.com/morangoo"><img src="https://avatars.githubusercontent.com/u/171176624?v=4" width="96" style="border-radius:50%" alt="morangoo"/></a>
  <br>
  <span style="font-size:1.5em; font-weight:bold;">morangoo</span> <br>
  <span style="display:inline-block; background:#1c60ff; color:#fff; border-radius:8px; padding:2px 10px; font-size:0.95em; font-weight:600; margin:4px 0 2px 0; vertical-align:middle;">Lead Developer 🧃</span>
  <br>
  <span style="display:inline-block; background:#f5e0dc; color:#24273a; border-radius:8px; padding:2px 10px; font-size:0.95em; font-weight:600; margin:2px 0 2px 0; vertical-align:middle;">🇵🇹 Portugal</span>
</div>


## FAQ

**Q: Is Crowbar free to use?**  
A: Yes! Crowbar is open source and free for everyone.

**Q: Can I contribute?**  
A: Absolutely! Check the GitHub repo for issues or open a pull request.

**Q: Where can I get support?**  
A: Open an issue on GitHub or join our community (Discord coming soon).



## Usage Examples

```http
GET /api/steam/market/search?appid=730&query=karambit
```
Response:
```json
{
  "apiversion": "v0.0.1cb",
  "code": 200,
  "success": true,
  "message": "OK",
  "size": 10,
  "data": [ /* ...results... */ ],
  "timestamp": "2025-07-25T12:00:00Z",
  "error": null
}
```


## Roadmap

| Feature                                   | Status        | Est. Release |
|-------------------------------------------|---------------|--------------|
| More game-specific stats (Dota 2, CSGO, etc.) | 🟡 Coming soon | Q4 2025      |
| Webhooks for real-time updates                | 🔜 Planned     | Q1 2026      |
| Discord bot integration                       | 🔜 Planned     | Q1 2026     |
| More community features                       | 🟡 Coming soon | Q4 2025      |



<div align="center" style="margin-top:5em; color: #888;">
  Crowbar API © 2025 by morangoo & contributors.<br>
  Open source on <a href="https://github.com/morangoo/crowbar">GitHub</a> &middot; Powered by <a href="https://vitepress.dev/">VitePress</a><br>
  Special thanks to all <a href="https://github.com/morangoo/crowbar/graphs/contributors">contributors</a> and our amazing community! 💙
</div>

