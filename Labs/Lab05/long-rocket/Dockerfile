FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=myip
ENV UID=10001

RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"


WORKDIR /myip

COPY ./src ./src
COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /myip

# Copy our build
COPY --from=builder /myip/target/x86_64-unknown-linux-musl/release/long-rocket ./

# Use an unprivileged user.

USER myip:myip
ENV ROCKET_ADDRESS "0.0.0.0"
ENV ROCKET_PORT 8000
ENV DB_PATH "/db.sqlite"
EXPOSE 8000

CMD ["/myip/long-rocket"]
