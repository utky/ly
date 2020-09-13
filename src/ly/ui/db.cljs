(ns ly.ui.db
  (:require [clojure.spec.alpha :as s]
            [re-frame.core :as rf]
            [ly.core.task :as t]))

(s/def ::entering boolean?)
(s/def ::new-task
  (s/keys
   :req
   [::entering
    ::t/summary
    ::t/estimate
    ::t/tags]))

(defn init-task []
  (s/conform
   ::new-task
   {::entering true
    ::t/summary ""
    ::t/estimate 0
    ::t/tags []}))

(s/def ::tasks (s/* ::t/task))

(s/def ::backlog
  (s/keys
   :req
   [::new-task
    ::tasks]))

(s/def ::todo
  (s/keys
   :req
   [::new-task
    ::tasks]))

(s/def ::done
  (s/keys
   :req
   [::tasks]))

(s/def ::selected ::t/id)
(s/def ::db
  (s/keys
   :req
   [::backlog
    ::todo
    ::done]
   :opt
   [::selected
    ]))

(def init
  (s/conform ::db
    {::backlog
     {::new-task
      (init-task)
      ::tasks
      [{::t/id 1 ::t/summary "backlog1" ::t/estimate 1 ::t/tags ["plan"]}]}

     ::todo
     {::new-task
      (init-task)
      ::tasks
      [{::t/id 2 ::t/summary "todo1" ::t/estimate 2 ::t/tags ["do"]}]}

     ::done
     {::tasks
      [{::t/id 3 ::t/summary "done1" ::t/estimate 3 ::t/tags ["check"]}]}}))
