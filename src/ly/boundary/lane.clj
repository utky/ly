(ns ly.boundary.lane
  (:require [duct.database.sql]
            [honeysql.core :as sql]
            [ly.boundary.core :as core]))

(defprotocol Lane
  (list-lanes [db]))

(extend-protocol Lane
  duct.database.sql.Boundary
  (list-lanes [db]
    (core/query
     db
     (sql/format
      {:select
       [:id :name :created-at :updated-at]
       :from
       [:lanes]}))))

