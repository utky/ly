(ns ly.core.task
  (:require [clojure.spec.alpha :as s]))

(s/def ::id number?)
(s/def ::summary string?)
(s/def ::task
  (s/keys :req [::id ::summary]))
