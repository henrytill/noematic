open Brr
open Fut.Syntax

module State : sig
  type t

  val make : Chrome.Tab.t -> t
  val uri : t -> Uri.t option
  val tab : t -> Chrome.Tab.t
end = struct
  type t = {
    tab : Chrome.Tab.t;
    uri : Uri.t option;
  }

  let is_web uri =
    let scheme = Uri.scheme uri in
    match Jstr.to_string scheme with
    | "http" | "https" -> true
    | _ -> false

  let make tab =
    let open Chrome in
    let uri = Tab.url tab in
    { tab; uri = (if is_web uri then Some uri else None) }

  let uri t = t.uri
  let tab t = t.tab
end

let search_handler () =
  let path = "/search/index.html" in
  let _ = Chrome.(tabs |> Tabs.create path) in
  Window.close G.window

let check_content_script_active tab : bool Fut.or_error =
  let ping_message = Jv.obj [| ("action", Jv.of_string "ping") |] in
  let+ ping_result = Chrome.(tabs |> Tabs.send_message tab ping_message) in
  match ping_result with
  | Ok _ -> Ok true
  | Error _ -> Ok false

let inject_content_script tab =
  let+ inject_results =
    Chrome.(
      let tab_id = Tab.id tab in
      let files = [ "./content/content.bc.js" ] in
      scripting |> Scripting.execute_script ~tab_id ~files)
  in
  match inject_results with
  | Ok [| _ |] as ret -> ret
  | Ok _ -> Error (Jv.Error.v (Jstr.v "Unexpected number of results"))
  | Error _ as err -> err

let maybe_inject_content_script tab =
  let* content_script_active = check_content_script_active tab in
  match content_script_active with
  | Ok true -> Fut.ok [||]
  | Ok false -> inject_content_script tab
  | Error _ as err -> Fut.return err

let save_handler state =
  let go uri =
    let payload = Jv.obj [| ("uri", Uri.to_jv uri) |] in
    let message = Jv.obj [| ("action", Jv.of_string "saveRequest"); ("payload", payload) |] in
    ignore
      begin
        let tab = State.tab state in
        let* _ = maybe_inject_content_script tab in
        let+ send_result = Chrome.(tabs |> Tabs.send_message tab message) in
        match send_result with
        | Error err -> Console.error [ Jv.Error.message err ]
        | Ok res -> Console.(log [ str "response"; res ])
      end
  in
  Option.iter go (State.uri state)

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

let render state =
  let main_div = Document.find_el_by_id G.document (Jstr.v "main") in
  (* add origin to main div *)
  (match State.uri state with
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
  let on_click _ = save_handler state in
  let save_button = create_button save footer_button "Save" ~on_click in
  if Option.is_none (State.uri state) then
    El.(set_at At.Name.disabled) (Some (Jstr.v "true")) save_button;
  El.append_children footer [ save_button ];
  (* add footer to main div *)
  El.append_children (Option.get main_div) [ footer ]

let main () : unit Fut.or_error =
  let+ active_tabs = Chrome.(tabs |> Tabs.active) in
  match active_tabs with
  | Ok [| active_tab |] -> Ok (render (State.make active_tab))
  | Ok _ -> Error (Jv.Error.v (Jstr.v "Unexpected number of active tabs"))
  | Error _ as err -> err

let () =
  Fut.await (main ()) @@ function
  | Ok () -> Console.(log [ str "Loaded" ])
  | Error err -> Console.(error [ Jv.Error.message err ])
