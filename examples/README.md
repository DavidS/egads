
```
$ docker run -p 4317:4317 -v $(pwd)/config.yaml:/etc/otelcol/config.yaml -v $(pwd)/config.yaml:/etc/otel/config.yaml otel/opentelemetry-collector:latest
```

then

```
cargo run --example main
```

from the root directory.