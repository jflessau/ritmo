FROM rust:latest as build
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

FROM nginx:1.27
COPY ./nginx.conf /etc/nginx/nginx.conf
COPY --from=build /dist/ /var/www/
