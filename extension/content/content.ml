open Brr
open Fut.Syntax
module Runtime = Chrome.Runtime

let scrape_page () =
  let title = Document.title G.document |> Jv.of_jstr in
  let body = Document.body G.document |> El.to_jv in
  let inner_text = Jv.get body "innerText" in
  (title, inner_text)

let handle_save_requests runtime request _sender send_response =
  let title, inner_text = scrape_page () in
  let payload = Jv.get request "payload" in
  Jv.set payload "title" title;
  Jv.set payload "body" inner_text;
  let _ =
    begin
      let+ send_fut = Runtime.send_message request runtime in
      match send_fut with
      | Error err ->
          let res = Jv.obj [| ("error", Jv.of_jstr (Jv.Error.message err)) |] in
          Jv.set res "action" (Jv.of_string "saveResult");
          Jv.apply send_response [| res |]
      | Ok res ->
          Jv.set res "action" (Jv.of_string "saveResult");
          Jv.apply send_response [| res |]
    end
  in
  Jv.of_bool true

let handle_ping_requests send_response =
  let response = Jv.obj [| ("action", Jv.of_string "pong") |] in
  ignore (Jv.apply send_response [| response |]);
  Jv.of_bool true

let listener runtime request _sender send_response =
  match Jv.get request "action" |> Jv.to_string with
  | "ping" -> handle_ping_requests send_response
  | "saveRequest" -> handle_save_requests runtime request _sender send_response
  | _ -> Jv.of_bool false

let () =
  let runtime = Chrome.runtime in
  Runtime.(runtime |> on_message |> On_message.add_listener (listener runtime));
  Console.(log [ str "Noematic scrape handler installed" ])
