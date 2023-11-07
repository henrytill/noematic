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
end

let v = Jv.get Jv.global "chrome"
let tabs = Jv.get v "tabs"

module Runtime = struct
  type t = Jv.t

  module Port = struct
    type t = Jv.t

    let equal = Jv.strict_equal
    let of_jv = Fun.id

    module Event = struct
      type t = Jv.t

      let add_listener t f = ignore (Jv.call t "addListener" [| Jv.callback ~arity:1 f |])
    end

    let post_message t msg = ignore (Jv.call t "postMessage" [| msg |])
    let on_message t = Jv.get t "onMessage"
    let on_disconnect t = Jv.get t "onDisconnect"
  end

  let connect t =
    let port = Jv.call t "connect" [||] in
    port

  module Event = struct
    type t = Jv.t

    let add_listener t f = ignore (Jv.call t "addListener" [| Jv.callback ~arity:1 f |])
  end

  let on_connect t = Jv.get t "onConnect"
end

let runtime = Jv.get v "runtime"
