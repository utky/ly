(defproject ly "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :min-lein-version "2.0.0"
  :dependencies [[org.clojure/clojure "1.10.0"]
                 [duct/core "0.8.0"]
                 [duct/module.ataraxy "0.3.0"]
                 [duct/module.cljs "0.4.1"]
                 [duct/module.logging "0.4.0"]
                 [duct/module.web "0.7.0"]
                 [duct/module.sql "0.6.0"]
                 [com.h2database/h2 "1.4.200"]
                 [honeysql "1.0.444"]
                 [duct/database.sql "0.1.0"]
                 [duct/database.sql.hikaricp "0.4.0"]

                 [reagent "1.0.0-alpha2"]
                 [re-frame "1.1.1"]
                 [day8.re-frame/http-fx "0.2.1"]
                 [cljs-ajax "0.8.1"]
                 [re-pressed "0.3.1"]]
  :plugins [[duct/lein-duct "0.12.1"]
            [lein-cljfmt "0.7.0"]]
  :main ^:skip-aot ly.main
  :resource-paths ["resources" "target/resources"]
  :prep-tasks     ["javac" "compile" ["run" ":duct/compiler"]]
  :middleware     [lein-duct.plugin/middleware]
  :profiles
  {:dev  [:project/dev :profiles/dev]
   :repl {:prep-tasks   ^:replace ["javac" "compile"]
          :dependencies [[cider/piggieback "0.4.0"]]
          :repl-options {:init-ns user, :nrepl-middleware [cider.piggieback/wrap-cljs-repl]}}
   :uberjar {:aot :all}
   :profiles/dev {}
   :project/dev  {:source-paths   ["dev/src"]
                  :resource-paths ["dev/resources"]
                  :dependencies   [[integrant/repl "0.3.1"]
                                   [eftest "0.5.7"]
                                   [kerodon "0.9.0"]]}})
