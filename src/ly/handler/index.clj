(ns ly.handler.index
  (:require 
   [ataraxy.response :as response] 
   [integrant.core :as ig]))

(defmethod ig/init-key ::get [_ {:keys []}]
  (fn [{[_] :ataraxy/result}]
    [::response/found "/index.html"]))
