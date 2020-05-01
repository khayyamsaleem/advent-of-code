(ns aoc-2019-day-11.input-fetcher
  (:gen-class)
  (:require [clojure.string :refer [trim]]
            [dotty.core :refer [env]]
            [clj-http.client :as client]
            [cheshire.core :as json :refer :all]))

(defn get-cookie-header [] (str "session=" (env "session")))

(defn get-puzzle-input [uri]
  (parse-string (str "[" (trim
    (:body
      (client/get
        uri
        {:headers {"Cookie" (get-cookie-header)}}))) "]")))