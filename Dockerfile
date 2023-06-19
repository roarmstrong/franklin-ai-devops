FROM rust:1.70 as builder
WORKDIR /app
COPY hello-world-server/ .
RUN cargo build --all-features -p hello-world-server --release

FROM rust:1.70-slim-bookworm

RUN groupadd -r crustacean && useradd --no-log-init -r -g crustacean crustacean

RUN apt-get update \
	&& apt-get install -y curl \
	&& apt-get upgrade -y \
	&& rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hello-world-server /app/
COPY hello-world-server/static/ /app/static/

HEALTHCHECK CMD curl -f http://localhost/ || exit 1
EXPOSE 80

WORKDIR /app
USER crustacean
CMD ["/app/hello-world-server"]
