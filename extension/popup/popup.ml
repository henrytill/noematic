open Brr
open Fut.Syntax
module Runtime = Chrome.Runtime
module Port = Chrome.Runtime.Port
module Tabs = Chrome.Tabs

module State : sig
  type t = {
    uri : Uri.t option;
    tab_id : int;
  }

  val make : Chrome.Tab.t -> t
  val to_jv : t -> Jv.t
end = struct
  type t = {
    uri : Uri.t option;
    tab_id : int;
  }

  let is_web uri =
    let scheme = Uri.scheme uri in
    match Jstr.to_string scheme with
    | "http" | "https" -> true
    | _ -> false

  let make tab =
    let open Chrome in
    let uri = Tab.url tab in
    let tab_id = Tab.id tab in
    { uri = (if is_web uri then Some uri else None); tab_id }

  let to_jv t =
    let uri : Jv.t = Option.fold ~some:Uri.to_jv ~none:Jv.null t.uri in
    Jv.obj [| ("uri", uri); ("tab_id", Jv.of_int t.tab_id) |]
end

let search_handler () =
  let url = "/search/index.html" in
  let _ = Chrome.tabs |> Tabs.create url in
  Window.close G.window

let save_handler runtime state =
  let message = Jv.obj [| ("action", Jv.of_string "save"); ("key", State.to_jv state) |] in
  let go runtime message =
    let+ send_fut = Runtime.send_message message runtime in
    match send_fut with
    | Error err -> Console.error [ Jv.Error.message err ]
    | Ok res -> Console.(log [ res ])
  in
  let _ = go runtime message in
  Window.close G.window

let create_button bid class_name text ~on_click =
  let button = El.button ~d:G.document ~at:At.[ id bid; class' class_name ] [ El.txt' text ] in
  ignore (Ev.listen Ev.click on_click (El.as_target button));
  button

let abbreviate_uri width uri =
  (* If length of uri is greater than width, then trim to width and add ellipsis *)
  let uri_len = String.length uri in
  if uri_len > width then
    let ellipsis = "..." in
    let uri = String.sub uri 0 width in
    String.cat uri ellipsis
  else
    uri

let render runtime state =
  let main_div = Document.find_el_by_id G.document (Jstr.v "main") in
  let open State in
  (* add origin to main div *)
  (match state.uri with
  | None -> ()
  | Some uri ->
      let uri_str = Uri.to_jstr uri |> Jstr.to_string in
      let uri_str = abbreviate_uri 50 uri_str in
      let origin_div =
        El.div ~at:At.[ id (Jstr.v "origin"); class' (Jstr.v "panel") ] El.[ txt' uri_str ]
      in
      El.append_children (Option.get main_div) [ origin_div ]);
  (* create footer *)
  let footer = El.footer [] in
  let footer_button = Jstr.v "footer-button" in
  (* add cancel button to footer *)
  let cancel = Jstr.v "cancel" in
  let on_click _ = Window.close G.window in
  let cancel_button = create_button cancel footer_button "Cancel" ~on_click in
  El.append_children footer [ cancel_button ];
  (* add search button to footer *)
  let search = Jstr.v "search" in
  let on_click _ = search_handler () in
  let search_button = create_button search footer_button "Open Search..." ~on_click in
  El.append_children footer [ search_button ];
  (* add save button to footer *)
  let save = Jstr.v "save" in
  let on_click _ = save_handler runtime state in
  let save_button = create_button save footer_button "Save" ~on_click in
  if Option.is_none state.uri then
    El.(set_at At.Name.disabled) (Some (Jstr.v "true")) save_button;
  El.append_children footer [ save_button ];
  (* add footer to main div *)
  El.append_children (Option.get main_div) [ footer ]

let main () =
  let runtime = Chrome.runtime in
  let+ active = Chrome.tabs |> Tabs.active in
  match active with
  | Error err -> Console.error [ Jv.Error.message err ]
  | Ok [| res |] -> render runtime (State.make res)
  | Ok _ -> Console.(error [ str "Unexpected number of tabs" ])

let () = ignore (main ())
