FROM rust:1.59-slim
# Create picture volume, and any images within ./assets

# Mount types: volume og binded mount, see: https://docs.docker.com/storage/
# binded mount can't be added in Dockerfile because it reduces portability

VOLUME /result-picture-dir
RUN mkdir -p picture-dir
COPY volume-poc/picture-dir/* /picture-dir

COPY ../target/debug/volume-poc ./run-service
ENTRYPOINT ["./run-service"]