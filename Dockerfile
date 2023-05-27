FROM rust:1.69 as builder

WORKDIR /app
COPY . ./
RUN cargo build --release


FROM debian:stable-20230502
COPY --from=builder /app/target/release/proxmox-backup-server-exporter /
CMD ["./proxmox-backup-server-exporter"]
