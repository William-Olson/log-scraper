# -- compile time container --

FROM rust:1.67 as build

WORKDIR /usr/src/log-scraper

# move src files into the container filesystem
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# compile and obtain binaries
RUN cargo install --path .


# -- run time container --

FROM debian:latest

RUN apt-get update && \
    apt-get -y upgrade

RUN mkdir -p /usr/src/app

COPY --from=build /usr/local/cargo/bin/log-scraper /usr/src/app/log-scraper

EXPOSE 3333

# default run command for the container
CMD ["/usr/src/app/log-scraper"]

