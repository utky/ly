(ns ly.ui.sub
  (:require [re-frame.core :refer [reg-sub]]
            [ly.core.task :as t]
            [ly.core.lane :as l]
            [ly.ui.db :as db]))

(reg-sub
 :timer
 (fn [db _]
   (::db/timer db)))
(reg-sub
 :backlog
 (fn [db _]
   (first (filter #(= (::l/name %) "backlog") (::db/lanes db)))))
(reg-sub
 :todo
 (fn [db _]
   (first (filter #(= (::l/name %) "todo") (::db/lanes db)))))
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
(reg-sub
 :current
 (fn [db _]
   (::db/current db)))
