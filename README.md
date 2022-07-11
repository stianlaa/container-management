# container-management
An experimental repo to test out management approaches of docker containers 

## Installation:
See: https://docs.docker.com/engine/install/ubuntu/

## Use:

Build image: docker build -t first-app -f first-app/Dockerfile .
Run specific image: docker run first-app
Run all images: docker compose up

## Aspects to investigate:

- creation of docker images
- providing images with dependencies
- sharing of docker images with tags
- docker registry, and consequence of image size. (alpine is around 700MB)
- api to manage services (e.g for management from web)


- feeding docker images with arguments, updating arguments
https://docs.docker.com/engine/reference/builder/
 The main purpose of a CMD is to provide defaults for an executing container. These defaults can include an executable, or they can omit the executable, in which case you must specify an ENTRYPOINT instruction as well.

excellent details on iteraction between CMD and ENTRYPOINT: https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact
