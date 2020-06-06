# Build the app
FROM node:14.4.0-alpine3.11 AS builder

WORKDIR /multiverse

COPY package.json yarn.lock tsconfig.json /multiverse/

RUN yarn install

COPY public /multiverse/public/
COPY src /multiverse/src/

RUN yarn build

# Create deployable image
FROM beamaustralia/react-env:latest

WORKDIR /var/www

COPY --from=builder /multiverse/build /var/www

ENV REACT_APP_URL_BASE=NOT_SPECIFIED