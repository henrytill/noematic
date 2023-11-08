module Tab = struct
  type t = Jv.t

  let url t =
    let open Brr in
    Uri.v (Jv.to_jstr (Jv.get t "url"))

  let id t = Jv.to_int (Jv.get t "id")
end

module Tabs = struct
  type t = Jv.t

  let active t =
    let params = Jv.obj [| ("currentWindow", Jv.of_bool true); ("active", Jv.of_bool true) |] in
    Fut.of_promise ~ok:Jv.to_jv_array (Jv.call t "query" [| params |])

  let create url t =
    let params = Jv.obj [| ("url", Jv.of_string url) |] in
    Fut.of_promise ~ok:Fun.id (Jv.call t "create" [| params |])
end

let v = Jv.get Jv.global "chrome"
let tabs = Jv.get v "tabs"

module Runtime = struct
  type t = Jv.t

  module Port = struct
    type t = Jv.t

    let equal = Jv.strict_equal

    module On_message = struct
      type t = Jv.t

      let add_listener f t = ignore (Jv.call t "addListener" [| Jv.callback ~arity:1 f |])
    end

    module On_disconnect = struct
      type t = Jv.t

      let add_listener f t = ignore (Jv.call t "addListener" [| Jv.callback ~arity:1 f |])
    end

    let name t = Jv.to_string (Jv.get t "name")
    let post_message t msg = ignore (Jv.call t "postMessage" [| msg |])
    let on_message t = Jv.get t "onMessage"
    let on_disconnect t = Jv.get t "onDisconnect"
  end

  let connect ?name t =
    match name with
    | None -> Jv.call t "connect" [||]
    | Some name ->
        let name = Jv.of_string name in
        let connect_info = Jv.obj [| ("name", name) |] in
        Jv.call t "connect" [| connect_info |]

  let connect_native name t = Jv.call t "connectNative" [| Jv.of_jstr (Jstr.v name) |]
  let send_message msg t = Fut.of_promise ~ok:Fun.id (Jv.call t "sendMessage" [| msg |])

  let send_native_message name msg t =
    Fut.of_promise ~ok:Fun.id (Jv.call t "sendNativeMessage" [| Jv.of_jstr (Jstr.v name); msg |])

  let id t = Jv.to_string (Jv.get t "id")

  module On_connect = struct
    type t = Jv.t

    let add_listener f t = ignore (Jv.call t "addListener" [| Jv.callback ~arity:1 f |])
  end

  let on_connect t = Jv.get t "onConnect"

  module Message_sender = struct
    type t = Jv.t

    let tab t = Jv.find t "tab"
  end

  module On_message = struct
    type t = Jv.t

    let add_listener f t = ignore (Jv.call t "addListener" [| Jv.callback ~arity:3 f |])
  end

  let on_message t = Jv.get t "onMessage"
end

let runtime = Jv.get v "runtime"
