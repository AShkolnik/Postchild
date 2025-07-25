# Postchild

This is the repository for the Hackathon participation on the [boot.dev](https://www.boot.dev) platform. The idea is to write a smaller version of Postman application.
For those not familiar with Postman, to simplify it, it's an application for testing API Endpoints with POST and GET requests and add visualisation to it, store the requests and responses in files and more. Postchild will try to replicate some of the most used features.

The application is written with the Tauri Framework with Svelte for the frontend using TailWindCSS and Rust for the backend.

# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npx sv create

# create a new project in my-app
npx sv create my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
