FROM rust:latest AS builder
RUN update-ca-certificates
ENV USER=rebalancer
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /rebalancer
COPY ./ .
RUN cargo build --release


FROM debian:stable-slim
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
WORKDIR /rebalancer
COPY --from=builder /rebalancer/target/release/rebalancer ./
USER rebalancer:rebalancer
CMD ["/rebalancer/rebalancer"]