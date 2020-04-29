(ns aoc-2019-day-11.input-fetcher-test
  (:require [clojure.test :refer :all]
            [aoc-2019-day-11.input-fetcher :refer :all]))

(deftest test-get-cookie-header
  (testing "fetching cookie header yields string longer than key"
    (is (> (count (get-cookie-header)) (count "session=")))))

(deftest test-get-puzzle-input
  (testing "fetching puzzle input yields puzzle input"
    (is (> (count (get-puzzle-input "https://adventofcode.com/2019/day/11/input")) 0))))