ARG TARGET=x86_64-unknown-linux-musl
ARG FRONTEND_DIR=/app/frontend

FROM node:24-alpine AS frontend-builder

WORKDIR /app/frontend

COPY frontend/package.json ./
COPY package-lock.json package.json ../

RUN \
  --mount=type=cache,target=/app/node_modules,sharing=locked \
  --mount=type=cache,target=/app/frontend/node_modules,sharing=locked \
  npm ci

COPY frontend/svelte.config.js frontend/tsconfig.json frontend/vite.config.ts ./
COPY frontend/src ./src
COPY frontend/static ./static

RUN \
  --mount=type=cache,target=/app/node_modules,sharing=locked \
  --mount=type=cache,target=/app/frontend/node_modules,sharing=locked \
  npm run build

FROM ghcr.io/profiidev/images/rust-musl-builder:main AS backend-planner

COPY backend/Cargo.toml backend/
COPY backend/entity/Cargo.toml backend/entity/
COPY backend/migration/Cargo.toml backend/migration/
COPY ./Cargo.lock ./Cargo.toml ./

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo chef prepare --recipe-path recipe.json --bin backend

FROM ghcr.io/profiidev/images/rust-musl-builder:main AS backend-builder

ARG TARGET
ARG FRONTEND_DIR

COPY --from=backend-planner /app/recipe.json .

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo chef cook --release --target $TARGET

COPY backend/Cargo.toml backend/
COPY backend/build.rs backend/
COPY backend/src backend/src
COPY backend/entity/Cargo.toml backend/entity/
COPY backend/entity/src backend/entity/src
COPY backend/migration/Cargo.toml backend/migration/
COPY backend/migration/src backend/migration/src
COPY ./Cargo.lock ./Cargo.toml ./

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cd backend && cargo build --release --target $TARGET \
  && mv ../target/$TARGET/release/backend ../app

FROM node:24-alpine

ARG FRONTEND_DIR
ENV STORAGE_PATH=/data

COPY --from=backend-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

WORKDIR /app
COPY --from=frontend-builder /app/frontend/build /app/frontend
COPY --from=frontend-builder /app/frontend/package.json /app/frontend/package.json
COPY --from=frontend-builder /app/package-lock.json /app/package-lock.json
COPY --from=backend-builder /app/app /usr/local/bin/sagittarius

CMD ["sagittarius"]