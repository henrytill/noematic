module Runtime = Chrome.Runtime
module Port = Chrome.Runtime.Port

let () =
  let name = "search" in
  let port = Chrome.runtime |> Runtime.connect ~name in
  Port.post_message port (Jv.of_string "Hello from search")
