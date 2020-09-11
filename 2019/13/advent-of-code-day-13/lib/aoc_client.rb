require 'faraday'
require 'json'

class AOCClient 
    def initialize(base_url, session_token)
        @base_url = base_url
        @conn = Faraday.new(
            url: @base_url,
            headers: {'Cookie' => "session=#{session_token}"}
        )
    end

    def get_input_for_year_and_day(year, day)
        @conn.get("/#{year}/day/#{day}/input").body
    end
end