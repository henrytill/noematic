open Brr
open Fut.Syntax
module Runtime = Chrome.Runtime

let scrape_page () =
  let open Brr in
  let title = Document.title G.document |> Jv.of_jstr in
  let body = Document.body G.document |> El.to_jv in
  let inner_text = Jv.get body "innerText" in
  (title, inner_text)

let send runtime request send_response =
  let message_type = ("type", Jv.of_string "response") in
  let message_action = ("action", Jv.of_string "save") in
  let+ send_fut = Runtime.send_message request runtime in
  match send_fut with
  | Error err ->
      let result = ("result", Jv.of_jstr (Jv.Error.message err)) in
      Jv.apply send_response [| Jv.obj [| message_type; message_action; result |] |]
  | Ok res ->
      let result = ("result", res) in
      Jv.apply send_response [| Jv.obj [| message_type; message_action; result |] |]

let handle_requests runtime request _sender send_response =
  let title, inner_text = scrape_page () in
  let payload = Jv.get request "payload" in
  Jv.set payload "title" title;
  Jv.set payload "body" inner_text;
  let _ = send runtime request send_response in
  Jv.of_bool true

let listener runtime request _sender send_response =
  match Jv.get request "type" |> Jv.to_string with
  | "ping" ->
      let response = Jv.obj [| ("type", Jv.of_string "pong") |] in
      ignore (Jv.apply send_response [| response |]);
      Jv.of_bool true
  | _ -> handle_requests runtime request _sender send_response

let () =
  let runtime = Chrome.runtime in
  Runtime.(runtime |> on_message |> On_message.add_listener (listener runtime));
  Console.(log [ str "Noematic scrape handler installed" ])
