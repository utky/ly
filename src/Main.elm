port module Main exposing (..)

-- Press buttons to increment and decrement a counter.
--
-- Read how it works:
--   https://guide.elm-lang.org/architecture/buttons.html
--


import Browser exposing (Document)
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)
import Time exposing (every, Posix)
import Http
import Json.Decode as D
import Time

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
  , timer : Maybe Timer
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

init : () -> (Model, Cmd Msg)
init _ =
  (
    { now = Time.millisToPosix 0
    , timer = Nothing
    , errorMsg = Nothing
    , loading = False
    }
  , Cmd.none)



-- UPDATE


type Msg
  = Tick Posix
  | TimerSuccess Timer
  | TimerFailure String
  | TimerNotFound

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

        Http.BadStatus_ metadata body ->
          Err (Http.BadStatus metadata.statusCode)

        Http.GoodStatus_ metadata body ->
          case D.decodeString decoder body of
            Ok value ->
              Ok value

            Err err ->
              Err (Http.BadBody (D.errorToString err))

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Tick now ->
      ({ model | now = now }
      , Http.get
          { url = "/api/timer"
          , expect = expectJson handleTimer decodeTimer
          }
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

view : Model -> Document Msg
view model =
  Document 
    "ly"
     [
       div []
           [ div [] [ text (Maybe.withDefault "" (Maybe.map (\t -> t.label) model.timer))]
           , div [] [ timer model]
           , div [] [ text (Maybe.withDefault "" model.errorMsg)]
           ]
     ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = every 1000 Tick
