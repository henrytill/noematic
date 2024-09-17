open Ppx_yojson_conv_lib.Yojson_conv.Primitives

module Semver_ext = struct
  type t = int * int * int [@@deriving show]

  include (Semver : module type of Semver with type t := t)

  let t_of_yojson json = Option.get (Semver.of_string ([%of_yojson: string] json))
  let yojson_of_t self = [%yojson_of: string] (Semver.to_string self)
end

module Uri_ext = struct
  include Uri

  let t_of_yojson json = Uri.of_string ([%of_yojson: string] json)
  let yojson_of_t self = [%yojson_of: string] (Uri.to_string self)
end

module Version = struct
  type t = Semver_ext.t [@@deriving show, yojson]

  let of_string = Semver_ext.of_string
  let to_string = Semver_ext.to_string
  let compare = Semver_ext.compare
  let equal = Semver_ext.equal
  let expected = (0, 1, 0)
end

module Correlation_id = struct
  type t = string [@@deriving show, yojson]

  let of_string (s : string) : t = s
  let to_string = Fun.id
  let compare = String.compare
  let equal = String.equal
end

module Title = struct
  type t = string [@@deriving show, yojson]

  let of_string (s : string) : t = s
  let to_string = Fun.id
  let compare = String.compare
  let equal = String.equal
end

module Inner_text = struct
  type t = string [@@deriving show, yojson]

  let of_string (s : string) : t = s
  let to_string = Fun.id
  let compare = String.compare
  let equal = String.equal
end

module Query = struct
  type t = string [@@deriving show, yojson]

  let of_string (s : string) : t = s
  let to_string = Fun.id
  let compare = String.compare
  let equal = String.equal
end

module Snippet = struct
  type t = string [@@deriving show, yojson]

  let of_string (s : string) : t = s
  let to_string = Fun.id
  let compare = String.compare
  let equal = String.equal
end

module Request = struct
  module Save = struct
    type t = {
      uri : Uri_ext.t; [@key "url"]
      title : Title.t;
      inner_text : Inner_text.t; [@key "innerText"]
    }
    [@@deriving show { with_path = false }, yojson]

    let uri self = self.uri
    let title self = self.title
    let inner_text self = self.inner_text
  end

  module Search = struct
    type t = { query : Query.t } [@@deriving show { with_path = false }, yojson]

    let query self = self.query
  end

  module Action = struct
    type t =
      | Save of { payload : Save.t }
      | Search of { payload : Search.t }
    [@@deriving show { with_path = false }]
  end

  type t = {
    version : Version.t;
    action : Action.t;
    correlation_id : Correlation_id.t;
  }
  [@@deriving show { with_path = false }]

  let t_of_yojson json =
    let open Yojson.Safe.Util in
    let version = json |> member "version" |> Version.t_of_yojson in
    let action =
      match json |> member "action" |> to_string with
      | "saveRequest" ->
          let payload = json |> member "payload" |> Save.t_of_yojson in
          Action.Save { payload }
      | "searchRequest" ->
          let payload = json |> member "payload" |> Search.t_of_yojson in
          Action.Search { payload }
      | _ -> failwith "unknown action"
    in
    let correlation_id = json |> member "correlationId" |> Correlation_id.t_of_yojson in
    { version; action; correlation_id }

  let yojson_of_t self =
    let version = Version.yojson_of_t self.version in
    let action =
      match self.action with
      | Save _ -> `String "saveRequest"
      | Search _ -> `String "searchRequest"
    in
    let payload =
      match self.action with
      | Save { payload } -> Save.yojson_of_t payload
      | Search { payload } -> Search.yojson_of_t payload
    in
    let correlation_id = Correlation_id.yojson_of_t self.correlation_id in
    `Assoc
      [
        ("version", version);
        ("action", action);
        ("payload", payload);
        ("correlationId", correlation_id);
      ]
end

module Response = struct
  module Save = struct
    type t = unit [@@deriving show { with_path = false }, yojson]
  end

  module Site = struct
    type t = {
      uri : Uri_ext.t; [@key "url"]
      title : Title.t;
      snippet : Snippet.t;
    }
    [@@deriving show { with_path = false }, yojson]
  end

  module Search = struct
    type t = {
      query : Query.t;
      results : Site.t list;
    }
    [@@deriving show { with_path = false }, yojson]
  end

  module Action = struct
    type t =
      | Save of { payload : Save.t }
      | Search of { payload : Search.t }
    [@@deriving show { with_path = false }, yojson]
  end

  type t = {
    version : Version.t;
    action : Action.t;
    correlation_id : Correlation_id.t;
  }
  [@@deriving show { with_path = false }]

  let t_of_yojson json =
    let open Yojson.Safe.Util in
    let version = json |> member "version" |> Version.t_of_yojson in
    let action =
      match json |> member "action" |> to_string with
      | "saveResponse" ->
          let payload = json |> member "payload" |> Save.t_of_yojson in
          Action.Save { payload }
      | "searchResponse" ->
          let payload = json |> member "payload" |> Search.t_of_yojson in
          Action.Search { payload }
      | _ -> failwith "unknown action"
    in
    let correlation_id = json |> member "correlationId" |> Correlation_id.t_of_yojson in
    { version; action; correlation_id }

  let yojson_of_t self =
    let version = Version.yojson_of_t self.version in
    let action =
      match self.action with
      | Save _ -> `String "saveResponse"
      | Search _ -> `String "searchResponse"
    in
    let payload =
      match self.action with
      | Save { payload } -> Save.yojson_of_t payload
      | Search { payload } -> Search.yojson_of_t payload
    in
    let correlation_id = Correlation_id.yojson_of_t self.correlation_id in
    `Assoc
      [
        ("version", version);
        ("action", action);
        ("payload", payload);
        ("correlationId", correlation_id);
      ]

  let site ~uri ~title ~snippet = Site.{ uri; title; snippet }
end
