FROM composer
WORKDIR /app
COPY composer.json /app
RUN docker-php-ext-install bcmath
RUN composer install

FROM php:alpine
WORKDIR /app
COPY . /app
RUN docker-php-ext-install bcmath
COPY --from=0 /app/vendor /app/vendor
CMD ["php", "index.php"]
