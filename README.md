# container-management
An experimental repo to test out management approaches of docker containers 

## Installation:
See: https://docs.docker.com/engine/install/ubuntu/

## Use:

Build image: docker build -t first-app -f first-app/Dockerfile .
Run specific image: docker run first-app
Run all images: docker compose up

### Writing the Dockerfile properly

- feeding docker images with arguments, updating arguments
https://docs.docker.com/engine/reference/builder/
 The main purpose of a CMD is to provide defaults for an executing container. These defaults can include an executable, or they can omit the executable, in which case you must specify an ENTRYPOINT instruction as well.

excellent details on iteraction between CMD and ENTRYPOINT: https://docs.docker.com/engine/reference/builder/#understand-how-cmd-and-entrypoint-interact

### Integration with github actions
- Github actions has nice Dockerfile build support, but it is mainly streamlined for creating a single image per build.
- Could consider build where only one image is built and published, to avoid unnecessary bumps to various version.
- Ideal:
  - Alternative 1: Dockerfile or docker-compose has a version, which is checked. If it has been bumped, then a new image is built and published
  - Alternative 2: Upon pushing code to a tagged branch, perhaps with a pattern, e.g `image-<feature-name>`, all images are created with that tag name or description added
  - Alternative 3: A local script which can be run to build and update image
  - Alternative 4: An integration with Clion docker plugin?

Given alternative 2, using tagged branch:
 - if branch is tagged create:systemname, start deploy image pipeline
 - look for docker-compose.<systemname> file, matching to tag. Version is placed in dockerfile as label:version:

### Pushing a local repository to docker registry manually
```
    docker login --username username
    docker tag my-image username/my-repo
    docker push username/my-repo
```

### Pushing a specific image to docker registry manually
```
    docker build -t appname -f dockerfilepath/Dockerfile .
    docker login --username username
    docker tag appname username/registryname:tagname
    docker push username/registryname:tagname
```

### Pulling a specific tag from docker registry:
``` 
docker pull stianlaa/dev-docker-repo:first-tag
```
So to specify a specific app image, and allow for updates, a decent convention might be `{appname-1.2.3}`, or `{appname__1.2.3}`, see conventions: https://docs.docker.com/engine/reference/commandline/tag/

Regarding image size etc., docker is excellent at layered pulls, that means it will reuse most of the parts of the images pulled. So at the initial pull alpine is around 700MB for one image, after that its much smaller.

### Authentication
It might be an alternative to do docker login upon setup. Docker login creates or updates the ~/.docker/config.json file for you.

## Remaining Aspects to investigate:
- building and pushing images from ci
- api to manage services (e.g. for management from web)
