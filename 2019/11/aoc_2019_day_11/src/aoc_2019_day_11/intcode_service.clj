(ns aoc-2019-day-11.intcode-service
  (:gen-class)
  (:require [clj-http.client :as client]
            [cheshire.core :as json :refer :all]))

(defn new-intcode-client [uri] {:uri uri})

(defn check-alive [intcode-client]
    (= 200 (:status (client/get (str (:uri intcode-client) "/health")))))

(defn eval-intcode [intcode-client intcode program_counter inputs]
    (parse-string (:body (client/post (str (:uri intcode-client) "/eval") {:form-params {
        :program intcode
        :program_counter program_counter
        :inputs inputs
    } :content-type :json :accept :json}))))