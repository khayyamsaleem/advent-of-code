FROM ruby
WORKDIR /app
COPY . /app
RUN gem install bundler
RUN bundle
CMD ["ruby", "main.rb"]