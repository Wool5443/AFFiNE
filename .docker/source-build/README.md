# AFFiNE: локальная сборка и запуск на сервере

Нужны Docker на компьютере и Docker Compose на сервере.
Команды ниже рассчитаны на сервер Linux x86-64.
Замени `root@server.example.com` на свой SSH-адрес.

## 1. Собрать локально

Из корня репозитория после изменения или обновления кода:

```bash
git submodule update --init --recursive
docker build --platform linux/amd64 \
  --build-arg GITHUB_SHA="$(git rev-parse HEAD)" \
  -f .docker/source-build/Dockerfile -t affine-local:release .
docker build --platform linux/amd64 \
  -t affine-worker-local:release affine-workers
```

Первый образ содержит сервер, web, admin и mobile. Второй — worker.
Node.js, Yarn и Rust устанавливаются внутри Docker.

## 2. Отправить образы на сервер

На своём компьютере:

```bash
set -o pipefail
docker save affine-local:release affine-worker-local:release \
  | gzip -1 \
  | ssh root@server.example.com 'gunzip | docker load'
```

Дождись успешного завершения и двух сообщений `Loaded image`.

## 3. Настроить сервер при первом запуске

На своём компьютере:

```bash
ssh root@server.example.com 'mkdir -p /opt/affine'
scp .docker/source-build/compose.yml .docker/source-build/.env.example \
  root@server.example.com:/opt/affine/
```

На сервере:

```bash
cd /opt/affine
cp -n .env.example .env
nano .env
docker compose up -d --no-build
```

В `.env` укажи свой домен, пароль базы и пути к данным.
Для существующей установки сохрани текущий `.env`, пароль и пути к данным.
Redis и PostgreSQL скачиваются автоматически. Миграции запускаются перед
приложением. Настрой HTTPS-прокси на порт `22385` с поддержкой WebSocket.

## 4. Обновить работающий сервер

Повтори шаги 1 и 2. На сервере, после резервного копирования базы и данных:

```bash
cd /opt/affine
docker compose up -d --no-build --pull never \
  --force-recreate affine_migration affine worker
docker compose ps -a
docker compose logs --tail=50 affine_migration affine
curl -fsS https://affine.example.com/info
```

У миграции должен быть статус `Exited (0)`, у приложения — `Up`.
Исходники на сервере не нужны; `.env` и данные сохраняются.
