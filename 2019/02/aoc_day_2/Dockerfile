FROM nimlang/nim:alpine
COPY . /app
WORKDIR /app
RUN nimble build -y
ENV AOC_DAY_2 "./aoc_day_2"
RUN chmod +x /app/run.sh
CMD ["/app/run.sh"]
