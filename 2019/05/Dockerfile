FROM jackfirth/racket:7.5-full
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN raco pkg install -u --auto --no-docs
RUN raco exe -o ham-aocd5-server intcode-server.rkt
EXPOSE 1337
CMD ["./ham-aocd5-server"]

