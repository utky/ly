(ns ly.core.base
  (:require [clojure.spec.alpha :as s]))

(defn map->nsmap
  [n m]
  (reduce-kv (fn [acc k v]
               (let [new-kw (if (and (keyword? k)
                                     (not (qualified-keyword? k)))
                              (keyword (str n) (name k))
                              k)]
                 (assoc acc new-kw v)))
             {} m))
