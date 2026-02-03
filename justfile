# Format all files
fmt:
    @cargo fmt --all

# Lint with clippy
lint:
    @cargo clippy --all-targets --all-features -- -D warnings

# Clean the project
clean:
    @cargo clean

# Builds a debug version with all features (themes turned on)
build-debug:
    @cargo build

# Builds a release version
build-release:
    @cargo build --release

# Publish crate on dry run
publish:
    cargo publish --all-features --dry-run

# Formats, lints and builds debug
dev: fmt lint build-debug

# Formats, lints and builds release
release: fmt lint build-release
