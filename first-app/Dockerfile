# Works with:
# sudo docker build -t first-app -f first-app/Dockerfile .
# see: https://stackoverflow.com/questions/27068596/how-to-include-files-outside-of-dockers-build-context


FROM rust:1.59-slim
COPY ./target/debug/first-app ./first-app
CMD ./first-app