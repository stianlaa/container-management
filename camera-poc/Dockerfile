FROM rust:1.59-slim
COPY ../target/debug/camera-poc ./run-service
ENTRYPOINT ["./run-service", \
"test1", "default-value-1", \
"test2", "default-value-2" \
]