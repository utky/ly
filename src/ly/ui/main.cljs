(ns ly.ui.main
  (:require [reagent.core :as r]
            [reagent.dom :as rd]
            [re-frame.core :as rf]))

(defn task []
  [:div.level
   [:div.level-left
    [:div.level-item [:span "task"]]]
   [:div.level-right
    [:div.level-item [:span "*--"]]]])

(defn lane [header]
  [:div.column
   {:style {:border-left-color "#dbdbdb"
            :border-left-style "solid"
            :border-left-width "1px"}}
   [:div
    [:h1.title header]
    [:ul
      [:li [task]]
      [:li [task]]]]])

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
