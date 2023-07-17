FROM rust:1.71 as builder

WORKDIR /app
COPY . ./
RUN cargo build --release


FROM debian:stable-20230522
COPY --from=builder /app/target/release/proxmox-backup-server-exporter /
CMD ["./proxmox-backup-server-exporter"]
