# mullvad-wg-establish-psk

## Building statically linked binaries

In order to build statically linked binaries, a musl toolchain is needed.
First, you'll need to install the following packages, using apt-get or similar:
```
autoconf automake libtool make musl-dev musl-tools libclang-dev
```

You'll also need to add a rust target using rustup. We add it to the stable toolchain, since we are going to build against that:
```
rustup target add x86_64-unknown-linux-musl --toolchain stable
```

Then, you can run the [`build-static.sh`] script to build the static binaries.


[oqs]: https://crates.io/crates/oqs
[`build-static.sh`]: https://github.com/mullvad/oqs-rs/blob/master/mullvad-wg-establish-psk/build-static.sh

License: MIT/Apache-2.0
