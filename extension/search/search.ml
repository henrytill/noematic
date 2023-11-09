open Brr
open Fut.Syntax
module Runtime = Chrome.Runtime

let attach_search_listener listener =
  let search_form = Document.find_el_by_id G.document (Jstr.v "search-form") in
  match search_form with
  | None -> Console.(error [ str "Could not find search-form" ])
  | Some seach_form -> ignore (Ev.listen Brr_io.Form.Ev.submit listener (El.as_target seach_form))

let send_query runtime query =
  let payload = Jv.obj [| ("query", Jv.of_jstr query) |] in
  let message =
    Jv.obj
      [|
        ("type", Jv.of_string "request"); ("action", Jv.of_string "search"); ("payload", payload);
      |]
  in
  let+ fut = Runtime.(runtime |> send_message message) in
  match fut with
  | Error err -> Console.error [ Jv.Error.message err ]
  | Ok response -> Console.(log [ str "response"; response ])

let listener runtime e =
  Ev.prevent_default e;
  let search_input = Document.find_el_by_id G.document (Jstr.v "search-input") in
  let query = Option.map El.(prop Prop.value) search_input in
  match query with
  | None -> Console.(error [ str "Could not find search-input" ])
  | Some query when Jstr.is_empty query -> ()
  | Some query -> ignore (send_query runtime query)

let () =
  let runtime = Chrome.runtime in
  attach_search_listener (listener runtime)
