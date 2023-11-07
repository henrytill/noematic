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

  module Port : sig
    type t

    val equal : t -> t -> bool
    val of_jv : Jv.t -> t

    module Event : sig
      type t

      val add_listener : (Jv.t -> unit) -> t -> unit
    end

    val name : t -> string
    val post_message : t -> Jv.t -> unit
    val on_message : t -> Event.t
    val on_disconnect : t -> Event.t
  end

  val connect : ?name:string -> t -> Port.t
  val connect_native : t -> string -> Port.t
  val id : t -> string

  module Event : sig
    type t

    val add_listener : (Port.t -> unit) -> t -> unit
  end

  val on_connect : t -> Event.t
end

val runtime : Runtime.t
