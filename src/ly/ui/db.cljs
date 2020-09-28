(ns ly.ui.db
  (:require [clojure.spec.alpha :as s]
            [re-frame.core :as rf]
            [ly.core.task :as t]
            [ly.core.lane :as l]))

(s/def ::entering boolean?)
(s/def ::new-task
  (s/keys
   :req
   [::entering
    ::t/lane-id
    ::t/summary
    ::t/estimate
    ::t/tags]))

(defn init-task []
  (s/conform
   ::new-task
   {::entering true
    ::t/lane-id 1
    ::t/summary ""
    ::t/estimate 0
    ::t/tags []}))

(s/def ::tasks (s/* ::t/task))

(s/def ::backlog
  (s/keys
   :req
   [::tasks]))

(s/def ::todo
  (s/keys
   :req
   [::tasks]))

(s/def ::done
  (s/keys
   :req
   [::tasks]))

(s/def ::lane
  (s/keys
   :req
   [::l/id
    ::l/name]
   :opt
   [::tasks]))

(s/def ::lanes (s/* ::lane))

(s/def ::selected ::t/id)
(s/def ::db
  (s/keys
   :req
   [::new-task
    ::lanes
    ::done]
   :opt
   [::selected
    ]))

(def init
  (s/conform ::db
    {::new-task
     (init-task)

     ::lanes
     []

     ::done
     {::tasks
      []}}))
