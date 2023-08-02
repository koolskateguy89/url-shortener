#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

# https://github.com/casey/just

# waiting for https://github.com/shuttle-hq/shuttle/issues/1008 to be resolved
STATIC_FOLDER := "static"
# STATIC_FOLDER := "apps/server-actix/static"

alias up := upgrade-npm-deps
alias sd := shuttle-deploy

# List available recipes
default:
  @just --list --unsorted

# List outdated cargo npm dependencies
outdated:
  cargo outdated --root-deps-only
  -pnpm -r outdated

upgrade-npm-deps:
  pnpm up -r --latest

# Run all apps in development mode
dev:
  pnpm dev --concurrency 20

# Unused Cargo deps - https://github.com/est31/cargo-udeps
udeps:
  cargo +nightly udeps

# TODO: use turbo for tests cos caching
# Build the Actix server static artifacts, running tests first
@build-static:
  @pnpm test --filter=web-yew --filter=yew-query-rs
  @pnpm server:build
  echo Removing static folder \'{{STATIC_FOLDER}}\'
  -rm -r {{STATIC_FOLDER}}
  echo Creating static folder \'{{STATIC_FOLDER}}\'
  mkdir -p {{STATIC_FOLDER}}
  echo Copying Yew static files to \'{{STATIC_FOLDER}}\'
  cp -r apps/web-yew/dist {{STATIC_FOLDER}}/yew

# Build the Actix server artifacts and deploy to Shuttle
@shuttle-deploy: build-static
  @cargo shuttle deploy --allow-dirty

# Run the `url-shortener-cli` with the given arguments (the server needs to be running)
cli *args:
  cargo run --bin url-shortener-cli --quiet -- {{args}}
