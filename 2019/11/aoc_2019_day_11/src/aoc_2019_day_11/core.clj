(ns aoc-2019-day-11.core
  (:gen-class)
  (:require [clojure.core.match :refer [match]]
            [aoc-2019-day-11.intcode-service :refer [new-intcode-client eval-intcode]]
            [aoc-2019-day-11.input-fetcher :refer [get-puzzle-input]]
            [aoc-2019-day-11.robot :refer :all]))

(defn run-paint-program [r intcode-client prg program-counter inputs]
  (let [response (eval-intcode intcode-client prg program-counter inputs)]
    (match [(get response "blocked") (get response "output-signals")]
      [true [x y]] 
        (do
          ;; (println (get response "output-signals"))
          ;; (println (get response "program-counter"))
          (let [next (move-robot (point
                      (paint-panel r (if (= x 0) :BLACK :WHITE))
                      (if (= y 0)
                        (left r)
                        (right r))))]
            (do
              ;; (println next)
              (recur
                next
                intcode-client
                (get response "program")
                (get response "program-counter")
                [(match [(get-current-panel-color next)]
                  [:BLACK] 0
                  [:WHITE] 1
                  [_] (throw (Exception. "Bad panel color")))]))))
      [nil [x y]]
        (move-robot (point
          (paint-panel r (if (= x 0) :BLACK :WHITE))
          (if (= y 0)
            (left r)
            (right r))))
      [nil []] r
      [_ _] (throw (Exception. "Bad response")))))

(defn part-one []
  (let [r (run-paint-program
    (new-robot)
    (new-intcode-client "http://intcode.docker.localhost")
    (get-puzzle-input "https://adventofcode.com/2019/day/11/input")
    0
    [0])] (do
      ;; (println r)
      (println (count (filter #(> (:times-painted %) 0) (vals (:panels r)))))
      r
    )))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (part-one))
  


