(ns aoc-2019-day-11.robot
  (:gen-class)
  (:require [clojure.core.match :refer [match]]))

(defn visit-panel [r]
  (assoc r :panels
    (update
      (:panels r)
      [(:x r) (:y r)]
      #(if (nil? %) {:color :BLACK :times-painted 0} %))))

(defn move-robot [r] 
  (match [(:orientation r)]
    [:UP] 
      (visit-panel (update r :y inc))
    [:DOWN]
      (visit-panel (update r :y dec))
    [:LEFT]
      (visit-panel (update r :x dec))
    [:RIGHT]
      (visit-panel (update r :x inc))))

(defn point [r new_orientation] (assoc r :orientation new_orientation))

(defn left [r]
  (match [(:orientation r)]
    [:UP] :LEFT
    [:LEFT] :DOWN
    [:DOWN] :RIGHT
    [:RIGHT] :UP))
    
(defn right [r]
  (match [(:orientation r)]
    [:UP] :RIGHT
    [:RIGHT] :DOWN
    [:DOWN] :LEFT
    [:LEFT] :UP))

(defn get-current-panel-color [r]
  (:color (get (:panels r) [(:x r) (:y r)])))

(defn paint-panel [r color]
  (assoc r :panels
    (update
      (:panels r)
      [(:x r) (:y r)] 
      #(assoc {} :color color :times-painted (inc (:times-painted %))))))

(defn new-robot
  ([]
    (new-robot 0 0 :UP))
  ([x y orientation]
    (new-robot x y orientation {[x y] {:color :BLACK :times-painted 0}}))
  ([x y orientation panels]
    {:x x :y y :orientation orientation :panels panels}))

(defn new-robot-start-white
  ([]
    (new-robot-start-white 0 0 :UP))
  ([x y orientation]
    (new-robot-start-white x y orientation {[x y] {:color :WHITE :times-painted 0}}))
  ([x y orientation panels]
    {:x x :y y :orientation orientation :panels panels}))