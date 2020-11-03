(ns ly.boundary.pomodoro
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]
            [ly.boundary.core :as core]
            [ly.core.base :as base]
            [ly.core.pomodoro :as p]))

(defprotocol Pomodoro
  (list-pomodoros [db {:keys [from to task-id]}])
  (new-pomodoro [db pomodoro]))

(extend-protocol Pomodoro
  duct.database.sql.Boundary
  (list-pomodoros [db {:keys [from to task-id]}]
    (map
     #(base/map->nsmap (create-ns 'ly.core.pomodoro) %)
     (core/query
      db
      (sql/format
       (cond->
        {:select
         [:id :task-id :started-at :finished-at]
         :from
         [:pomodoros]
         :order-by [:started-at]}
         (not (nil? from))    (merge-where [:>= :started-at from])
         (not (nil? to))      (merge-where [:<  :finished-at to])
         (not (nil? task-id)) (merge-where [:=  :task-id task-id]))))))
  (new-pomodoro [db pomodoro]
    (core/execute!
     db
     (-> (insert-into :pomodoros)
         (columns
          :task-id
          :started-at
          :finished-at)
         (values
          [(vec (map #(% pomodoro) [::p/task-id ::p/started-at ::p/finished-at]))])
         sql/format))))

