ARG THE_USER=internal \ 
    IPORT=8000

## first stage base system
FROM messense/rust-musl-cross:x86_64-musl as chef
ARG THE_USER \ 
    IPORT    

# using the root user so that we can install stuff
USER root

RUN cargo install cargo-chef

WORKDIR /app

## second stage
FROM chef as planner
ARG THE_USER \ 
    IPORT    

# Prepare the recipe with that we can create the cash layer
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

## third stage
FROM chef as builder
ARG THE_USER \ 
    IPORT    

# Create appuser

ENV USER=${THE_USER}
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

# download build deps
RUN apt-get update && apt-get install upx -y

COPY --from=planner /app/recipe.json recipe.json

# Precopile all the dependencies -> This cashes the docker layer
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Compile actual program
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Strip
RUN musl-strip /app/target/x86_64-unknown-linux-musl/release/webhook

# Compress executable 
RUN upx --best --ultra-brute  /app/target/x86_64-unknown-linux-musl/release/webhook

## final (fourth) stage
FROM scratch as runtime
ARG THE_USER \ 
    IPORT    

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy the now builded file from the build image to 
# the current small one
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/webhook /usr/local/bin/

# Use an unprivileged user.
USER "${THE_USER}:${THE_USER}"

# Required ENV PATHS
# define the needed internal port
ENV ADDRESS="0.0.0.0" \
	PORT=${IPORT}

# EXPOSE the internally used port to the outside world
EXPOSE ${IPORT}

# healthcheck based on https://scoutapm.com/blog/how-to-use-docker-healthcheck
HEALTHCHECK --interval=5s --timeout=3s CMD ["webhook", "--healthcheck"]

CMD [ "webhook" ]

