FROM rust:1.65 as builder

WORKDIR /app
COPY . ./
RUN cargo build --release


FROM debian:stable-20221114
COPY --from=builder /app/target/release/proxmox-backup-server-exporter /
CMD ["./proxmox-backup-server-exporter"]
