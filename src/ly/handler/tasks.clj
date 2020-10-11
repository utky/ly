(ns ly.handler.tasks
  (:require 
   [ataraxy.response :as response] 
   [integrant.core :as ig]
   [ly.boundary.task :as boundary]
   [clojure.set :as set]
   [ly.core.task :as t]))

(defmethod ig/init-key ::list [_ {:keys [db]}]
  (fn [{[_ lane-id] :ataraxy/result}]
    [::response/ok (boundary/list-tasks db lane-id)]))

(defmethod ig/init-key ::new [_ {:keys [db]}]
  (fn [{[_ task] :ataraxy/result}]
    [::response/ok (boundary/new-task db (set/rename-keys task {:lane-id ::t/lane-id :summary ::t/summary :estimate ::t/estimate}))]))
