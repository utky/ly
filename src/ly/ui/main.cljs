(ns ly.ui.main
  (:require [reagent.core :as r]
            [reagent.dom :as rd]
            [ly.core.task :as t]
            [re-frame.core :refer [subscribe dispatch]]))

(defn task [t selected]
  [:div.level
   {:style (assoc (if selected {:background-color "#FFFFE0"} {}) :padding "5px")} 
   [:div.level-left
    [:div.level-item
     [:span (::t/summary t)]]]
   [:div.level-right
    [:div.level-item
     [:span {:style { :color "red" :font-size "20px" :font-weight "bolder" }}
      "●--"]]]])

(defn lane [header]
  (let [backlog     @(subscribe [:backlog])
        selected-id @(subscribe [:selected])]
    [:div.column
   {:style {:border-left-color "#dbdbdb"
            :border-left-style "solid"
            :border-left-width "1px"}}
   [:div
    [:h1.title header]
    [:ul
      (for [t backlog]
        [:li
         {:on-click #(dispatch [:select-task (::t/id t)])
          :key (::t/id t)}
         [task t (= selected-id (::t/id t))]])]]]))

(defn pomodoro-status []
  [:div
   [:span "09:00"]
   [:progress.progress.is-danger {:value 15 :max 100}]])

(defn status-bar []
  [:div.navbar
   [:div.navbar-menu
    [:div.navbar-start
     [:div.navbar-item
      [:span.title "current working task "]]]
    [:div.navbar-end
     [:div.navbar-item
      [pomodoro-status]]]]])

(defn main []
  [:div.container
   [status-bar]
   [:div.tabs
    [:ul
     [:li.is-active [:a "Tasks"]]
     [:li [:a "Statistics"]]]]
   [:div.columns
    [lane "backlog"]
    [lane "todo"]
    [lane "done"]]])
