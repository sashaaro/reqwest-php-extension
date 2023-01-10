# reqwest php extension

Php extension wrapper under [Reqwest](https://github.com/seanmonstar/reqwest) - powerful Rust HTTP Client.
[Does reqwest support HTTP Multiplexing?](https://github.com/seanmonstar/reqwest/discussions/1470)

Reuse and share tpc connections and multiplex supporting for php-fpm\
Attempt to resolve some problems what described here https://github.com/guzzle/guzzle/issues/1249

```bash

docker run --rm -u $UID -v $PWD:/app -w /app rust:alpine3.16 cargo build

sudo apt-get install musl-tools
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=-crt-static" cargo build --target x86_64-unknown-linux-musl --release
docker run --rm -it -v "$(pwd)":/home/rust/src -e RUSTFLAGS="-C target-feature=-crt-static" messense/rust-musl-cross:x86_64-musl cargo build --release

php -d extension=target/debug/libreqwest.so index.php 

docker build . -t php-fpm:ubuntu

cargo build --release
docker compose up -d
docker compose cp target/x86_64-unknown-linux-musl/release/libreqwest.so php-fpm:/usr/local/lib/php/extensions/no-debug-zts-20210902/
# docker compose cp target/x86_64-unknown-linux-musl/release/libreqwest.so php-fpm:/usr/local/lib/php/extensions/no-debug-zts-20220829/

docker compose exec php-fpm docker-php-ext-enable libreqwest
docker compose exec php-fpm sh
apt-get update
apt-get install libssl-dev
cd /usr/lib/x86_64-linux-gnu
ln -s libssl.so libssl.so.3
ln -s libcrypto.so libcrypto.so.3
php -r "var_dump(extension_loaded('reqwest'));"

docker compose restart php-fpm

docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl
git checkout php-8.2.0
apt install re2c
apt-get install -y pkg-config
make clean
./buildconf --force
./configure --enable-zts --without-sqlite3 --without-pdo-sqlite
make -j4
make install

RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
```
