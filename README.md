## promwatch

Watch over Prometheus TSDB API

```
cargo run -- --config ./src/promwatch-config.yaml

curl localhost:8080/metrics
# HELP status TSDB Status.
# TYPE status gauge
status{instance="prometheus-1",name="status"} 0
# HELP series_count_by_metric_name Series Count by Metric Name.
# TYPE series_count_by_metric_name gauge
series_count_by_metric_name{instance="prometheus-1",name="container_tasks_state"} 210
series_count_by_metric_name{instance="prometheus-1",name="grafana_feature_toggles_info"} 179
series_count_by_metric_name{instance="prometheus-1",name="prometheus_http_response_size_bytes_bucket"} 351
series_count_by_metric_name{instance="prometheus-1",name="node_systemd_unit_state"} 970
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_request_duration_seconds_bucket"} 576
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_response_duration_seconds_bucket"} 516
series_count_by_metric_name{instance="prometheus-1",name="grafana_http_request_duration_seconds_bucket"} 702
series_count_by_metric_name{instance="prometheus-1",name="prometheus_http_request_duration_seconds_bucket"} 390
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_request_size_bytes_bucket"} 432
series_count_by_metric_name{instance="prometheus-1",name="caddy_http_response_size_bytes_bucket"} 432
# HELP series_count_by_label_name Series Count by Metric Name.
# TYPE series_count_by_label_name gauge
series_count_by_label_name{instance="prometheus-1",name="state"} 18
series_count_by_label_name{instance="prometheus-1",name="handler"} 96
series_count_by_label_name{instance="prometheus-1",name="id"} 43
series_count_by_label_name{instance="prometheus-1",name="device"} 19
series_count_by_label_name{instance="prometheus-1",name="le"} 217
series_count_by_label_name{instance="prometheus-1",name="method"} 41
series_count_by_label_name{instance="prometheus-1",name="collector"} 47
series_count_by_label_name{instance="prometheus-1",name="code"} 22
series_count_by_label_name{instance="prometheus-1",name="name"} 377
series_count_by_label_name{instance="prometheus-1",name="__name__"} 1099
# HELP memory_in_bytes_by_label_name Memory in Bytes by Label Name.
# TYPE memory_in_bytes_by_label_name gauge
memory_in_bytes_by_label_name{instance="prometheus-1",name="id"} 45176
memory_in_bytes_by_label_name{instance="prometheus-1",name="method"} 14234
memory_in_bytes_by_label_name{instance="prometheus-1",name="le"} 22551
memory_in_bytes_by_label_name{instance="prometheus-1",name="__name__"} 379444
memory_in_bytes_by_label_name{instance="prometheus-1",name="instance"} 237904
memory_in_bytes_by_label_name{instance="prometheus-1",name="name"} 26635
memory_in_bytes_by_label_name{instance="prometheus-1",name="env"} 14236
memory_in_bytes_by_label_name{instance="prometheus-1",name="job"} 71294
memory_in_bytes_by_label_name{instance="prometheus-1",name="handler"} 50471
memory_in_bytes_by_label_name{instance="prometheus-1",name="state"} 10349
# EOF
```
