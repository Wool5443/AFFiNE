# Source-build Docker stack

This stack builds AFFiNE from the current checkout instead of pulling the
published `ghcr.io/toeverything/affine` image.

It builds:

- `@affine/server-native`
- `@affine/web`
- `@affine/admin`
- `@affine/mobile`
- `@affine/server`

It also includes the `affine-workers` submodule and is configured to expose the
app on port `22385` for a reverse proxy such as nginx in front of
`https://affine.twenty5443.duckdns.org`.

## Start

```sh
cp .docker/source-build/.env.example .docker/source-build/.env
mkdir -p data/storage data/config data/postgres/pgdata
docker compose --env-file .docker/source-build/.env -f .docker/source-build/compose.yml up -d --build
```

Data directories are configured through `.docker/source-build/.env`:

- `DB_DATA_LOCATION`
- `UPLOAD_LOCATION`
- `CONFIG_LOCATION`

## Stop

```sh
docker compose --env-file .docker/source-build/.env -f .docker/source-build/compose.yml down
```
