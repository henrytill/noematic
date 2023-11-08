module Tab : sig
  type t

  val url : t -> Brr.Uri.t
  val id : t -> int
end

module Tabs : sig
  type t

  val active : t -> Tab.t array Fut.or_error
  val create : string -> t -> Tab.t Fut.or_error
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
    module OnMessage : sig
      type t

      val add_listener : (Jv.t -> unit) -> t -> unit
    end

    val on_message : t -> OnMessage.t

    (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/Port#ondisconnect}} *)
    module OnDisconnect : sig
      type t

      val add_listener : (Port.t -> unit) -> t -> unit
    end

    val on_disconnect : t -> OnDisconnect.t
  end

  val connect : ?name:string -> t -> Port.t
  val connect_native : string -> t -> Port.t
  val send_message : Jv.t -> t -> Jv.t Fut.or_error
  val send_native_message : string -> Jv.t -> t -> Jv.t Fut.or_error

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onConnect}} *)
  module OnConnect : sig
    type t

    val add_listener : (Port.t -> unit) -> t -> unit
  end

  val on_connect : t -> OnConnect.t

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/MessageSender}} *)
  module MessageSender : sig
    type t

    val tab : t -> Tab.t option
  end

  (** {{:https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/onMessage}} *)
  module OnMessage : sig
    type t

    val add_listener : (Jv.t -> MessageSender.t -> Jv.t -> Jv.t) -> t -> unit
  end

  val on_message : t -> OnMessage.t
end

val runtime : Runtime.t
