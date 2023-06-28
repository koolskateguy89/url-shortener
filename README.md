# URL Shortener

A Simple URL Shortener for exploring Rust and other technologies.

Project structure based on [`spa5k/monorepo-typescript-rust`](https://github.com/spa5k/monorepo-typescript-rust), and my **extremely**
limited knowledge of
[Turborepo](https://turbo.build/repo),
[Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html),
and
[pnpm workspaces](https://pnpm.io/workspaces).

## Apps, Packages and Crates

- `server-actix`: an [Actix Web](https://actix.rs/) web server, with [Shuttle](https://www.shuttle.rs/) infrastructure
- `common`: shared Rust code
- `web-solid-start`: a [Solid-Start](https://start.solidjs.com/) app
- `web-nextjs`: a [Next.js](https://nextjs.org/) app
- `web-yew`: a [Yew](https://yew.rs/) app
- `api`: API to interact with the server
- `ui-core`: 'core' component library, contains styles and variants using [`class-variance-authority`](https://cva.style/)
- `ui-solid`: Solid component library
- `ui-react`: React component library
- `eslint-config-custom`: base [`eslint`](https://eslint.org/) configuration
- `eslint-config-custom-solid`: Solid `eslint` configuration (includes `eslint-plugin-solid`)
- `eslint-config-custom-next`: Next.Js `eslint` configuration (includes `eslint-config-next`)
- `tailwind-config`: shared [Tailwind CSS](https://tailwindcss.com/) configuration
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

TODO?: add /target to turbo build output

## Deployments

| App             | Port (dev) | Deployment                                                                                                                             |
|-----------------|------------|----------------------------------------------------------------------------------------------------------------------------------------|
| server-actix    | 8000       | [https://url-shortener-server-actix.shuttleapp.rs/](https://url-shortener-server-actix.shuttleapp.rs/)                                 |
| web-solid-start | 3000       | [https://url-shortener-solid-start.vercel.app/](https://url-shortener-solid-start.vercel.app/)                                         |
| web-nextjs      | 3001       | [https://url-shortener-nextjs-eight.vercel.app/](https://url-shortener-nextjs-eight.vercel.app/)                                       |
| web-yew         | 3002       | [https://url-shortener-server-actix.shuttleapp.rs/yew/](https://url-shortener-server-actix.shuttleapp.rs/yew/)                         |

## Useful Links

- [Sharing Tailwind Config in monorepo](https://github.com/vercel/turbo/tree/main/examples/with-tailwind)
- [Using Tailwind with Yew & Trunk](https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9)

## App progress

- Actix Server
  - [x] Shorten URL
  - [x] Lengthen URL
  - [ ] Register
  - [...] Sign in
- Solid-Start
  - [x] Home (shorten URL)
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [...] Sign in
- NextJs
  - [x] Home
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [...] Sign in
- Yew
  - [x] Home
  - [x] Redirect
  - [~] Stats
  - [~] URL Error Page
  - [...] Sign in
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
  - more routes coming in the future (e.g. `:id/stats`)

# Turborepo starter

This is an official starter Turborepo.

## Using this example

Run the following command:

```sh
npx create-turbo@latest
```

## What's inside?

This Turborepo includes the following packages/apps:

### Apps and Packages

- `docs`: a [Next.js](https://nextjs.org/) app
- `web`: another [Next.js](https://nextjs.org/) app
- `ui`: a stub React component library shared by both `web` and `docs` applications
- `eslint-config-custom`: `eslint` configurations (includes `eslint-config-next` and `eslint-config-prettier`)
- `tsconfig`: `tsconfig.json`s used throughout the monorepo

Each package/app is 100% [TypeScript](https://www.typescriptlang.org/).

### Utilities

This Turborepo has some additional tools already setup for you:

- [TypeScript](https://www.typescriptlang.org/) for static type checking
- [ESLint](https://eslint.org/) for code linting
- [Prettier](https://prettier.io) for code formatting

### Build

To build all apps and packages, run the following command:

```
cd my-turborepo
pnpm build
```

### Develop

To develop all apps and packages, run the following command:

```
cd my-turborepo
pnpm dev
```

### Remote Caching

Turborepo can use a technique known as [Remote Caching](https://turbo.build/repo/docs/core-concepts/remote-caching) to share cache artifacts across machines, enabling you to share build caches with your team and CI/CD pipelines.

By default, Turborepo will cache locally. To enable Remote Caching you will need an account with Vercel. If you don't have an account you can [create one](https://vercel.com/signup), then enter the following commands:

```
cd my-turborepo
npx turbo login
```

This will authenticate the Turborepo CLI with your [Vercel account](https://vercel.com/docs/concepts/personal-accounts/overview).

Next, you can link your Turborepo to your Remote Cache by running the following command from the root of your Turborepo:

```
npx turbo link
```

## Useful Links

Learn more about the power of Turborepo:

- [Tasks](https://turbo.build/repo/docs/core-concepts/monorepos/running-tasks)
- [Caching](https://turbo.build/repo/docs/core-concepts/caching)
- [Remote Caching](https://turbo.build/repo/docs/core-concepts/remote-caching)
- [Filtering](https://turbo.build/repo/docs/core-concepts/monorepos/filtering)
- [Configuration Options](https://turbo.build/repo/docs/reference/configuration)
- [CLI Usage](https://turbo.build/repo/docs/reference/command-line-reference)
