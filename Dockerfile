FROM rust:1.70 as builder

WORKDIR /app
COPY . ./
RUN cargo build --release


FROM debian:stable-20230703
COPY --from=builder /app/target/release/proxmox-backup-server-exporter /
CMD ["./proxmox-backup-server-exporter"]
