FROM rust:1.59-slim
ENV ROCKET_ADDRESS=0.0.0.0
COPY ../target/debug/rocket-poc ./run-service
ENTRYPOINT ["./run-service"]