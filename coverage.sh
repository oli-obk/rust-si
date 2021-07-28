


rustup default nightly
cargo install grcov
rustup component add llvm-tools-preview

rm -rf ./target *.prof*

# Export the flags needed to instrument the program to collect code coverage.
export RUSTFLAGS="-Zinstrument-coverage"

# Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable (%p will be replaced by the process ID, and %m by the binary signature):
export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"

# Build the program
cargo build

# Run the program (you can replace this with `cargo test` if you want to collect code coverage for your tests).
cargo test

# Generate a HTML report in the coverage/ directory.
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/

# Generate a LCOV report and upload it to codecov.io.
grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing -o ./lcov.info
bash <(curl -s https://codecov.io/bash) -f lcov.info