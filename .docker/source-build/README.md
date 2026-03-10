# Source-build Docker stack

This stack builds AFFiNE from the current checkout instead of pulling the
published `ghcr.io/toeverything/affine` image.

It builds:

- `@affine/server-native`
- `@affine/web`
- `@affine/admin`
- `@affine/mobile`
- `@affine/server`

It also includes the `affine-workers` submodule and exposes the app at
`http://localhost:8080`.

## Start

```sh
cp .docker/source-build/.env.example .docker/source-build/.env
mkdir -p .local/affine-source/storage .local/affine-source/config .local/affine-source/postgres
docker compose --env-file .docker/source-build/.env -f .docker/source-build/compose.yml up -d --build
```

## Stop

```sh
docker compose --env-file .docker/source-build/.env -f .docker/source-build/compose.yml down
```
