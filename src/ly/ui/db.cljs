(ns ly.ui.db
  (:require [clojure.spec.alpha :as s]
            [re-frame.core :as rf]
            [ly.core.task :as t]))

(s/def ::backlog (s/* ::t/task))
(s/def ::db (s/keys :req [::backlog]))

(def init
  (s/conform ::db
    {::backlog
     [{::t/id 1 ::t/summary "task1"}
      {::t/id 2 ::t/summary "task2"}]
     }))

