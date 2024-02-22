
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
  @cargo build --all-features

# Builds a release version
build-release:
  @cargo build  --all-features --release

# Publish crate on dry run
publish:
    cargo publish --all-features --dry-run

# Formats, lints and builds debug
dev: fmt lint build-debug

# Formats, lints and builds release
release: fmt lint build-release

