FROM rust:latest as build
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

FROM nginx:1.27
COPY ./nginx.conf /etc/nginx/nginx.conf
COPY --from=build /dist/ /var/www/
