# reqwest php extension

Php extension wrapper under [Reqwest](https://github.com/seanmonstar/reqwest) - powerful Rust HTTP Client.
[Does reqwest support HTTP Multiplexing?](https://github.com/seanmonstar/reqwest/discussions/1470)

Reuse and share tpc connections and multiplex supporting for php-fpm\
Attempt to resolve some problems what described here https://github.com/guzzle/guzzle/issues/1249

```bash

docker run --rm -u $UID -v $PWD:/app -w /app rust:alpine3.16 cargo build
php -d extension=target/debug/libreqwest.so index.php 

docker build . -t php-fpm:ubuntu

cargo build --release
docker compose up -d
docker compose cp target/release/libreqwest.so php-fpm:/usr/local/lib/php/extensions/no-debug-non-zts-20210902

docker compose exec php-fpm docker-php-ext-enable libreqwest
docker compose exec php-fpm bash
apt-get update
apt-get install libssl-dev
cd /usr/lib/x86_64-linux-gnu
ln -s libssl.so libssl.so.3
ln -s libcrypto.so libcrypto.so.3
php -r "var_dump(extension_loaded('reqwest'));"

docker compose restart php-fpm
```
