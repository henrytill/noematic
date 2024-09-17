module Context : sig
  type t

  val in_memory : unit -> t
  val persistent : string -> t
end

val handle_request : Context.t -> Message.Request.t -> Message.Response.t
val extract_version : Yojson.Safe.t -> Message.Version.t
