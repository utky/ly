(ns ly.ui.event
  (:require [re-frame.core :refer [reg-event-db]]
            [ly.core.task :as t]
            [ly.ui.db :as db]))

(reg-event-db
 :init
 (fn [_ _]
   db/init))

(reg-event-db
 :select-task
 (fn [db [_ task-id]]
   (assoc db ::db/selected task-id)))

(reg-event-db
 :change-new-summary
 (fn [db [_ value lane-key]]
   (js/console.log "value" value)
   (js/console.log "lane-key" lane-key)
   (assoc-in db [lane-key ::db/new-task ::t/summary] value)))

(reg-event-db
 :change-new-estimate
 (fn [db [_ value lane-key]]
   (js/console.log "value" value)
   (js/console.log "lane-key" lane-key)
   (assoc-in db [lane-key ::db/new-task ::t/estimate] value)))
