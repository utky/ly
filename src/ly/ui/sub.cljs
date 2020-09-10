(ns ly.ui.sub
  (:require [re-frame.core :refer [reg-sub]]
            [ly.ui.db :as db]))

(reg-sub
 :backlog
 (fn [db _]
   (::db/backlog db)))

(reg-sub
 :selected
 (fn [db _]
   (::db/selected db)))
