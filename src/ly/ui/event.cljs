(ns ly.ui.event
  (:require [re-frame.core :refer [reg-event-db]]
            [ly.ui.db :as db]))

(reg-event-db
 :init
 (fn [_ _]
   db/init))

(reg-event-db
 :select-task
 (fn [db [_ task-id]]
   (assoc db ::db/selected task-id)))
