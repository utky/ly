(ns ly.ui.event
  (:require [re-frame.core :refer [reg-event-db]]
            [ly.ui.db :as db]))

(reg-event-db
 :init
 (fn [_ _]
   db/init))
