# url-shortener-cli

A CLI version of the URL Shortener. Uses the [same API](/apps/server-actix/) as the web apps.

Built using [Clap](https://github.com/clap-rs/clap).

> Note in local development, you can replace `url-shortener-cli` with `cargo run --` or `just cli` in the examples below.

The server must be running for the CLI to work.

The server url is set in the `URL_SHORTENER_API_URL` environment variable (it needs to include `/api). Defaults to `http://localhost:8080/api`.

## List

table format?

```sh
$ url-shortener-cli list
<ID> <URL>
<ID> <URL>
...
```

## Shorten

```sh
$ url-shortener-cli shorten <URL>
<ID>
```

## Lengthen

```sh
$ url-shortener-cli lengthen <ID>
<URL>
```

## Stats

```sh
$ url-shortener-cli stats <ID>
```

### Format

- json
- pretty json
- plain text
- rust debug (`StatsResponse`)
- rust pretty debug (`StatsResponse`)

```sh
$ url-shortener-cli stats <ID> --format <FORMAT>

or

$ url-shortener-cli stats <ID> --json
```

## Claim

TODO: auth - what is best way?

```sh
$ url-shortener-cli claim <ID>
```
