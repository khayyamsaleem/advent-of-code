FROM haskell
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN stack setup
RUN stack build
CMD ["stack", "run"]
