(ns ly.boundary.task
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]
            [ly.boundary.core :as core]
            [ly.core.base :as base]
            [ly.core.task :as t]))

(defprotocol Task 
  (list-tasks [db lane-id])
  (new-task [db task]))

(extend-protocol Task
  duct.database.sql.Boundary
  (list-tasks [db lane-id]
    (map
     #(base/map->nsmap (create-ns 'ly.core.task) %)
     (core/query
      db
      (sql/format
       (-> (select :t.id :t.uuid :t.summary :t.lane-id [:%sum.e.value :estimate] [:%count.p.id :done] :t.created-at :t.updated-at)
           (from [:tasks :t])
           (join [:estimates :e] [:= :t.id :e.task-id])
           (left-join [:pomodoros :p] [:= :t.id :p.task-id])
           (group :t.id)
           ((fn [q] (if lane-id (merge q {:where [:= :lane-id lane-id]}) q))))))))

  (new-task [db task]
    (let [uuid (java.util.UUID/randomUUID)]
      (core/execute!
       db
       (-> (insert-into :tasks)
           (columns
            :uuid
            :summary
            :lane-id)
           (values
            [(concat
              [uuid]
              (vec (map (fn [k] (k task)) [::t/summary ::t/lane-id])))])
           sql/format))
      (let [created-task
            (core/query
             db
             (sql/format
              {:select [:id] :from [:tasks] :where [:= :uuid uuid]}))]
        (println "created-task" created-task)
        (core/execute!
         db
         (-> (insert-into :estimates)
             (columns
              :task-id
              :value)
             (values
              [[(:id (first created-task)) (::t/estimate task)]])
             sql/format))))))
