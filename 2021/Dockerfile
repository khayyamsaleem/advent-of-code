FROM imolein/luarocks:5.4
RUN apk add unzip gcc musl-dev openssl-dev make bsd-compat-headers m4
WORKDIR /app
COPY . /app
RUN luarocks init
RUN ./luarocks install --tree lua_modules --only-deps aoc2021-scm-0.rockspec