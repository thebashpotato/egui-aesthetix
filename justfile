
# Format all files
fmt:
  @cargo fmt --all

# Lint with clippy
lint:
  @cargo clippy --all-targets --all-features -- -D warnings

# Clean the project
clean:
  @cargo clean

# Builds a debug version
build-debug:
  @cargo build

# Builds a release version
build-release:
  @cargo build --release

# Formats, lints and builds debug
dev: fmt lint build-debug

# Formats, lints and builds release
release: fmt lint build-release
