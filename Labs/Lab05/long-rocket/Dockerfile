FROM ekidd/rust-musl-builder:stable AS builder

WORKDIR /home/rust

# make changing something in the source code faster
# (it doesn't need to compile everything again)
COPY Cargo.toml .
COPY Cargo.lock .

# a hack to precompile the dependencies
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# compile first step
RUN cargo build --release

COPY src/. src/.

# Compile actual program
RUN cargo build --release

FROM scratch

COPY --from=builder /home/rust/long-rocket/target/x86_64-unknown-linux-musl/release/long-rocket /usr/local/bin/

USER 1000

CMD [ "long-rocket"]
