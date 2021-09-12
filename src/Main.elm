port module Main exposing (..)

import Browser exposing (Document)
import Html exposing (Html, a, div, span, text, header, main_, section, ul, li)
import Html.Attributes exposing (class, href, style)
import Html.Events exposing (onClick)
import Time exposing (every, Posix)
import Http
import Json.Decode as D
import Time
import Task
import Set exposing (Set)

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
  , dailySummaries: List DailySummary
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

type alias DailySummary =
  { date: Posix
  , taskId: Id
  , pomodoroCount: Int
  , interruptionCount: Int
  }

init : () -> (Model, Cmd Msg)
init _ =
  ( { now = Time.millisToPosix 0
    , timeZone = Nothing
    , timer = Nothing
    , dailySummaries = []
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
  | DailySummarySuccess (List DailySummary)
  | DailySummaryFailure String
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

decodeDailySummary : D.Decoder DailySummary
decodeDailySummary =
  D.map4 DailySummary
    (D.field "date" posix)
    (D.field "task_id" D.int)
    (D.field "pomodoro_count" D.int)
    (D.field "interruption_count" D.int)

handleDailySummaries : Result Http.Error (List DailySummary) -> Msg
handleDailySummaries result =
  case result of
    Ok s ->
      DailySummarySuccess s

    Err _ ->
      DailySummaryFailure "failed"

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
          , Http.get
            -- timezoneも渡すようにして
            { url = "/api/daily_summary?" ++ (dailySummaryQueryParams model now)
            , expect = Http.expectJson handleDailySummaries (D.list decodeDailySummary)
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

    DailySummarySuccess summaries ->
      ({ model | dailySummaries = summaries }, Cmd.none)

    DailySummaryFailure message ->
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

dailySummary : Time.Zone -> DailySummary -> Html Msg
dailySummary zone summary =
  let
    year = Time.toYear zone summary.date
    month = Time.toMonth zone summary.date
    day = Time.toDay zone summary.date
  in
    li []
      [ span [] [ text ((String.fromInt year) ++ "-" ++ padZero (monthInt month) ++ "-" ++ padZero day) ]
      , span [] [ text (String.fromInt summary.taskId) ]
      , span [] [ text (String.fromInt summary.pomodoroCount) ]
      ]

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

dailySummaries : Model -> Html Msg
dailySummaries model =
  let
    summaryItems = 
      case model.timeZone of
        Just timeZone ->
          List.map (dailySummary timeZone) model.dailySummaries
        Nothing ->
          []
  in
    ul [] summaryItems


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
        [ section [ class "pure-g" ] [ div [ class "pure-u-1" ] [ dailySummaries model ] ]
        , section [ class "pure-g" ] [ div [ class "pure-u-1" ] [ text (Maybe.withDefault "" model.errorMsg) ] ]
        ]
    ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = every 1000 Tick
