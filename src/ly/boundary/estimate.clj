(ns ly.boundary.estimate
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [honeysql.helpers :refer :all :as helpers]
            [ly.boundary.core :as core]
            [ly.core.base :as base]
            [ly.core.estimate :as e]))

(defprotocol Estimate
  (list-estimates [db task-id]))

(extend-protocol Estimate
  duct.database.sql.Boundary
  (list-estimates [db task-id]
    (map
     #(base/map->nsmap (create-ns 'ly.core.estimate) %)
     (core/query
      db
      (sql/format
       {:select [:id :task-id :value :created-at :updated-at]
        :from [:estimates]})))))

