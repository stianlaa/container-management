FROM rust:1.59-slim
ENV ROCKET_ADDRESS=0.0.0.0
COPY ../target/debug/manager-api ./run-service
COPY ../compose.yml ./compose.yml
ENTRYPOINT ["./run-service"]