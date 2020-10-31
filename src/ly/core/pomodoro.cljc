(ns ly.core.pomodoro
  (:require [clojure.spec.alpha :as s]
            [ly.core.base :as base]
            [ly.core.task :as task]))

;; ドメイン定義
(s/def ::id number?)
(s/def ::task-id ::task/id)
(s/def ::started-at inst?)
(s/def ::finished-at inst?)
(s/def ::pomodoro
  (s/keys
   :req
   [::id
    ::task-id
    ::started-at
    ::finished-at]))
(s/def ::new-pomodoro
  (s/keys
   :req
   [::task-id
    ::started-at
    ::finished-at]))
