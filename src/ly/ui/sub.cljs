(ns ly.ui.sub
  (:require [re-frame.core :refer [reg-sub]]
            [ly.ui.db :as db]))

(reg-sub
 :backlog          ;; usage: (subscribe [:active-page])
 (fn [db _]            ;; db is the (map) value stored in the app-db atom
   (::db/backlog db))) ;; extract a value from the application state
