module Schema_version : sig
  type t

  val of_string : string -> t option
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
end

val init_tables : Sqlite3.db -> unit
val upsert : Sqlite3.db -> Message.Request.Save.t -> unit

val search_sites :
  Sqlite3.db ->
  Message.Request.Search.t ->
  (Message.Query.t -> string) ->
  Message.Response.Site.t list
