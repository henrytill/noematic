open Brr
module Runtime = Chrome.Runtime
module Port = Chrome.Runtime.Port

let native_messaging_host = "com.github.henrytill.noematic"
let handle_host_message message = Console.(log [ str "Received host message: "; message ])
let handle_host_disconnect _ = Console.(log [ str "Host disconnected" ])

(* https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/#native-messaging-client *)
let connect_host runtime =
  let host_port = Runtime.connect_native runtime native_messaging_host in
  Port.(
    host_port |> on_message |> Event.add_listener handle_host_message;
    host_port |> on_disconnect |> Event.add_listener handle_host_disconnect);
  host_port

let handle_popup_message host_port message =
  Port.post_message host_port message;
  Console.(log [ str "Received message: "; message ])

let handle_search_message _ message = Console.(log [ str "Received search message: "; message ])

let handle_disconnects connected port =
  connected := List.filter (fun p -> not (Port.equal p (Port.of_jv port))) !connected;
  Console.(log [ str "Disconnected"; port ])

let listener connected host_port port =
  connected := port :: !connected;
  Console.(log [ str "Connected to port:"; str (Port.name port) ]);
  (match Port.name port with
  | "popup" -> Port.(port |> on_message |> Event.add_listener (handle_popup_message host_port))
  | "search" -> Port.(port |> on_message |> Event.add_listener (handle_search_message host_port))
  | _ -> ());
  Port.(port |> on_disconnect |> Event.add_listener (handle_disconnects connected))

let () =
  let connected_ports = ref [] in
  Console.(log [ str "Hello from the background" ]);
  let runtime = Chrome.runtime in
  let host_port = connect_host runtime in
  (* https://developer.chrome.com/docs/extensions/mv3/messaging/#connect *)
  Runtime.(runtime |> on_connect |> Event.add_listener (listener connected_ports host_port))
