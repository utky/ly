(ns ly.core.task
  (:require [clojure.spec.alpha :as s]))

(s/def ::id number?)
(s/def ::summary string?)
(s/def ::lane-id number?)
(s/def ::estimate #(and (number? %) (>= % 0)))
(s/def ::tags (s/* string?))
(s/def ::task
  (s/keys
   :req
   [::id
    ::lane-id
    ::summary
    ::estimate
    ::tags]))
