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
  , currentTask : Maybe CurrentTask
  , errorMsg : Maybe String
  , loading: Bool
  }

type alias CurrentTask =
  { id: Id
  , task: Task
  , startedAt: Posix
  , durationMin: Int
  }

type alias Task =
  { id: Id
  , laneId: Id
  , priority: Id
  , summary: String
  , estimate: Int
  , createdAt: Posix
  , updatedAt: Posix
  }


init : () -> (Model, Cmd Msg)
init _ =
  (
    { now = Time.millisToPosix 0
    , currentTask = Nothing
    , errorMsg = Nothing
    , loading = False
    }
  , Cmd.none)



-- UPDATE


type Msg
  = Tick Posix
  | CurrentTaskSuccess CurrentTask
  | CurrentTaskFailure String
  | CurrentTaskNotFound

posix : D.Decoder Posix
posix = D.map Time.millisToPosix D.int

decodeTask : D.Decoder Task
decodeTask =
  D.map7 Task
    (D.field "id" D.int)
    (D.field "lane_id" D.int)
    (D.field "priority" D.int)
    (D.field "summary" D.string)
    (D.field "estimate" D.int)
    (D.field "created_at" posix)
    (D.field "updated_at" posix)

decodeCurrentTask : D.Decoder CurrentTask
decodeCurrentTask =
  D.map4 CurrentTask
    (D.field "id" D.int)
    (D.field "task" decodeTask)
    (D.field "started_at" posix)
    (D.field "duration_min" D.int)

handleCurrentTask : Result Http.Error CurrentTask -> Msg
handleCurrentTask result =
  case result of
    Ok t ->
      CurrentTaskSuccess t

    Err (Http.BadStatus status) ->
      case status of
        404 ->
          CurrentTaskNotFound
        other ->
          CurrentTaskFailure ("Error" ++ String.fromInt other)

    Err (Http.BadUrl url) ->
      CurrentTaskFailure url

    Err Http.Timeout ->
      CurrentTaskFailure "timeout"

    Err Http.NetworkError ->
      CurrentTaskFailure "network error"

    Err (Http.BadBody body) ->
      CurrentTaskFailure ("bad body: " ++ body)

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
          { url = "/api/current"
          , expect = expectJson handleCurrentTask decodeCurrentTask
          }
      )

    CurrentTaskSuccess task ->
      ({ model | currentTask = Just task, errorMsg = Nothing }, Cmd.none)

    CurrentTaskFailure message ->
      ({ model | currentTask = Nothing, errorMsg = Just message }, Cmd.none)

    CurrentTaskNotFound ->
      ({ model | currentTask = Nothing, errorMsg = Nothing }
      , case model.currentTask of
        Just(_) -> notify "pomodoro completed"
        Nothing -> Cmd.none
      )

-- VIEW

padZero : Int -> String
padZero x =
  if x >= 10 then String.fromInt x else "0" ++ String.fromInt x

timer : Model -> Html Msg
timer model =
  case model.currentTask of
    Nothing ->
      text "00:00"
    Just currentTask ->
      let
        duration = (Time.posixToMillis model.now) - (Time.posixToMillis currentTask.startedAt)
        maxSeconds = currentTask.durationMin * 60
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
           [ div [] [ text (Maybe.withDefault "" (Maybe.map (\t -> t.task.summary) model.currentTask))]
           , div [] [ timer model]
           , div [] [ text (Maybe.withDefault "" model.errorMsg)]
           ]
     ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = every 1000 Tick
