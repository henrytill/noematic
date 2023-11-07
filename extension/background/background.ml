open Brr
module Runtime = Chrome.Runtime
module Event = Chrome.Runtime.Event
module Port = Chrome.Runtime.Port

let native_messaging_host = "com.github.henrytill.noematic"

let handle_disconnects connected port =
  connected := List.filter (fun p -> Port.equal p (Port.of_jv port)) !connected;
  Console.(log [ str "Disconnected"; port ])

let handle_message host_port message =
  Port.post_message host_port message;
  Console.(log [ str "Received message: "; message ])

let listener connected host_port port =
  connected := port :: !connected;
  Port.Event.add_listener (Port.on_message port) (handle_message host_port);
  Port.Event.add_listener (Port.on_disconnect port) (handle_disconnects connected)

let () =
  let connected_ports = ref [] in
  Console.(log [ str "Hello from the background" ]);
  let runtime = Chrome.runtime in
  let host_port = Runtime.connect_native runtime native_messaging_host in
  let on_connect = Runtime.on_connect runtime in
  Event.add_listener on_connect (listener connected_ports host_port)
