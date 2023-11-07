open Brr
module Runtime = Chrome.Runtime
module Event = Chrome.Runtime.Event
module Port = Chrome.Runtime.Port

let handle_disconnects connected port =
  connected := List.filter (fun p -> Port.equal p (Port.of_jv port)) !connected;
  Console.log [ Jstr.v "Disconnected"; port ]

let handle_message host_port message =
  Port.post_message host_port message;
  Console.log [ Jstr.v "Received message: "; message ]

let listener connected host_port port =
  connected := port :: !connected;
  Port.Event.add_listener (Port.on_message port) (handle_message host_port);
  Port.Event.add_listener (Port.on_disconnect port) (handle_disconnects connected)

let connect_to_host runtime = Runtime.connect_native runtime "com.github.henrytill.noematic"

let () =
  let connected_ports = ref [] in
  Console.log [ Jstr.v "Hello from the background" ];
  let runtime = Chrome.runtime in
  let host_port = connect_to_host runtime in
  let on_connect = Runtime.on_connect runtime in
  Event.add_listener on_connect (listener connected_ports host_port)
