FROM rustlang/rust:nightly

RUN apt update
RUN apt install gdb -y

WORKDIR /sgp4

# Compile the dependencies first
# The flags required to build the executable break libm's build.rs script,
# hence the two-step compilation
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN printf "fn main() {}" > src/main.rs
RUN cargo install cargo-build-deps
RUN RUSTFLAGS="-Z location-detail=none -C target-cpu=native" cargo build-deps --release

# Build main.rs by directly calling rustc
COPY . .
SHELL ["/bin/bash", "-c"]
RUN command='rustc --edition=2021 src/main.rs --out-dir target/release \
    -Z location-detail=none \
    -L dependency=target/release/deps \
    -C panic=abort \
    -C target-cpu=native \
    -C link-args=-nostartfiles \
    -C opt-level=z \
    -C lto \
    -C strip=symbols \
    -C link-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,--build-id=none \
    --cfg feature="libm"'; \
libraries=('sgp4' 'num_traits' 'libm' 'compiler_builtins'); \
for library in "${libraries[@]}"; do \
    library_path=$(ls target/release/deps/lib$library-*.rlib); \
    command="${command} --extern $library=${library_path}"; \
done; \
$command
