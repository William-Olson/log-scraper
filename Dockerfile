FROM rust:1.67

WORKDIR /usr/src/log-scraper
COPY . .

RUN cargo install --path .

CMD ["log-scraper"]
