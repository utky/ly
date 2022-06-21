port module Main exposing (..)

import Browser exposing (Document)
import Html exposing (Html, a, div, span, text, header, main_, section, ul, li)
import Html.Attributes exposing (class, href, style)
import Html.Events exposing (onClick)
import Time exposing (every, toYear, toMonth, toDay, toHour, toMinute, toSecond, Posix)
import Http
import Json.Decode as D
import Time
import Task
import Svg exposing (svg, rect, title, desc, g, line, text_)
import Svg.Attributes as Svg
import Dict exposing (Dict)

-- MAIN


main : Program () Model Msg
main =
  Browser.document
    { init = init
    , subscriptions = subscriptions
    , update = update
    , view = view
    }

port notify : String -> Cmd msg

-- MODEL

type alias Id = Int

type alias Model =
  { now : Posix
  , timeZone : Maybe Time.Zone
  , timer : Maybe Timer
  , pomodoroDaily: Maybe Measurements
  , errorMsg : Maybe String
  , loading: Bool
  }

type alias Timer =
  { id: Id
  , timer_type: Int
  , label: String
  , startedAt: Posix
  , durationMin: Int
  }

type alias Measuremet =
  { time: Posix
  , value: Float
  }

type alias Measurements = 
  { instrument: String
  , labels: Dict String String
  , data: List Measuremet
  }

type alias SummaryContext =
  { maxPomodoroCount: Int
  , summaryCount: Int
  , timeZone: Time.Zone
  , barWidthPercentage: Float
  }

type alias ScaleTrans =
  { coeff: Float
  , const: Float
  }

scaleTrans : ScaleTrans -> Float -> Float
scaleTrans trans value =
  trans.coeff * value + trans.const

-- take domain (min, max), range (min, max) then returns Scale transformer function
-- ((value - dMin) * coeff) + rMin
-- (value * coeff) - (dMin * coeff) + rMin
makeScaleTrans : (Float, Float) -> (Float, Float) -> ScaleTrans
makeScaleTrans (domainMin, domainMax) (rangeMin, rangeMax) =
  let
      domainDistance = domainMax - domainMin
      rangeDistance = rangeMax - rangeMin
      coeff = rangeDistance / domainDistance
  in
    { coeff = coeff
    , const = -1 * domainMin * coeff + rangeMin
    }  

type alias ChartAxis =
  { label: String
  -- , domainMin: Float -- 
  -- , domainMax: Float 
  -- , rangeMin: Float -- minimum value of window size
  -- , rangeMax: Float -- maximum value of window size
  -- , scaleView: Float -> String
  }

type alias BarChart =
  { title: String
  , xAxis: ChartAxis
  , yAxis: ChartAxis
  , margin: Int
  , width: Int
  , height: Int
  , data: List (Posix, Float)
  }

svgTranslate : Float -> Float -> String
svgTranslate x y =
  "translate(" ++ (String.fromFloat x) ++ "," ++ (String.fromFloat y) ++ ")"

-- TODO
-- xScale : BarChart -> x -> Float
-- xScale barChart xValue =
--   let
--     xMax = barChart.width - (2 * barChart.margin)
--     -- value : 0 .. 5 .. 10
--     -- axis  : 0 .. 150 .. 300
--     ratio = xValue / (barChart.xAxis.max)
--     xPoint = xMax * ratio
--   in
--     xPoint

--renderYAxisScale : BarChart -> Html Msg
--renderYAxisScale barChar =
--  let
--    domainSpace = barChar.yAxis.domainMax - barChar.yAxis.domainMin
--    valuePosition y = (y - barChar.yAxis.domainMin) / domainSpace
--    tick value =
--      g [ class "tick", Svg.opacity "1", Svg.transform (svgTranslate 0.0 0.0) ]
--        [ line [Svg.stroke "black", Svg.x2 "-6"] []
--        , text_ [Svg.fill "black", Svg.x "-9", Svg.dy "0.32em" ] []
--        ]
--    ticks = []
--    axisLine = line [ Svg.x1 "10%", Svg.y1 "90%", Svg.x2 "90%", Svg.y2 "90%", Svg.stroke "black" ] []
--  in
--    g [ Svg.fill "none", Svg.fontSize "10", Svg.fontFamily "sans-serif", Svg.textAnchor "end" ]
--      (List.concat [ [ axisLine ], ticks ])

testPomodoroDaily : Measurements
testPomodoroDaily =
  { instrument = "pomodoro.daily"
  , labels = Dict.empty
  , data =
      [ { time = Time.millisToPosix 1631404800000, value = 1.0 }
      , { time = Time.millisToPosix 1631404886400, value = 2.0 }
      , { time = Time.millisToPosix 1631404972800, value = 3.0 }
      , { time = Time.millisToPosix 1631405059200, value = 4.0 }
      ]
  }

init : () -> (Model, Cmd Msg)
init _ =
  ( { now = Time.millisToPosix 0
    , timeZone = Nothing
    , timer = Nothing
    , pomodoroDaily = Nothing
    , errorMsg = Nothing
    , loading = False
    }
    , Task.perform SetTimeZone Time.here
  )



