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

(defn part-two []
  (let [r (run-paint-program
    (new-robot-start-white)
    (new-intcode-client "http://intcode.docker.localhost")
    (get-puzzle-input "https://adventofcode.com/2019/day/11/input")
    0
    [1])]
      (let
        [ minx (apply min (map first (keys (:panels r))))
          maxx (apply max (map first (keys (:panels r))))
          maxy (* -1 (apply min (map second (keys (:panels r)))))
          miny (apply max (map second (keys (:panels r))))
          grid (apply map vector (partition (inc maxy) (map #(if (= :WHITE %) "#" " ") (for [x (range (inc maxx)) y (range (inc maxy))] (:color (get (:panels r) [x (* -1 y)]))))))
          ]
        (println (clojure.string/join "\n" (map #(clojure.string/join "" %) grid)))
    )))

(defn -main
  "Runs all parts"
  [& args]
  (doall
    (part-one)
    (part-two)))