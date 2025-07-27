Postchild
---

This is the repository for Hackathon participation on the [boot.dev](https://www.boot.dev) platform. The idea is to write a smaller version of the Postman application.
It is a Svelte + TypeScript desktop application built with Tauri 2 providing a strong backend with Rust. It is deployed and confirmed to run on Ubuntu 24.04. A release for Ubuntu 24.04 was provided.

Next Features:
- Shortkeys to url input field
- Button for saving http response to clipboard
- History for past requests and load HTTP-Method and URL easily

Download & Run
---

1. Visit the [Releases](https://github.com/AShkolnik/Postchild/releases/tag/hackathon) page.
2. Download the `Postchild-deb.zip` for Ubuntu 24.04.
3. Unzip it
4. Install it:
   ```bash
   dpkg -i PostchildApp_0.1.0_amd64.deb
   ```

Features
---

- REST API request crafting and testing
- Lightweight, cross-platform desktop app (Tauri)
- Svelte-based frontend
- Rust-powered backend

Main Application Interface
---

It provides a way to pass down a specific HTTP Method (GET, PUT, POST, DELETE are supported).

![Main Application](doc/imgs/main.png)

You get the HTTP-Response as plain text in their corresponding tabs.
The other HTTP-Methods are also working fine, with the addition of some Error Messages being displayed as well.

![Malformed URL](doc/imgs/error_malformed_url.png)

Further images can be found in `doc/imgs/`.

Prerequisites
---

Before setting up or running the project locally, you’ll need:

Node.js (v24+ recommended): Download Node.js
npm (comes with Node.js)
Rust (v1.88+ recommended): Install Rust
Tauri CLI (install with cargo install tauri-cli)
Linux (Ubuntu 24.04 recommended)
For other OSes, see Tauri prerequisites and install platform-specific dependencies.

Setup and Development
---

Clone the repository:

```sh
git clone https://github.com/AShkolnik/Postchild.git
cd Postchild
```

Install frontend dependencies:

```sh
npm install
```

Build and run the app in development mode:
```sh
npm run tauri dev
```

This will start both the frontend (Svelte) and the Tauri backend (Rust).
The app should open in a desktop window.

Troubleshooting Notes
---

- glibc compatibility: You must build on Ubuntu 24.04 if that's the target runtime environment. Building on newer systems may break on older ones due to mismatched glibc versions 
- WebKit/WebView: Make sure libwebkit2gtk‑4.1 is installed on both build and target machines.

License
---

[GPL-3.0](LICENSE)
