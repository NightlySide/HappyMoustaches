FROM node:slim AS builder

WORKDIR /wd
COPY . /wd
RUN npm run build

FROM node:slim

ARG version=unknown
ARG release=unreleased

LABEL name="Happy moustaches" \
    maintainer="froalexandre@gmail.com" \
    vendor="Moustaches & Cie" \
    version=${version} \
    release=${release} \
    description="Frontend of the Happy moustaches management app" 

WORKDIR /app

COPY --from=builder /wd/build /app/build
COPY --from=builder /wd/package.json /app/
COPY --from=builder /wd/package-lock.json /app/
RUN npm ci --omit dev

ENV PORT 8080
CMD ["node", "build"]