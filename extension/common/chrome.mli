module Tab : sig
  type t

  val url : t -> Brr.Uri.t
  val id : t -> int
end

module Tabs : sig
  type t

  val active : t -> Tab.t array Fut.or_error
end

val tabs : Tabs.t

module Runtime : sig
  type t

  module Port : sig
    type t

    val equal : t -> t -> bool
    val of_jv : Jv.t -> t

    module Event : sig
      type t

      val add_listener : t -> (Jv.t -> unit) -> unit
    end

    val post_message : t -> Jv.t -> unit
    val on_message : t -> Event.t
    val on_disconnect : t -> Event.t
  end

  val connect : t -> Port.t
  val connect_native : t -> string -> Port.t

  module Event : sig
    type t

    val add_listener : t -> (Port.t -> unit) -> unit
  end

  val on_connect : t -> Event.t
end

val runtime : Runtime.t