-- UPDATE


type Msg
  = Tick Posix
  | TimerSuccess Timer
  | TimerFailure String
  | TimerNotFound
  | MeasurementsSuccess Measurements
  | MeasurementsFailure String
  | SetTimeZone Time.Zone

posix : D.Decoder Posix
posix = D.map Time.millisToPosix D.int

decodeTimer : D.Decoder Timer
decodeTimer =
  D.map5 Timer
    (D.field "id" D.int)
    (D.field "timer_type" D.int)
    (D.field "label" D.string)
    (D.field "started_at" posix)
    (D.field "duration_min" D.int)

decodeMeasurement : D.Decoder Measuremet
decodeMeasurement =
    D.map2 Measuremet
        (D.index 0 posix) 
        (D.index 1 D.float)

decodeMeasurements : D.Decoder Measurements
decodeMeasurements =
  D.map3 Measurements
    (D.field "instrument" D.string)
    (D.field "labels"(D.dict D.string))
    (D.field "data" (D.list decodeMeasurement))

handleMeasurements : Result Http.Error Measurements -> Msg
handleMeasurements result =
  case result of
    Ok s ->
      MeasurementsSuccess s

    Err _ ->
      MeasurementsFailure "failed"

handleTimer : Result Http.Error Timer -> Msg
handleTimer result =
  case result of
    Ok t ->
      TimerSuccess t

    Err (Http.BadStatus status) ->
      case status of
        404 ->
          TimerNotFound
        other ->
          TimerFailure ("Error" ++ String.fromInt other)

    Err (Http.BadUrl url) ->
      TimerFailure url

    Err Http.Timeout ->
      TimerFailure "timeout"

    Err Http.NetworkError ->
      TimerFailure "network error"

    Err (Http.BadBody body) ->
      TimerFailure ("bad body: " ++ body)

expectJson : (Result Http.Error a -> msg) -> D.Decoder a -> Http.Expect msg
expectJson toMsg decoder =
  Http.expectStringResponse toMsg <|
    \response ->
      case response of
        Http.BadUrl_ url ->
          Err (Http.BadUrl url)

        Http.Timeout_ ->
          Err Http.Timeout

        Http.NetworkError_ ->
          Err Http.NetworkError

        Http.BadStatus_ metadata _ ->
          Err (Http.BadStatus metadata.statusCode)

        Http.GoodStatus_ _ body ->
          case D.decodeString decoder body of
            Ok value ->
              Ok value

            Err err ->
              Err (Http.BadBody (D.errorToString err))

dailySummaryQueryParams : Model -> Time.Posix -> String
dailySummaryQueryParams model now =
  let
    daysBack = 7 * 4
    millisDay = 1000 * 60 * 60 * 24
    start = (Time.posixToMillis now) - (millisDay * daysBack)
    end = (Time.posixToMillis now) + 1
  in
    "start=" ++ String.fromInt start ++ "&end=" ++ String.fromInt end

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Tick now ->
      ({ model | now = now }
      , Cmd.batch
        [
          Http.get
            { url = "/api/timer"
            , expect = expectJson handleTimer decodeTimer
            }
          , Http.get -- updateコスト高そうなので60秒に一回とかにする
            -- timezoneも渡すようにして
            { url = "/api/pomodoro_daily?" ++ (dailySummaryQueryParams model now)
            , expect = Http.expectJson handleMeasurements decodeMeasurements
            }
        ]
      )

    TimerSuccess currentTimer ->
      ({ model | timer = Just currentTimer, errorMsg = Nothing }, Cmd.none)

    TimerFailure message ->
      ({ model | timer = Nothing, errorMsg = Just message }, Cmd.none)

    TimerNotFound ->
      ({ model | timer = Nothing, errorMsg = Nothing }
      , case model.timer of
        Just(_) -> notify "pomodoro completed"
        Nothing -> Cmd.none
      )

    MeasurementsSuccess measurements ->
      if measurements.instrument == "pomodoro.daily" then
        ({ model | pomodoroDaily = Just testPomodoroDaily }, Cmd.none)
      else
        (model, Cmd.none)

    MeasurementsFailure message ->
      ({ model | errorMsg = Just message }, Cmd.none)

    SetTimeZone zone ->
      ({ model | timeZone = Just zone }, Cmd.none)

-- VIEW

padZero : Int -> String
padZero x =
  if x >= 10 then String.fromInt x else "0" ++ String.fromInt x

timer : Model -> Html Msg
timer model =
  case model.timer of
    Nothing ->
      text "00:00"
    Just currentTimer ->
      let
        duration = (Time.posixToMillis model.now) - (Time.posixToMillis currentTimer.startedAt)
        maxSeconds = currentTimer.durationMin * 60
        durationSeconds = duration // 1000
        remainingSeconds = maxSeconds - durationSeconds
        minutes = if 0 <= remainingSeconds then remainingSeconds // 60 else 0
        seconds = if 0 <= remainingSeconds then modBy 60 remainingSeconds else 0
      in
        text <| (padZero minutes) ++ ":" ++ (padZero seconds)

