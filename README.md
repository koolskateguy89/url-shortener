# URL Shortener

A Simple URL Shortener for exploring Rust and other technologies.

Project structure based on [`spa5k/monorepo-typescript-rust`](https://github.com/spa5k/monorepo-typescript-rust), and my **extremely**
limited knowledge of
[Turborepo](https://turbo.build/repo),
[Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
and
[pnpm workspaces](https://pnpm.io/workspaces).

## Apps, Packages and Crates

TODO?: try to [on a non-very-basic level] port React Query to Yew

TODO: `logger` package

- `server-actix`: an [Actix Web](https://actix.rs/) web server, with [Shuttle](https://www.shuttle.rs/) infrastructure
- `common`: shared Rust code
- `yew-query`: a VERY basic port of [Tanstack Query](https://tanstack.com/query/latest/) for [Yew](https://yew.rs/)
- `web-solid-start`: a [Solid-Start](https://start.solidjs.com/) app
- `web-nextjs`: a [Next.js](https://nextjs.org/) app
- `web-yew`: a [Yew](https://yew.rs/) app
- `web-svelte-kit`: a [SvelteKit](https://kit.svelte.dev/) app
- `api`: API to interact with the server
- `ui-core`: 'core' component library, contains styles and variants using [`class-variance-authority`](https://cva.style/)
- `ui-solid`: Solid component library
- `ui-react`: React component library
- `eslint-config-custom`: base [`eslint`](https://eslint.org/) configuration
- `eslint-config-custom-solid`: Solid `eslint` configuration (includes `eslint-plugin-solid`)
- `eslint-config-custom-next`: Next.Js `eslint` configuration (includes `eslint-config-next`)
- `tailwind-config`: shared [Tailwind CSS](https://tailwindcss.com/) configuration
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

## Deployments

| App             | Port (dev) | Deployment                                                                                                                             |
|-----------------|------------|----------------------------------------------------------------------------------------------------------------------------------------|
| server-actix    | 8000       | [https://url-shortener-server-actix.shuttleapp.rs/](https://url-shortener-server-actix.shuttleapp.rs/)                                 |
| web-solid-start | 3000       | [https://url-shortener-solid-start.vercel.app/](https://url-shortener-solid-start.vercel.app/)                                         |
| web-nextjs      | 3001       | [https://url-shortener-nextjs-eight.vercel.app/](https://url-shortener-nextjs-eight.vercel.app/)                                       |
| web-yew         | 3002       | [https://url-shortener-server-actix.shuttleapp.rs/yew/](https://url-shortener-server-actix.shuttleapp.rs/yew/)                         |
| web-svelte-kit  | 5173       | [https://url-shortener-svelte-kit.vercel.app/](https://url-shortener-svelte-kit.vercel.app/)                                           |

## Useful Links

- [Sharing Tailwind Config in monorepo](https://github.com/vercel/turbo/tree/main/examples/with-tailwind)
- [Using Tailwind with Yew & Trunk](https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9)

## App progress

TODO: change to table?

- Actix Server
  - [x] Shorten URL
  - [x] Lengthen URL
  - [x] Register
  - [...] Sign in
- Solid-Start
  - [x] Home (shorten URL)
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [x] Sign in
  - [x] Register
- NextJs
  - [x] Home
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [x] Sign in
  - [x] Register
- Yew
  - [x] Home
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [x] Sign in
  - [ ] Register
- SvelteKit
  - [x] Home
  - [ ] Redirect
  - [ ] Stats
  - [ ] URL Error Page
  - [ ] Sign in
  - [ ] Register
- Tauri
  - [ ] Home
  - [ ] Redirect
  - [ ] Stats

### Roadmap

- [~] Auth
  - [ ] View all my short URLs
    - [ ] Stats
  - [ ] Deletion of short URLs
- [ ] Tauri

## Web Apps

### Routes

- `/` - Home
- `/:id` - Redirect
  - `/:id/stats` - Stats
  - + more in future hopefully
- `/login`
- `/register`

# Turborepo starter

This is an official starter Turborepo.

## Useful Links

Learn more about the power of Turborepo:

- [Tasks](https://turbo.build/repo/docs/core-concepts/monorepos/running-tasks)
- [Caching](https://turbo.build/repo/docs/core-concepts/caching)
- [Remote Caching](https://turbo.build/repo/docs/core-concepts/remote-caching)
- [Filtering](https://turbo.build/repo/docs/core-concepts/monorepos/filtering)
- [Configuration Options](https://turbo.build/repo/docs/reference/configuration)
- [CLI Usage](https://turbo.build/repo/docs/reference/command-line-reference)
