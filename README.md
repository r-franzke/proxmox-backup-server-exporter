# Proxmox Backup Server Exporter
A small script that talks to the API of the Proxmox Backup Server (PBS) to generate metrics prometheus metrics.

Can be started with this command:
```
docker run -p 9898:9898 -e BASE_URL='https://<your-pbs-ip>:8007/api2/json/' -e TOKEN_ID='<your-token-id>' -e TOKEN_SECRET='<your-token-secret>' robertfranzke/proxmox-backup-server-exporter:latest
```

## Metrics
These metrics will be exported:
| Name                          | Description                                                                        |
| ----------------------------- | ---------------------------------------------------------------------------------- |
| pbs_datastore_available_bytes | Number of available bytes in the datastore.                                        |
| pbs_datastore_estimated_full  | Time and Date when the datastore will be estimated full as a Epoch Unix Timestamp. |
| pbs_datastore_total_bytes     | Number of total bytes in the datastore.                                            |
| pbs_datastore_used_bytes      | Number of used bytes in the datastore.                                             |
