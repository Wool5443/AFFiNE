# syntax=docker/dockerfile:1.7

FROM node:22-bookworm AS build

WORKDIR /repo

RUN apt-get update && \
  apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    openssl \
    pkg-config \
    python3 && \
  rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
RUN corepack enable

COPY . /repo

RUN yarn install
RUN yarn affine server-native build
RUN cp /repo/packages/backend/native/server-native.node /repo/packages/backend/native/server-native.x64.node
RUN cp /repo/packages/backend/native/server-native.node /repo/packages/backend/native/server-native.arm64.node
RUN cp /repo/packages/backend/native/server-native.node /repo/packages/backend/native/server-native.armv7.node
RUN yarn affine web build
RUN yarn affine admin build
RUN yarn affine mobile build
RUN yarn affine server build
RUN yarn affine server prisma generate
RUN mv /repo/node_modules /repo/packages/backend/server/node_modules

FROM node:22-bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && \
  apt-get install -y --no-install-recommends openssl libjemalloc2 && \
  rm -rf /var/lib/apt/lists/*

ENV LD_PRELOAD=libjemalloc.so.2

COPY --from=build /repo/packages/backend/server /app
COPY --from=build /repo/packages/frontend/apps/web/dist /app/static
COPY --from=build /repo/packages/frontend/admin/dist /app/static/admin
COPY --from=build /repo/packages/frontend/apps/mobile/dist /app/static/mobile

ARG TARGETARCH
ARG TARGETVARIANT
RUN AFFINE_DOCKER_CLEAN=1 TARGETARCH="${TARGETARCH}" TARGETVARIANT="${TARGETVARIANT}" \
  node ./scripts/docker-clean.mjs

EXPOSE 3010

CMD ["node", "./dist/main.js"]
