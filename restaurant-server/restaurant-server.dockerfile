FROM rust:1.69-buster as builder
WORKDIR /app
ENV DB_URL=postgres://postgres:postgres@localhost:5432/postgres
COPY . .
RUN cargo build --release

# Production stage
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/restaurant-server .
CMD ["./restaurant-server"]

#EXPOSE 8080