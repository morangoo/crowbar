<p align="center">
  <img src="./public/crowbar.png" width="220px">
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

## Notes

- The backend runs by default at `localhost:8000` (Rocket).
- For development, use `cargo run` inside the `api` folder.
- To update Rust dependencies: `cargo update`.
- To run tests: `cargo test`.

If you have any questions, check the `CONTRIBUTING.md` file or open an issue.

<p align="center">
  <b>Made with ❤️ by the Crowbar community</b>
</p>