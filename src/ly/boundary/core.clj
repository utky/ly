(ns ly.boundary.core
  (:require [clojure.java.jdbc :as jdbc]
            [clojure.string :as string]))

(defn kebab [col]
  (-> col string/lower-case (string/replace "_" "-")))

(defn query
  [db q]
  ;;(jdbc/query (:spec db) q :identifiers kebab))
  (jdbc/query (:spec db) q))

(defn execute!
  [db q]
  ;;(jdbc/execute! (:spec db) q :identifiers kebab))
  (jdbc/execute! (:spec db) q))
