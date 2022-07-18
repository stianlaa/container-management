FROM rust:1.59-slim
COPY ../target/debug/arg-test ./run-service
ENTRYPOINT ["./run-service", \
"test3", "default-value-3", \
"test4", "default-value-4" \
]