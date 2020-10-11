(ns ly.ui.main
  (:require [reagent.core :as r]
            [reagent.dom :as rd]
            [ly.core.task :as t]
            [ly.core.lane :as l]
            [ly.ui.db :as db]
            [re-frame.core :refer [subscribe dispatch]]))

(defn new-task-form []
  (let [new-task @(subscribe [:new-task])
        options [{:id 1 :name "backlog"} {:id 2 :name "todo"}]]
    [:div.field.has-addons
     [:div.control.select
      [:select
       {:value (::t/lane-id new-task)
        :on-change #(dispatch [:change-new-lane-id (-> % .-target .-value)])}
       (for [o options]
         [:option {:value (:id o) :key (:id o)} (:name o)])]]
     [:div.control
      [:input.input
       {:type "text"
        :placeholder "summary"
        :value (::t/summary new-task)
        :on-change #(dispatch [:change-new-summary (-> % .-target .-value)])}]]
     [:div.control
      [:input.input
       {:style {:width "2rem"}
        :type "text"
        :value (::t/estimate new-task)
        :on-change #(dispatch [:change-new-estimate (-> % .-target .-value)])}]]
     [:div.control
      [:input.button.is-primary
       {:type "button"
        :value "add"
        :on-click #(dispatch [:submit-new-task @(subscribe [:new-task])])}]]]))

(defn icon [icon-name]
  [:span.icon
   {:style {:width "inherit"}}
   [:img
    {:src
     (str "/img/"
          (if (keyword? icon-name)
            (name icon-name)
            icon-name)
          ".svg")}]])

(defn done []
  [icon :check-circle])
(defn undone []
  [icon :circle])

(defn task [t selected]
  [:div.level
   {:style (assoc (if selected {:background-color "#FFFFE0"} {}) :padding "5px")} 
   [:div.level-left
    [:div.level-item
     [:span (::t/summary t)]]]
   [:div.level-right
    [:div.level-item
     [:div
      [:span (::t/done t)]
      [:span {:style {:margin-left "5px" :margin-right "5px"}} "/"]
      [:span (::t/estimate t)]]]]])

(defn lane [state]
  (let [selected-id @(subscribe [:selected])]
    [:div.column
     {:key (::l/id state)
      :style {:border-left-color "#dbdbdb"
              :border-left-style "solid"
              :border-left-width "1px"}}
     [:div
      [:h1.title (::l/name state)]
      [:ul
        (for [t (::db/tasks state)]
          [:li
           {:on-click #(dispatch [:select-task (::t/id t)])
            :key (::t/id t)}
           [task t (= selected-id (::t/id t))]])]]]))

(defn pomodoro-status []
  [:div
   [:span "09"]
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

(defn lanes []
  (let [backlog-todo @(subscribe [:lanes])
        done @(subscribe [:done])]
    (conj (map (fn [l] [lane l]) backlog-todo)
          [lane {::l/id 0 ::l/name "done" ::db/tasks (::db/tasks done)}])))

(defn main []
  [:div.container
   [status-bar]
   [:div.tabs
    [:ul
     [:li.is-active [:a "Tasks"]]
     [:li [:a "Statistics"]]]]

   [:div.columns
    [:div.column
     [new-task-form]]]

   [:div.columns
    (concat
     (for [l @(subscribe [:lanes])]
      [lane l])
     [[lane {::l/id 0 ::l/name "done" ::db/tasks (::db/tasks @(subscribe [:done]))}]])]])
