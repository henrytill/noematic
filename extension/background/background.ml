open Brr
module Runtime = Chrome.Runtime
module Port = Chrome.Runtime.Port

let native_messaging_host = "com.github.henrytill.noematic"

let handle_host_message response_map message =
  let correlation_id = Jv.to_string (Jv.get message "correlationId") in
  match Hashtbl.find_opt response_map correlation_id with
  | None -> Console.(log [ str "No matching request for correlation ID"; str correlation_id ])
  | Some send_response ->
      ignore (Jv.apply send_response [| message |]);
      Hashtbl.remove response_map correlation_id

let handle_host_disconnect _ = Console.(log [ str "Host disconnected" ])

(* https://developer.chrome.com/docs/extensions/mv3/nativeMessaging/#native-messaging-client *)
let connect_host runtime response_map =
  let host_port = Runtime.(runtime |> connect_native native_messaging_host) in
  Port.(
    host_port |> on_message |> On_message.add_listener (handle_host_message response_map);
    host_port |> on_disconnect |> On_disconnect.add_listener handle_host_disconnect);
  host_port

let generate_uuid () =
  let crypto = Jv.get Jv.global "crypto" in
  let random_uuid = Jv.call crypto "randomUUID" [||] in
  Jv.to_string random_uuid

let message_listener response_map host_port request sender send_response =
  Console.(log [ str "request"; request; str " sender"; sender ]);
  let correlation_id = generate_uuid () in
  Jv.set request "correlationId" (Jv.of_string correlation_id);
  Hashtbl.add response_map correlation_id send_response;
  Port.post_message host_port request;
  Jv.of_bool true

let () =
  Console.(log [ str "Hello from the background" ]);
  let response_map = Hashtbl.create 10 in
  let runtime = Chrome.runtime in
  let host_port = connect_host runtime response_map in
  let listener = message_listener response_map host_port in
  Runtime.(runtime |> on_message |> On_message.add_listener listener)
