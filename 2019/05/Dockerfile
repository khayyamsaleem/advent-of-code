FROM jackfirth/racket:7.4-full
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN raco pkg install -u --auto
RUN raco exe -o ham-aocd5 main.rkt
CMD ["./ham-aocd5"]

