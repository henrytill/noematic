open Brr
module Runtime = Chrome.Runtime
module Port = Chrome.Runtime.Port

let native_messaging_host = "com.github.henrytill.noematic"
let handle_host_message message = Console.(log [ str "Received host message: "; message ])
let handle_host_disconnect _ = Console.(log [ str "Host disconnected" ])

(* https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/#native-messaging-client *)
let connect_host runtime =
  let host_port = Runtime.(runtime |> connect_native native_messaging_host) in
  Port.(
    host_port |> on_message |> On_message.add_listener handle_host_message;
    host_port |> on_disconnect |> On_disconnect.add_listener handle_host_disconnect);
  host_port

let message_listener _host_port request sender send_response =
  Console.(log [ str "request"; request; str " sender"; sender ]);
  let response = Jv.obj [| ("response", Jv.of_string "Hello from the background") |] in
  ignore (Jv.apply send_response [| response |]);
  Jv.of_bool true

let () =
  Console.(log [ str "Hello from the background" ]);
  let runtime = Chrome.runtime in
  let host_port = connect_host runtime in
  Runtime.(runtime |> on_message |> On_message.add_listener (message_listener host_port))
