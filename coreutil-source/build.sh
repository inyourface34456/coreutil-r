rm -f /home/runner/corutils-r/coreutil-bin/*
cargo build --workspace -r
find /home/runner/corutils-r/coreutil-source/target/release -maxdepth 1 -executable -type f -exec mv {} /home/runner/corutils-r/coreutil-bin \;
zip -r coreutil-r.zip /home/runner/corutils-r -i /home/runner/corutils-r/coreutil-bin/*
mv coreutil-r.zip /home/runner/corutils-r