(ns ly.ui.db
  (:require [clojure.spec.alpha :as s]
            [re-frame.core :as rf]
            [ly.core.task :as t]
            [ly.core.lane :as l]))

(defn get-timer-seconds [timer-type]
  (* 60 (cond (= timer-type :pomodoro)    1;; 25
              (= timer-type :short-break) 5
              (= timer-type :long-break)  15
              :else                       1)))
(s/def ::timer-state #{:stopped :running :paused})
(s/def ::timer-type #{:pomodoro :short-break :long-break})
(s/def ::timer-remaining number?) ;; seconds
(s/def ::timer-last-updated inst?)
(s/def ::timer-id number?)
(s/def ::timer
       (s/keys
        :req
        [::timer-type
         ::timer-state
         ::timer-remaining]
        :opt
        [::timer-last-updated
         ::timer-id]))
(defn init-timer []
  {::timer-type  :pomodoro
   ::timer-state :stopped
   ::timer-remaining 0})

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
   [::timer
    ::new-task
    ::lanes
    ::done]
   :opt
   [::selected
    ]))

(def init
  (s/conform ::db
    {::timer
     (init-timer)
     
     ::new-task
     (init-task)

     ::lanes
     []

     ::done
     {::tasks
      []}}))
