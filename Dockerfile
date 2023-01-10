FROM ubuntu:22.10
ARG DEBIAN_FRONTEND=noninteractive

RUN apt update -y && apt install -y php-fpm
COPY target/release/libreqwest.so /usr/lib/php/20210902/
RUN echo "extension=libreqwest.so" >> /etc/php/8.1/cli/conf.d/20-reqwest.ini && echo "extension=libreqwest.so" >>/etc/php/8.1/fpm/conf.d/20-reqwest.ini

COPY docker-php-entrypoint /bin/docker-php-entrypoint
RUN chmod +x /bin/docker-php-entrypoint

#ENTRYPOINT /bin/docker-php-entrypoint
#CMD ["php-fpm8.1", "-F"]
ENTRYPOINT ["php-fpm8.1", "-F"]
