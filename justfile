#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

# https://github.com/casey/just

# waiting for https://github.com/shuttle-hq/shuttle/issues/1008 to be resolved
STATIC_FOLDER := "static"
# STATIC_FOLDER := "apps/server-actix/static"

alias up := upgrade-npm-deps

# List available recipes
default:
  @just --list --unsorted

# TODO: make this a git alias instead
uai:
  git update-index --again

outdated:
  -pnpm -r outdated

upgrade-npm-deps:
  pnpm up -r --latest

dev:
  pnpm dev --concurrency 15

# Build the Actix server artifacts and deploy to Shuttle
@deploy-shuttle:
  @pnpm server:build
  echo Removing static folder \'{{STATIC_FOLDER}}\'
  -rm -r {{STATIC_FOLDER}}
  echo Creating static folder \'{{STATIC_FOLDER}}\'
  mkdir -p {{STATIC_FOLDER}}
  echo Copying Yew static files to \'{{STATIC_FOLDER}}\'
  cp -r apps/web-yew/dist {{STATIC_FOLDER}}/yew
  @cargo shuttle deploy --allow-dirty
