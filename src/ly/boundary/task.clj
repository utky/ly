(ns ly.boundary.task
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]
            [ly.boundary.core :as core]))

(defprotocol Task 
  (list-tasks [db lane-id])
  (new-task [db task]))

(extend-protocol Task
  duct.database.sql.Boundary
  (list-tasks [db lane-id]
    (core/query
     db
     (sql/format
      (merge 
       {:select
       [:id :summary :lane-id :created-at :updated-at]
       :from
       [:tasks]}
       (if lane-id {:where [:= :lane-id lane-id]} {})))))
  (new-task [db task]
    (core/execute!
     db
    (-> (insert-into :tasks)
        (columns :summary :lane-id)
        (values [[(:summary task) (:lane-id task)]])
        sql/format))))
