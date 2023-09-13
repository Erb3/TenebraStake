FROM rust:1.71-slim AS build
WORKDIR /src
COPY . .
RUN apt update && apt install -y libssl-dev pkg-config ca-certificates
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /app
RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/tenebra-stake /usr/local/bin/tenebra-stake
CMD ["tenebra-stake"]