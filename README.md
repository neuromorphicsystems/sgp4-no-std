# sgp4-no-std

This repository compiles a simple SGP4 example (see _src/main.rs_) in no-std mode to illustrate the use of the library in no-std environments. No-std is typically required on microcontrollers that do not have a full operating system. On machines that do have an operating system, no-std can help reduce the compiled program size at the cost of portability, program safety, and code simplicity.

Only Linux x86-64 is supported, but Docker can be used to run this on a different operating system.

Tested on a x86-64 host with a Docker container based on nightly 2024-05-26. This may fail on an ARM host if the default Docker machine is ARM as well. Future Rust versions may break the program as well.

## Build and run

Run the commands in _Dockerfile_ to build the program without Docker.

```sh
docker build . -t sgp4
docker run -t sgp4 './target/release/main'
```

## Debug

Run the program with gdb to catch segfaults.

```sh
docker run -it sgp4
gdb ./target/release/main
run
```

Extract the executable from the container. A disassembler such as https://cutter.re can be used to inspect the compiled binary.

```sh
docker cp $(docker ps -n 1 --format json | jq -r '.ID'):/sgp4/target/release/main main
```

## Resources

-   https://fasterthanli.me/series/making-our-own-executable-packer/part-12
-   https://darkcoding.net/software/a-very-small-rust-binary-indeed/
-   https://github.com/johnthagen/min-sized-rust
