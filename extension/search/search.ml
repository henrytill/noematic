open Brr
open Fut.Syntax
module Runtime = Chrome.Runtime

let attach_search_listener listener =
  let search_form = Document.find_el_by_id G.document (Jstr.v "search-form") in
  match search_form with
  | None -> ()
  | Some seach_form -> ignore (Ev.listen Brr_io.Form.Ev.submit listener (El.as_target seach_form))

let send_query runtime query =
  let+ fut = Runtime.(runtime |> send_message (Jv.obj [| ("search", Jv.of_jstr query) |])) in
  match fut with
  | Error err -> Console.error [ Jv.Error.message err ]
  | Ok response -> Console.log [ response ]

let listener runtime e =
  Ev.prevent_default e;
  let search_input = Document.find_el_by_id G.document (Jstr.v "search-input") in
  let query = search_input |> Option.map (fun el -> El.(prop Prop.value el)) in
  match query with
  | None -> ()
  | Some query when Jstr.is_empty query -> ()
  | Some query -> Fut.await (send_query runtime query) ignore

let () =
  let runtime = Chrome.runtime in
  attach_search_listener (listener runtime)
