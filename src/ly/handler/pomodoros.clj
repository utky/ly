(ns ly.handler.pomodoros
  (:require 
   [ataraxy.response :as response] 
   [integrant.core :as ig]
   [ly.boundary.pomodoro :as boundary]
   [clojure.set :as set]
   [ly.core.pomodoro :as p]))

(defmethod ig/init-key ::list [_ {:keys [db]}]
  (fn [{[_ from to task-id] :ataraxy/result}]
    [::response/ok (boundary/list-pomodoros db {:from from :to to :task-id task-id})]))

(defmethod ig/init-key ::new [_ {:keys [db]}]
  (fn [{[_ pomodoro] :ataraxy/result}]
    [::response/ok (boundary/new-pomodoro db (set/rename-keys pomodoro {:task-id ::p/task-id :started-at ::p/started-at :finished-at ::p/finished-at}))]))

