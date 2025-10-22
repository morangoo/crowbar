<p align="center">
  <img src="./public/crowbar.png" width="220px">

### For local development
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/html) (comes with Rust)
- [Redis](https://redis.io/download) (for caching)
- [Node.js](https://nodejs.org/) (if you want to use documentation or frontend)

### For Docker deployment
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Starting the API (backend)

### Using Docker (recommended)

This will start both the API and Redis:

```sh
# Build and start containers
docker-compose up -d

# View logs
docker-compose logs -f

# Stop containers
docker-compose down
```

### Local Development

If you prefer to run the services separately:

1. Start Redis locally (required)
2. Run the API:
```sh
cd api
cargo run
```c/crowbar.png" width="220px">
</p>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-990000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  </a>
  <a href="https://rocket.rs/">
    <img src="https://img.shields.io/badge/Rocket-bb2b36?style=for-the-badge&logo=rocket&logoColor=white" alt="Rocket"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-aa2b36.svg?style=for-the-badge" alt="License: MIT"/>
  </a>
</p>


<h3 align="center">🛠️ Crowbar</h3>

<p align="center">
  <b>The open-source crowbar for Steam data!</b><br>
  <i>Break open the Steam Community and get real-time info on games, users, inventories, and more.</i>
</p>

# Local Development Setup


## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started.html) (comes with Rust)
- [Node.js](https://nodejs.org/) (if you want to use documentation or frontend)

## Starting the API (backend)

```sh
cd api
cargo run
```

## Starting the documentation (optional)

```sh
cd documentation
npm install
npm start
```

## Project Structure

```
crowbar/
├── api/              # Backend API (Rust/Rocket)
├── documentation/    # API Documentation
└── public/          # Public assets
```

## Environment Variables

The API uses the following environment variables:

- `REDIS_URL`: Redis connection URL (default: `redis://localhost:6379`)
- `ROCKET_PORT`: API port (default: 8000)

## Service Ports

When running with Docker:
- API: http://localhost:8000
- Redis: localhost:6379 (internal)

## Some endpoints to play with:
api/steam/apps
Example: POST crowbar.onrender.com/api/steam/apps
content: application/json
{"query": "Final Fantasy", "cc": "JP"}

/api/steam/app/<appid>
lets you fetch detailed info about any Steam game using its AppID. Just hit /api/steam/app/<appid> and you’ll get the game’s name, images, tags, reviews, player count, and more 
Example: GET crowbar.onrender.com/api/steam/app/730?language=english&CC=US
(Gives you CS2 information)

/api/steam/market/search?appid&query&page
lets you search the steam market for items
Example: GET crowbar.onrender.com/api/steam/market/search?appid=730&query=karambit&page=1

/api/steam/market/search?appid&query&page
fetches information about a specific item
Example: GET crowbar.onrender.com/api/steam/market/item/730?hashname=★%20Karambit%20|%20Ultraviolet%20(Field-Tested)
(you can even get the inspect link from this)


## Development Notes

- The backend runs by default at `localhost:8000` (Rocket)
- For local development, use `cargo run` inside the `api` folder
- To update Rust dependencies: `cargo update`
- To run tests: `cargo test`
- Redis is used for caching Steam API responses

## Troubleshooting

### Docker Issues
- Make sure Docker and Docker Compose are installed and running
- If containers fail to start, check logs with `docker-compose logs`
- To rebuild containers: `docker-compose up -d --build`

### Local Development Issues
- Ensure Redis is running locally when not using Docker
- Check if ports 8000 and 6379 are available
- For Redis connection issues, verify the `REDIS_URL` environment variable

If you have any questions, check the `CONTRIBUTING.md` file or open an issue.

<p align="center">
  <b>Made with ❤️ by the Crowbar community</b>
</p>