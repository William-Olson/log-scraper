FROM rust:1.67

WORKDIR /usr/src/log-scraper

# move src files into the container filesystem
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# compile and obtain binaries
RUN cargo install --path .

# default run command for the container
CMD ["log-scraper"]
