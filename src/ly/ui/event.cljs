(ns ly.ui.event
  (:require [re-frame.core :refer [reg-event-db reg-event-fx]]
            [ly.core.task :as t]
            [ly.core.lane :as l]
            [ly.ui.db :as db]
            [ajax.core :as ajax]
            [day8.re-frame.http-fx]))

(reg-event-fx
 :init
 (fn [_ _]
   {:db db/init
    :fx [[:dispatch [:fetch-lane-list]]]}))

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
;; Lane fetch
;; ------------------------------------
(reg-event-fx
 :fetch-lane-list
 (fn [_ [_]]
   {:http-xhrio
    {:method :get
     :uri "/api/lanes"
     :timeout 8000
     :response-format (ajax/json-response-format {:keywords? true})
     :on-success [:fetch-lane-list-ok]
     :on-failure [:fetch-lane-list-fail]}}))
(reg-event-fx
 :fetch-lane-list-ok
 ;; TODO It seems to be heavy operation when this event involves fetching tasks
 (fn [{:keys [db]} [_ result]]
   {:db (assoc db ::db/lanes result)
    :fx (map (fn [d] [:dispatch [:fetch-task-list (::l/id d)]]) result)}))
(reg-event-db
 :fetch-lane-list-fail
 (fn [db _]
   db))

;; ------------------------------------
;; Task fetch
;; ------------------------------------
(reg-event-fx
 :fetch-task-list
 (fn [_ [_ lane-id]]
   (println "fetch-task-list")
   {:http-xhrio
    {:method :get
     :uri "/api/tasks"
     :params {:lane-id lane-id} 
     :timeout 8000
     :response-format (ajax/json-response-format {:keywords? true})
     :on-success [:fetch-task-list-ok lane-id]
     :on-failure [:fetch-task-list-fail]}}))
(reg-event-db
 :fetch-task-list-ok
 (fn [db [_ lane-id result]]
   (println result)
   (assoc-in db [::db/lanes (- lane-id 1) ::db/tasks] result)))
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
   (println "submit-new-task" value)
   {:http-xhrio
    {:method          :post
     :uri             "/api/tasks"
     :params          value
     :timeout 8000
     :response-format  (ajax/json-response-format {:keywords? true})
     :format          (ajax/json-request-format)
     :on-success      [:submit-new-task-ok (::t/lane-id value)]
     :on-failure      [:submit-new-task-fail]} }))
    ;;(merge http-post-config
    ;;       {:uri "/api/tasks"
    ;;        :params value
    ;;        :on-success [:submit-new-task-ok]
    ;;        :on-failure [:submit-new-task-fail]})}))

(reg-event-fx
 :submit-new-task-ok
 (fn [{:keys [db]} [_ lane-id _result]]
   (println "submit-new-task-ok")
   (let [new-db (assoc db ::db/new-task (db/init-task))]
     (println "new-db" new-db)
     ;; initialize input state after succceeded
     {:db new-db
      :fx [[:dispatch [:fetch-task-list lane-id]]]})))
(reg-event-db
 :submit-new-task-fail
 (fn [db _]
   db))
