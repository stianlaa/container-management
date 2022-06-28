# Works with:
# sudo docker build -t second-app -f second-app/Dockerfile .
# see: https://stackoverflow.com/questions/27068596/how-to-include-files-outside-of-dockers-build-context


FROM rust:1.59-slim
COPY ./target/debug/second-app ./run-service
CMD ./run-service