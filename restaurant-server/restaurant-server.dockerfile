FROM rust:1.72-buster as builder
RUN mkdir /app
ARG DB_URL=postgres://postgres:postgres@localhost:5432/postgres
#COPY ./target/release/restaurant-server /app
COPY . .
#CMD ["./app/restaurant-server"]
CMD ["tail", "-f", "/dev/null"]