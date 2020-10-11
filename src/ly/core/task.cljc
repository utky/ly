(ns ly.core.task
  (:require [clojure.spec.alpha :as s]
            [ly.core.base :as base]))

;; ドメイン定義
(s/def ::id number?)
(s/def ::uuid uuid?)
(s/def ::summary string?)
(s/def ::lane-id number?)
(s/def ::estimate #(and (number? %) (>= % 0)))
(s/def ::done #(and (number? %) (>= % 0)))
(s/def ::tags (s/* string?))
(s/def ::created-at string?)
(s/def ::updated-at string?)
(s/def ::task
  (s/keys
   :req
   [::id
    ::uuid
    ::lane-id
    ::summary
    ::estimate
    ::done
    ::tags]
   :opt
   [::created-at
    ::updated-at]))
(s/def ::new-task
  (s/keys
   :req
   [::lane-id
    ::summary
    ::estimate]
   :opt
   [::tags]))
