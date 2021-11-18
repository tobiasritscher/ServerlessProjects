# first stage base system
FROM messense/rust-musl-cross:x86_64-musl as chef

# using the root for simpler code
USER root

RUN cargo install cargo-chef

WORKDIR /app

# second stage
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
COPY . .

# Precopile all the dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
# Compile actual program
RUN cargo build --release --target x86_64-unknown-linux-musl

# Strip
RUN musl-strip /app/target/x86_64-unknown-linux-musl/release/long-rocket

# final stage
FROM alpine as runtime

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/long-rocket /usr/local/bin/

# Prepare the db folder
RUN addgroup -S iuser && adduser -S iuser -G iuser &&\
	mkdir -p /db &&\
	chown iuser /db

# Required ENV PATHS
ENV ROCKET_ADDRESS="0.0.0.0" \
	ROCKET_PORT=8000 \
	DB_PATH="/db/sql.db"

EXPOSE 8000

USER iuser

CMD [ "long-rocket" ]
