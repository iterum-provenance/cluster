# FROM rust

# COPY src src
# COPY Cargo.toml Cargo.toml

# RUN cargo build --release
# RUN mkdir -p /app
# RUN cp ./target/release/manager /app

# CMD /app/manager


FROM rust

RUN mkdir -p /app/manager_dir
WORKDIR /app/manager_dir
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/manager*
COPY src src
RUN cargo build --release
RUN cp ./target/release/manager /app
CMD /app/manager
# RUN cargo install --path .
# CMD ["/usr/local/cargo/bin/manager"]