weekdayString : Time.Weekday -> String
weekdayString weekday =
  case weekday of
    Time.Mon -> "月"
    Time.Tue -> "火"
    Time.Wed -> "水"
    Time.Thu -> "木"
    Time.Fri -> "金"
    Time.Sat -> "土"
    Time.Sun -> "日"

monthInt : Time.Month -> Int
monthInt month =
  case month of
    Time.Jan -> 1
    Time.Feb -> 2
    Time.Mar -> 3
    Time.Apr -> 4
    Time.May -> 5
    Time.Jun -> 6
    Time.Jul -> 7
    Time.Aug -> 8
    Time.Sep -> 9
    Time.Oct -> 10
    Time.Nov -> 11
    Time.Dec -> 12

percent : Float -> String
percent v = (String.fromFloat v) ++ "%"

renderAxis : BarChart -> List (Html Msg)
renderAxis barChart =
  let
    datapoints = barChart.data
  in
    -- x axis
    [ g [ ]
        [ g [ ]
            [ line [ Svg.x1 "5%", Svg.y1 "95%", Svg.x2 "95%", Svg.y2 "95%", Svg.stroke "black" ] []
            , Svg.text_
                [ Svg.x "90%", Svg.y "100%" ]
                [ text barChart.xAxis.label ]
            ]
        ]
    -- y axis
    , g [  ]
        [ g [  ]
            [ line [ Svg.x1 "5%", Svg.y1 "5%", Svg.x2 "5%", Svg.y2 "95%", Svg.stroke "black" ] []
            , Svg.text_
                [ Svg.transform "rotate(-90, 20, 150)", Svg.x "5%", Svg.y "50%" ]
                [ text barChart.yAxis.label ]
            ]
        ]
    ]

renderBarChart : BarChart -> Html Msg
renderBarChart barChart =
  let
    margin = String.fromInt barChart.margin
    svgTitle = title [] [ text barChart.title ]
    svgChart = renderAxis barChart
    svgBody = svgTitle :: svgChart
  in
    svg
      [ Svg.width <| String.fromInt barChart.width
      , Svg.height <| String.fromInt barChart.height
      ]
      [ g []
          svgBody
      ]

-- TODO
makePomodoroDailyChart : Model -> Maybe BarChart
makePomodoroDailyChart model =
  let
    yAxis : ChartAxis
    yAxis =
      { label = "Pomodoro"
      -- , scale = \pomo -> pomo + 1
      -- , scaleView = \pomo -> String.fromFloat pomo
      }
    xAxis : Time.Zone -> ChartAxis
    xAxis timeZone =
      { label = "Date"
      --, scale = \t -> Time.millisToPosix <| (86400 * 1000) + (Time.posixToMillis t)
      --, scaleView = \t ->
      --    let
      --      month = Time.toMonth timeZone t
      --      day = Time.toDay timeZone t
      --    in
      --      (String.fromInt <| monthInt month) ++ "/" ++ (String.fromInt day)
      }
    barChart : Time.Zone -> Measurements -> BarChart
    barChart timeZone measurements =
      { title = "Pomodoro Daily"
      , xAxis = xAxis timeZone
      , yAxis = yAxis
      , margin = 10
      , width = 1000
      , height = 300
      , data = List.map (\m -> (m.time, m.value)) measurements.data
      }
  in
    Maybe.andThen (\timeZone -> Maybe.map (barChart timeZone) model.pomodoroDaily) model.timeZone

renderPomodoroDaily : Model -> Html Msg
renderPomodoroDaily model =
  case makePomodoroDailyChart model of
     Just chart -> renderBarChart chart
     Nothing -> div [] [ text "No pomodoro activities" ]

view : Model -> Document Msg
view model =
  Document 
    "ly"
    [ header [ ]
        [ div [ class "pure-menu", class "pure-menu-horizontal", class "pure-menu-fixed" ]
            [ a [ class "pure-menu-heading", href "#" ] [ text "ly" ]
            , ul [ class "pure-menu-list" ]
                [ li [ class "pure-menu-item" ] [ span [ style "padding" ".5em 1em" ] [ timer model ] ]
                , li [ class "pure-menu-item" ] [ span [ style "padding" ".5em 1em" ] [ text (Maybe.withDefault "" (Maybe.map (\t -> t.label) model.timer)) ] ]
                ]
            ]
        ]
    , main_ [ style "width" "100%", style "padding-top" "45px", style "padding-right" "15px", style "padding-left" "15px", style "margin-right" "auto", style "margin-left" "auto" ]
        [ section [ class "pure-g" ] [ div [ class "pure-u-1" ] [ renderPomodoroDaily model ] ]
        , section [ class "pure-g" ] [ div [ class "pure-u-1" ] [ text (Maybe.withDefault "" model.errorMsg) ] ]
        ]
    ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = every 1000 Tick
