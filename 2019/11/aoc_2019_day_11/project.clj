(defproject aoc_2019_day_11 "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "EPL-2.0 OR GPL-2.0-or-later WITH Classpath-exception-2.0"
            :url "https://www.eclipse.org/legal/epl-2.0/"}
  :dependencies [
    [org.clojure/clojure "1.10.1"]
    [org.clojure/core.match "1.0.0"]
    [cwhitey/dotty "0.2.3"]
    [clj-http "3.10.1"]
    [cheshire "5.10.0"]
    ]
  :main ^:skip-aot aoc-2019-day-11.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
