(ns ly.core.lane
  (:require [clojure.spec.alpha :as s]))

(s/def ::id number?)
(s/def ::name string?)
(s/def ::created-at string?)
(s/def ::updated-at string?)
(s/def ::lane
  (s/keys
   :req
   [::id
    ::name]
   :opt
   [::created-at
    ::updated-at]))
