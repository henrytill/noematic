module Tab : sig
  type t

  val url : t -> Brr.Uri.t
  val id : t -> int
end

module Tabs : sig
  type t

  val active : t -> Tab.t array Fut.or_error
  val create : string -> t -> Tab.t Fut.or_error
  val send_message : Tab.t -> Jv.t -> t -> Jv.t Fut.or_error
end

val tabs : Tabs.t

module Runtime : sig
  type t

  val id : t -> string

  module rec Port : sig
    type t

    val equal : t -> t -> bool
    val name : t -> string
    val post_message : t -> Jv.t -> unit

    (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/Port#onmessage}} *)
    module On_message : sig
      type t

      val add_listener : (Jv.t -> unit) -> t -> unit
    end

    val on_message : t -> On_message.t

    (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/Port#ondisconnect}} *)
    module On_disconnect : sig
      type t

      val add_listener : (Port.t -> unit) -> t -> unit
    end

    val on_disconnect : t -> On_disconnect.t
  end

  val connect : ?name:string -> t -> Port.t
  val connect_native : string -> t -> Port.t
  val send_message : Jv.t -> t -> Jv.t Fut.or_error
  val send_native_message : string -> Jv.t -> t -> Jv.t Fut.or_error

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onConnect}} *)
  module On_connect : sig
    type t

    val add_listener : (Port.t -> unit) -> t -> unit
  end

  val on_connect : t -> On_connect.t

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/MessageSender}} *)
  module Message_sender : sig
    type t

    val tab : t -> Tab.t option
  end

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onMessage}} *)
  module On_message : sig
    type t

    val add_listener : (Jv.t -> Message_sender.t -> Jv.t -> Jv.t) -> t -> unit
  end

  val on_message : t -> On_message.t
end

val runtime : Runtime.t
