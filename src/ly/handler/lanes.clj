(ns ly.handler.lanes
  (:require 
   [ataraxy.response :as response] 
   [integrant.core :as ig]
   [ly.boundary.lane :as boundary]))

(defmethod ig/init-key ::list [_ {:keys [db]}]
  (fn [{[_] :ataraxy/result}]
    [::response/ok (boundary/list-lanes db)]))
