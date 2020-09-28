(ns ly.boundary.lane
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [ly.boundary.core :as core]
            [ly.core.base :as base]))

(defprotocol Lane
  (list-lanes [db]))

(extend-protocol Lane
  duct.database.sql.Boundary
  (list-lanes [db]
    (map
     #(base/map->nsmap (create-ns 'ly.core.lane) %)
     (core/query
      db
      (sql/format
       {:select
        [:id :name :created-at :updated-at]
        :from
        [:lanes]
        :order-by [:id]})))))

