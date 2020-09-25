(ns ly.handler.tasks
  (:require 
   [ataraxy.response :as response] 
   [integrant.core :as ig]
   [ly.boundary.task :as boundary]))

(defmethod ig/init-key ::list [_ {:keys [db]}]
  (fn [{[_] :ataraxy/result}]
    [::response/ok (boundary/list-tasks db)]))

(defmethod ig/init-key ::new [_ {:keys [db]}]
  (fn [{[_ task] :ataraxy/result}]
    [::response/created (boundary/new-task db task)]))
