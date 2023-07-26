# url-shortener-cli

A CLI version of the URL Shortener. Uses the [same API](/apps/server-actix/) as the web apps.

Build using [Clap](https://github.com/clap-rs/clap).

> Note in local development, you can replace `url-shortener-cli` with `cargo run --` in the examples below.

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
- plain text

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
