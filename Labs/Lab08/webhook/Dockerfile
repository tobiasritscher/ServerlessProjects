# first stage base system
FROM messense/rust-musl-cross:x86_64-musl as chef

# using the root user so that we can install stuff
USER root

RUN cargo install cargo-chef

WORKDIR /app

# second stage
FROM chef as planner


# Prepare the recipe with that we can create the cash layer
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

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

# final stage
FROM alpine as runtime

# Copy the now builded file from the build image to 
# the current small one
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/webhook /usr/local/bin/

# Prepare the db folder
# Prepare the internally used user
# Add curl so that the healthcheck will work
RUN addgroup -S iuser &&\ 
	adduser -S iuser -G iuser

USER iuser

# Required ENV PATHS
# define the needed internal port
ARG IPORT=8000
ENV ADDRESS="0.0.0.0" \
	PORT=${IPORT}

# EXPOSE the internally used port to the outside world
EXPOSE ${IPORT}

# healthcheck based on https://scoutapm.com/blog/how-to-use-docker-healthcheck
HEALTHCHECK --interval=5s --timeout=3s CMD webhook --healthcheck

CMD [ "webhook" ]

