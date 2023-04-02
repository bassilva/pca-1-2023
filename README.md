## Installing rustup on Linux

Follow the [Installation page](https://www.rust-lang.org/tools/install) instructions:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add `~/.cargo/bin` to your PATH variable.

## Learning Rust

[The Rust Programming Language](https://doc.rust-lang.org/book/) is an online book that covers most of Rust's basic (and maybe even more advanced?) concepts.

## Running with Docker

[Install Docker](https://docs.docker.com/engine/install/) and run the following command from this directory to build the image:

```
docker build . -t rust-dev:latest
```

Execute the command bellow to run your new docker image with the Rust development environment:

```
docker run -d -it --name rust-dev \
   --mount type=bind,source="$(pwd)/src,target=/root/src" \
   --mount type=bind,source="${HOME},target=${HOME},readonly" \
   --name rust-dev \
   rust-dev:latest
```

Now enter in the rust-dev docker:

```
docker exec -it rust-dev sh
```
