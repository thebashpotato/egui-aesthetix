format:
  @cargo fmt

lint:
  @cargo clippy --all-targets --all-features -- -D warnings

clean:
  @cargo clean

build-debug:
  @cargo build

build-release:
  @cargo build --release

dev: format lint build-debug

release: format lint build-release
