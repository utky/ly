module Main exposing (..)

-- Press buttons to increment and decrement a counter.
--
-- Read how it works:
--   https://guide.elm-lang.org/architecture/buttons.html
--


import Browser exposing (Document)
import Platform.Cmd exposing (Cmd)
import Platform.Sub exposing (Sub)
import Html exposing (Html, button, div, text)
import Html.Events exposing (onClick)
import Time exposing (every)


-- MAIN


main =
  Browser.document { init = init, subscriptions = subscriptions, update = update, view = view }



-- MODEL


type alias Model = Int


init : () -> (Model, Cmd Msg)
init _ = (0, Cmd.none)



-- UPDATE


type Msg
  = Increment
  | Decrement


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Increment ->
      (model + 1, Cmd.none)

    Decrement ->
      (model - 1, Cmd.none)



-- VIEW


view : Model -> Document Msg
view model =
  Document 
    "ly"
     [
       div []
           [ button [ onClick Decrement ] [ text "--" ]
           , div [] [ text (String.fromInt model) ]
           , button [ onClick Increment ] [ text "++" ]
           ]
     ]
  


subscriptions : Model -> Sub Msg
subscriptions _ = Sub.none
