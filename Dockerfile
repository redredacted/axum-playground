FROM docker.io/rust:1.66
COPY . .
RUN cargo build --release
CMD ["./target/release/axum-playground"]
EXPOSE 8080
