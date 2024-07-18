FROM node:lts-alpine3.20 AS builder

WORKDIR /wd
COPY package*.json .
RUN npm ci
COPY . .
RUN ls -lah && npm run build
RUN npm prune --production

FROM node:lts-alpine3.20

ARG version=unknown
ARG release=unreleased

LABEL name="Happy moustaches" \
    maintainer="froalexandre@gmail.com" \
    vendor="Moustaches & Cie" \
    version=${version} \
    release=${release} \
    description="Frontend of the Happy moustaches management app" 

WORKDIR /app

COPY --from=builder /wd/build build/
COPY --from=builder /wd/node_modules node_modules/
COPY package.json .
COPY package-lock.json .

ENV NODE_ENV production
ENV PORT 8080
CMD ["node", "build"]