# Steam as a Data Source

<p align="center" style="color:#888; font-size:1.1em;">
  <em>The main data provider powering Crowbar API.</em>
</p>


Steam is the primary and most comprehensive data source for Crowbar API. Most endpoints rely on official Steam Web APIs and public Steam Community data to deliver up-to-date information for developers and users.

## 🔍 What We Fetch from Steam

- **Market Data**: Item prices, listings, sales history, and market trends.
- **User Profiles**: Public profile info, avatars, Steam level, and more.
- **Inventories**: User inventories for games like CS:GO, Dota 2, TF2, etc.
- **Game Details**: App info, achievements, stats, and store data.
- **Community Content**: Public comments, groups, and badges.


## 📦 Example Endpoints Using Steam

| Endpoint                        | Description                       |
|----------------------------------|-----------------------------------|
| `/api/steam/market/search`       | Search Steam Market for items      |
| `/api/steam/user/inventory`      | Get a user's public inventory      |
| `/api/steam/game/730`            | Get info about CS:GO (appid 730)   |


## ⚠️ Limitations & Notes

- Some data is only available for public profiles or inventories.
- Steam APIs may have rate limits or temporary outages.
- Crowbar does not require a Steam API key for public endpoints, but respects all Steam terms of use.


<p align="center" style="color:#1c60ff; font-size:1.08em; margin-top:2em;">
  <b>Want to know more or found an issue?</b> <br>
  Check our <a href="../data-sources.md">Data Sources</a> page or open an issue!
</p>
