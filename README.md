# container-management
An experimental repo to test out management approaches of docker containers 

## Installation:
See: https://docs.docker.com/engine/install/ubuntu/

## Basic use:

Build image: `docker compose build arg-test`
Run specific image: `docker compose up arg-test`
Run all images: `docker compose up`

### Integration with github actions
- if branch is tagged to match regex: `image_v[0-9]+.[0-9]+.[0-9]+`, start deploy image pipeline, regarding tag naming, see conventions: https://docs.docker.com/engine/reference/commandline/tag/
- look for docker-compose.<systemname> file, matching to tag. Version is placed in dockerfile as label:version:

So to tag and create images from ci, use:
```
    git tag -a image_v0.0.1 -m "example tag to trigger build"
    git push origin <tag_name>
```
Regarding image size etc., docker is excellent at layered pulls, that means it will reuse most of the parts of the images pulled. So at the initial pull alpine is around 700MB for one image, after that it's much smaller.

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

### Pulling a specific tag from docker registry
``` 
    docker login --username username
    docker pull stianlaa/dev-docker-repo:first-tag
```

### Authentication
There might be an alternative to do docker login upon setup. Docker login creates or updates the ~/.docker/config.json file for you.

### Management api
- api to manage services (e.g. for management from web)
- maybe there are node modules which provide the integration out-of-the-box
Node packages to interact with docker:
- https://www.npmjs.com/package/dockerode
- https://www.npmjs.com/package/node-docker-api

### Removing unknown containers, (permission issue)
Removing unknown containers
```
sudo aa-remove-unknown
docker container kill $(docker ps -q)
```

### Stopping running rust apps
```
Rust doesn't automatically handle sigterm signal, docker by default emits sigkill which works after 10 seconds
To emit sigkill with a shorter delay, --time option can be used
docker stop <container> --time=1
```

### Remaining to explore:
- make example app with network use
- make example app with device use
- make example app which uses camera
- make example app with volume use
- make example app which panics every X seconds (test failed container)
- make example app testing logging capability
- make example apps which interact over network
- implementing an equivalent services view, adjust api integration to use one of the modems above
- evalute docker sdk, and potential `nucli` integration