module Main exposing (..)

-- Press buttons to increment and decrement a counter.
--
-- Read how it works:
--   https://guide.elm-lang.org/architecture/buttons.html
--


import Browser exposing (Document)
--import Platform.Cmd exposing (Cmd)
--import Platform.Sub exposing (Sub)
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)
import Time exposing (every, Posix)
import Http
import Json.Decode as D
import Time

-- MAIN


main =
  Browser.document { init = init, subscriptions = subscriptions, update = update, view = view }



-- MODEL

type alias Id = Int

type alias Model =
  { currentTask : Maybe CurrentTask
  , loading: Bool
  }

type alias CurrentTask =
  { id: Id
  , task: Task
  , startedAt: Posix
  }

type alias Task =
  { id: Id
  , laneId: Id
  , priority: Id
  , summary: String
  , createdAt: Posix
  , updatedAt: Posix
  }


init : () -> (Model, Cmd Msg)
init _ = ({ currentTask = Nothing, loading = False }, Cmd.none)



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
  D.map6 Task
    (D.field "id" D.int)
    (D.field "laneId" D.int)
    (D.field "priority" D.int)
    (D.field "summary" D.string)
    (D.field "createdAt" posix)
    (D.field "updatedAt" posix)

decodeCurrentTask : D.Decoder CurrentTask
decodeCurrentTask =
  D.map3 CurrentTask
    (D.field "id" D.int)
    (D.field "task" decodeTask)
    (D.field "startedAt" posix)

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
      CurrentTaskFailure "itimeout"

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
      (model
      , Http.get
          { url = "/api/current"
          , expect = expectJson handleCurrentTask decodeCurrentTask
          }
      )

    CurrentTaskSuccess task ->
      ({ model | currentTask = Just task }, Cmd.none)

    CurrentTaskFailure message ->
      (model, Cmd.none)

    CurrentTaskNotFound ->
      (model, Cmd.none)

-- VIEW


view : Model -> Document Msg
view model =
  Document 
    "ly"
     [
       div []
           [ div [] [ text (Maybe.withDefault "" (Maybe.map (\t -> t.task.summary) model.currentTask))]
           ]
     ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = every 1000 Tick
