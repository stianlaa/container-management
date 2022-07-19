# Works with:
# sudo docker build -t rocket-poc -f rocket-poc/Dockerfile .
# see: https://stackoverflow.com/questions/27068596/how-to-include-files-outside-of-dockers-build-context


FROM rust:1.59-slim
COPY ../target/debug/rocket-poc ./run-service
ENTRYPOINT ["./run-service", \
"test1", "default-value-1", \
"test2", "default-value-2" \
]