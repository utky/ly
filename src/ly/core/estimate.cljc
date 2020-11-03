(ns ly.core.estimate
  (:require [clojure.spec.alpha :as s]
            [ly.core.base :as base]
            [ly.core.task :as task]))

(s/def ::id number?)
(s/def ::task-id ::task/id)
(s/def ::value number?)
(s/def ::started-at inst?)
(s/def ::finished-at inst?)

(s/def ::estimate
  (s/keys
   :req
   [::id
    ::task-id
    ::value]
   :opt
   [::created-at
    ::updated-at]))
