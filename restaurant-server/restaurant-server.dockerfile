FROM alpine:latest
RUN mkdir /app
ARG DB_URL=postgres://postgres:postgres@localhost:5432/postgres
COPY ./target/release/restaurant-server /app
CMD ["./app/restaurant-server"]