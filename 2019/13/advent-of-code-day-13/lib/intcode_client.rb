require 'faraday'
require 'json'

class IntcodeClient
    def initialize(base_url)
        @base_url = base_url
        @conn = Faraday.new(@base_url)
    end

    def health_check()
        JSON.parse @conn.get("/health").body
    end

    def eval_intcode(program, inputs = [], program_counter = 0)
        JSON.parse @conn.post("/eval", {
            program: program,
            inputs: inputs,
            program_counter: program_counter
        }.to_json, { 'Content-Type' => 'application/json' }).body
    end
end