module Uri_ext : sig
  include module type of Uri

  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Version : sig
  type t

  val of_string : string -> t option
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val expected : t
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Correlation_id : sig
  type t

  val of_string : string -> t
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Title : sig
  type t

  val of_string : string -> t
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Inner_text : sig
  type t

  val of_string : string -> t
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Query : sig
  type t

  val of_string : string -> t
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Snippet : sig
  type t

  val of_string : string -> t
  val to_string : t -> string
  val compare : t -> t -> int
  val equal : t -> t -> bool
  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Request : sig
  module Save : sig
    type t = {
      uri : Uri_ext.t;
      title : Title.t;
      inner_text : Inner_text.t;
    }

    val pp : Format.formatter -> t -> unit
    val t_of_yojson : Yojson.Safe.t -> t
    val yojson_of_t : t -> Yojson.Safe.t
  end

  module Search : sig
    type t = {
      query : Query.t;
      page_num : int;
      page_length : int;
    }

    val pp : Format.formatter -> t -> unit
    val t_of_yojson : Yojson.Safe.t -> t
    val yojson_of_t : t -> Yojson.Safe.t
  end

  module Action : sig
    type t =
      | Save of { payload : Save.t }
      | Search of { payload : Search.t }

    val pp : Format.formatter -> t -> unit
  end

  type t = {
    version : Version.t;
    action : Action.t;
    correlation_id : Correlation_id.t;
  }

  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end

module Response : sig
  module Save : sig
    type t = unit

    val pp : Format.formatter -> t -> unit
    val t_of_yojson : Yojson.Safe.t -> t
    val yojson_of_t : t -> Yojson.Safe.t
  end

  module Search : sig
    type t = {
      query : Query.t;
      page_num : int;
      page_length : int;
      has_more : bool;
    }

    val pp : Format.formatter -> t -> unit
    val t_of_yojson : Yojson.Safe.t -> t
    val yojson_of_t : t -> Yojson.Safe.t
  end

  module Site : sig
    type t = {
      uri : Uri_ext.t;
      title : Title.t;
      snippet : Snippet.t;
    }

    val pp : Format.formatter -> t -> unit
    val t_of_yojson : Yojson.Safe.t -> t
    val yojson_of_t : t -> Yojson.Safe.t
  end

  module Action : sig
    type t =
      | Save of { payload : Save.t }
      | Search of { payload : Search.t }
      | Site of { payload : Site.t }

    val pp : Format.formatter -> t -> unit
  end

  type t = {
    version : Version.t;
    action : Action.t;
    correlation_id : Correlation_id.t;
  }

  val pp : Format.formatter -> t -> unit
  val t_of_yojson : Yojson.Safe.t -> t
  val yojson_of_t : t -> Yojson.Safe.t
end
