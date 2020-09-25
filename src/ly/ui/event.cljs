(ns ly.ui.event
  (:require [re-frame.core :refer [reg-event-db reg-event-fx]]
            [ly.core.task :as t]
            [ly.ui.db :as db]
            [ajax.core :as ajax]
            [day8.re-frame.http-fx]))

(def http-base-config
  {:timeout 8000
   :reponse-format (ajax/json-response-format {:keywords? true})})

(def http-post-config
  (merge
   http-base-config
   {:method          :post
    :format          (ajax/json-request-format)}))

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
 (fn [db [_ value]]
   (assoc-in db [::db/new-task ::t/summary] value)))

(reg-event-db
 :change-new-estimate
 (fn [db [_ value]]
   (assoc-in db [::db/new-task ::t/estimate] value)))

(reg-event-db
 :change-new-lane-id
 (fn [db [_ value]]
   (assoc-in db [::db/new-task ::t/lane-id] value)))

;; ------------------------------------
;; Task fetch
;; ------------------------------------
(reg-event-fx
 :fetch-task-list
 (fn [_ [_ lane-id]]
   {:http-xhrio
    (merge http-base-config
           {:method :get
            :uri "/api/tasks"
            :params {:lane-id lane-id}
            :on-success [:fetch-task-list-ok lane-id]
            :on-failure [:fetch-task-list-fail]})}))
(reg-event-db
 :fetch-task-list-ok
 (fn [db [_ lane-id result]]
   ;; ここは view 側を lanes のリストベースで打ち分けるようにする
   (let [k (if (= lane-id 1) ::db/backlog ::db/todo)]
     (update db k ::db/tasks result))))
(reg-event-db
 :fetch-task-list-fail
 (fn [db _]
   db))

;; ------------------------------------
;; Task new
;; ------------------------------------
(reg-event-fx
 :submit-new-task
 (fn [_ [_ value]]
   {:http-xhrio
    {:method          :post
     :uri             "/api/tasks"
     :params          value
     :timeout         5000
     :format          (ajax/json-request-format)
     :response-format (ajax/json-response-format {:keywords? true})
     :on-success      [:submit-new-task-ok]
     :on-failure      [:submit-new-task-fail]}}))
    ;;(merge http-post-config
    ;;       {:uri "/api/tasks"
    ;;        :params value
    ;;        :on-success [:submit-new-task-ok]
    ;;        :on-failure [:submit-new-task-fail]})}))

(reg-event-db
 :submit-new-task-ok
 (fn [db _]
   (update db ::db/new-task (db/init-task))))
(reg-event-db
 :submit-new-task-fail
 (fn [db _]
   db))
