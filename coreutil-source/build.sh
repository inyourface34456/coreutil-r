rm -f /home/runner/corutils-r/coreutil-bin/*
rustup default nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release --workspace
find /home/runner/corutils-r/coreutil-source/target/x86_64-unknown-linux-gnu/release -maxdepth 1 -executable -type f -exec mv {} /home/runner/corutils-r/coreutil-bin \;
mv /home/runner/coreutils-r/coreutil-bin/base64_ /home/runner/coreutils-r/coreutil-bin/base64
mv /home/runner/coreutils-r/coreutil-bin/base32_ /home/runner/coreutils-r/coreutil-bin/base32
zip -r coreutil-r.zip /home/runner/corutils-r -i /home/runner/corutils-r/coreutil-bin/*
mv coreutil-r.zip /home/runner/corutils-r