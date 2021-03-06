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
COPY --from=planner /app/recipe.json recipe.json

# Precopile all the dependencies -> This cashes the docker layer
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Compile actual program
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Strip
RUN musl-strip /app/target/x86_64-unknown-linux-musl/release/long-rocket

# final stage
FROM alpine as runtime

# Copy the now builded file from the build image to q
# the current small one
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/long-rocket /usr/local/bin/

# Prepare the db folder
# Prepare the internally used user
# Add curl so that the healthcheck will work
RUN apk add --no-cache curl &&\
	addgroup -S iuser &&\ 
	adduser -S iuser -G iuser &&\
	mkdir -p /db &&\
	chown iuser /db

USER iuser

# Required ENV PATHS
# define the needed internal port
ARG DB_PORT=8000
ENV ROCKET_ADDRESS="0.0.0.0" \
	ROCKET_PORT=${DB_PORT} \
	DB_PATH="/db/sql.db"

# EXPOSE the internally used port to the outside world
EXPOSE ${DB_PORT}

# healthcheck based on https://scoutapm.com/blog/how-to-use-docker-healthcheck
HEALTHCHECK --interval=5s --timeout=3s CMD curl --fail http://$ROCKET_ADDRESS:$ROCKET_PORT || exit 1

CMD [ "long-rocket" ]
