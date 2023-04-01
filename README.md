## Running from Docker

[Install Docker](https://docs.docker.com/engine/install/) and run the following command from this directory to build the image:

```
docker build . -t rust-dev:latest
```

Execute the command bellow to run your new docker image with the Rust development environment:

```
docker run -d -it --name rust-dev \
   --mount type=bind,source="$(pwd)/src,target=/root/src" \
   --name rust-dev \
   rust-dev:latest
```

Now enter in the rust-dev docker:

```
docker exec -it rust-dev sh
```
