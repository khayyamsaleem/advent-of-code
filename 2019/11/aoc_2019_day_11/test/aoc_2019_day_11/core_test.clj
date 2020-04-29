(ns aoc-2019-day-11.core-test
  (:require [clojure.test :refer :all]
            [aoc-2019-day-11.core :refer :all]))

(deftest test-move-robot
  (testing "moving robot when orientation is up moves robot up"
    (is (=
      (move-robot (new-robot))
      (new-robot 0 1 :UP {[0 0] {:color :BLACK :times-painted 0} [0 1] {:color :BLACK :times-painted 0}}))))
  (testing "moving robot when orientation is down moves robot down"
    (is (=
      (move-robot (new-robot 0 0 :DOWN))
      (new-robot 0 -1 :DOWN {[0 0] {:color :BLACK :times-painted 0} [0 -1] {:color :BLACK :times-painted 0}}))))
  (testing "moving robot when orientation is left moves robot left"
    (is (=
      (move-robot (new-robot 0 0 :LEFT))
      (new-robot -1 0 :LEFT {[0 0] {:color :BLACK :times-painted 0} [-1 0] {:color :BLACK :times-painted 0}}))))
  (testing "moving robot when orientation is right moves robot right"
    (is (=
      (move-robot (new-robot 0 0 :RIGHT))
      (new-robot 1 0 :RIGHT {[0 0] {:color :BLACK :times-painted 0} [1 0] {:color :BLACK :times-painted 0}}))))
    )

(deftest test-paint-panel
  (testing "panel painted with desired color when paint called"
    (is (=
      (paint (new-robot) :WHITE)
      (new-robot 0 0 :UP {[0 0] {:color :WHITE :times-painted 1}}))))
    )
