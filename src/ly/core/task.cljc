(ns ly.core.task
  (:require [clojure.spec.alpha :as s]
            [ly.core.base :as base]))

(s/def ::id number?)
(s/def ::summary string?)
(s/def ::lane-id number?)
(s/def ::estimate #(and (number? %) (>= % 0)))
(s/def ::tags (s/* string?))
(s/def ::created-at string?)
(s/def ::updated-at string?)
(s/def ::task
  (s/keys
   :req
   [::id
    ::lane-id
    ::summary
    ::estimate
    ::tags]
   :opt
   [::created-at
    ::updated-at]))
