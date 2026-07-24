FROM oven/bun:alpine AS base
WORKDIR /usr/src/app

FROM base AS builder
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

COPY . .
ENV NODE_ENV=production
RUN bun run build

FROM base AS production-deps
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile --production

FROM oven/bun:alpine AS release
WORKDIR /usr/src/app

COPY --from=production-deps --chown=bun:bun /usr/src/app/node_modules ./node_modules
COPY --from=builder --chown=bun:bun /usr/src/app/build ./build
COPY --from=builder --chown=bun:bun /usr/src/app/package.json ./package.json

USER bun
EXPOSE 3000/tcp

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
  CMD wget -q --spider http://127.0.0.1:3000/ || exit 1

ENTRYPOINT [ "bun", "start" ]
