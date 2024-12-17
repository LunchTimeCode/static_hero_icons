


# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------  

run:
    cargo run

# Run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt


publish:
    cargo publish --token $GLOBAL_CARGO_TOKEN


d:
    curl https://codeload.github.com/tailwindlabs/heroicons/tar.gz/master | \
    tar -xz --strip=2 heroicons-master/optimized