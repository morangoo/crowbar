# Data Sources

<p align="center" style="color:#888; font-size:1.1em;">
  <em>Where Crowbar gets its data: Steam, games, and more.</em>
</p>

---

Crowbar API aggregates and enriches data from multiple official and community sources to provide a unified, developer-friendly experience.

## 🌐 Main Data Sources

- **Steam Web APIs**  
  Official endpoints for market, user, inventory, and game data.

- **Steam Community**  
  Public user profiles, inventories, and market listings.

- **Game-specific APIs**  
  Data from games like CS:GO, Dota 2, and others (ranks, stats, etc).

- **Third-party sites**  
  Community-maintained sources for price history, item info, and more.

---

## 🧩 Example Integrations

| Source                | What is used for                | Example Endpoint                |
|-----------------------|---------------------------------|---------------------------------|
| Steam Web API         | Market, inventory, user data    | `/api/steam/market/search`      |
| Steam Community       | Public profiles, inventories    | `/api/steam/user/inventory`     |
| CS:GO API             | Player ranks, stats             | `/api/csgo/player/rank`         |
| Dota 2 API            | Match stats, player info        | `/api/dota2/player/stats`       |
| Community price sites | Price history, item details     | `/api/steam/market/item`        |

---

## 🤝 Transparency & Attribution

We respect the terms of use of all data providers. If you are a maintainer of a source and want attribution or removal, please open an issue.

---

<p align="center" style="color:#1c60ff; font-size:1.08em; margin-top:2em;">
  <b>Want to suggest a new integration?</b> <br>
  Open an issue or pull request!
</p>