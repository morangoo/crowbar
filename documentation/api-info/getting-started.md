# Getting Started

Welcome to Crowbar! You have two wild ways to use the API:

## 1. Use the Hosted API (The Easy Way)

You can use the Crowbar API hosted by the project maintainer—no setup, no fuss! Just send your HTTP requests to the public endpoint and instantly access Steam Market data, as well as profiles, inventories, store info, games, promotions, events, and even player-specific stats for various games.

- **Base URL:** (Ask the maintainer for the latest hosted URL)
- **Example:**
  ```http
  GET https://your-crowbar-instance.com/api/steam/market/search?appid=730&query=karambit
  ```
- Check the API docs for all available endpoints, parameters, and response formats.

## 2. Host Crowbar Locally (The Hacker Way)

Want to run your own instance, make cool changes, or hack the API to your heart’s content? Here’s how to get started locally:

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or newer)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- (Optional) [Node.js](https://nodejs.org/) if you want to build the documentation

### Steps
1. **Clone the repo:**
   ```sh
   git clone https://github.com/morangoo/crowbar.git
   cd crowbar/api
   ```
2. **Build and run the API:**
   ```sh
   cargo run
   ```
   The API will be available at `http://localhost:8000` by default.
3. **Try it out:**
   ```http
   GET http://localhost:8000/api/steam/market/search?appid=730&query=karambit
   ```
4. **Make changes:**
   - Edit the Rust source files in `api/src/` to add features, tweak logic, or go wild!
   - Restart the server to see your changes.

### Docs & Endpoints
- Full API documentation and examples are in the `documentation/` folder and on the docs site.
- Explore endpoints for Steam Market, profiles, inventories, store, games, promotions, events, and player-specific game stats!

---

Whether you use the hosted API or run your own, Crowbar is yours to play with. Fork it, break it, improve it, and share your creations with the community!