(ns ly.boundary.task
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]
            [ly.boundary.core :as core]))

(defprotocol Task 
  (list-tasks [db])
  (new-task [db task]))

(extend-protocol Task
  duct.database.sql.Boundary
  (list-tasks [db]
    (core/query
     db
     (sql/format {:select [:id :summary :lane_id :created_at :updated_at] :from [:tasks]})))
  (new-task [db task]
    (core/execute!
     db
    (-> (insert-into :tasks)
        (columns :summary :lane_id)
        (values [[(:summary task) (:lane_id task)]])
        sql/format))))
