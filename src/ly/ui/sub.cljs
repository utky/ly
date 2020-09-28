(ns ly.ui.sub
  (:require [re-frame.core :refer [reg-sub]]
            [ly.ui.db :as db]))

(reg-sub
 :backlog
 (fn [db _]
   (::db/backlog db)))
(reg-sub
 :todo
 (fn [db _]
   (::db/todo db)))
(reg-sub
 :done
 (fn [db _]
   (::db/done db)))
(reg-sub
 :selected
 (fn [db _]
   (::db/selected db)))
(reg-sub
 :new-task
 (fn [db _]
   (::db/new-task db)))
(reg-sub
 :lanes
 (fn [db _]
   (::db/lanes db)))
