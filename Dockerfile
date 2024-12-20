# -- compile time container --

FROM rust:1.83 as build

WORKDIR /usr/src/log-scraper

# build dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src \
    && echo "// dummy file" > ./src/lib.rs \
    && cargo build
RUN rm ./src/lib.rs

# move src files into the container filesystem
COPY ./src ./src

# compile and obtain binaries
RUN cargo install --path .

# RUN cargo doc --document-private-items
COPY ./build ./build


# -- run time container --

FROM debian:latest

RUN apt-get update && \
    apt-get -y upgrade

RUN mkdir -p /usr/src/app

# COPY --from=build /usr/src/log-scraper/target/doc /docs
COPY --from=build /usr/src/log-scraper/build /build

COPY --from=build /usr/local/cargo/bin/log-scraper /usr/src/app/log-scraper

EXPOSE 3333

# default run command for the container
CMD ["/usr/src/app/log-scraper"]

