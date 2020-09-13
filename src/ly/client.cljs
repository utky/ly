(ns ly.client
  (:require [reagent.core :as r]
            [reagent.dom :as rdom]
            [re-frame.core :as rf]
            [day8.re-frame.http-fx]
            [ajax.core :as ajax]
            [goog.events :as events]
            [clojure.string :as str]
            [ly.ui.event]
            [ly.ui.sub]
            [ly.ui.main :as main]))

;; -- Key event -------------------------------------------------------------
(rf/reg-event-fx
 :keydown
 (fn [{:keys [db]} [_ e]]
   (js/console.log "target" (.-target e))
   (js/console.log "code"   (.-code e))
   (js/console.log "ctrlKey" (.-ctrlKey e))
   (js/console.log "altKey" (.-altKey e))
   (js/console.log "shiftKey" (.-shiftKey e))
   (js/console.log "metaKey" (.-metaKey e))))
(rf/reg-event-fx
 :keyup
 (fn [{:keys [db]} [_ e]]
   (js/console.log "target" (.-target e))
   (js/console.log "code"   (.-code e))
   (js/console.log "ctrlKey" (.-ctrlKey e))
   (js/console.log "altKey" (.-altKey e))
   (js/console.log "shiftKey" (.-shiftKey e))
   (js/console.log "metaKey" (.-metaKey e))))

;; -- Entry Point -------------------------------------------------------------

(defn app []
  [main/main])

(defn ^:export run
  []
  ;; (js/document.addEventListener "keydown" (fn [e] (js/console.log "handle keydown" e) (rf/dispatch [:keydown e])))
  ;; (js/document.addEventListener "keyup" (fn [e] (js/console.log "handle keyup" e) (rf/dispatch [:keyup e])))
  (rf/dispatch-sync [:init])
  (rdom/render [app]              ;; mount the application's ui into '<div id="app" />'
    (js/document.getElementById "app")))
