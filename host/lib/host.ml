module Context = struct
  type t = {
    db : Sqlite3.db;
    process : Message.Query.t -> string;
  }

  let make_process regex query =
    Message.Query.to_string query |> Re.replace_string regex ~by:" " |> String.trim

  let in_memory () =
    let db = Sqlite3.db_open ":memory:" in
    let process_regex = Re.Perl.compile_pat {|\W+|} in
    let process query = make_process process_regex query in
    Db.init_tables db;
    { db; process }

  let persistent db_path =
    let db = Sqlite3.db_open db_path in
    let process_regex = Re.Perl.compile_pat {|\W+|} in
    let process query = make_process process_regex query in
    Db.init_tables db;
    { db; process }

  let close self = ignore (Sqlite3.db_close self.db)
end

let handle_request (context : Context.t) (request : Message.Request.t) : Message.Response.t =
  let open Message in
  let version = request.version in
  let correlation_id = request.correlation_id in
  match request.action with
  | Request.Action.Save { payload } ->
      Db.upsert context.db payload;
      let action = Response.Action.Save { payload = () } in
      { version; action; correlation_id }
  | Request.Action.Search { payload } ->
      let results = Db.search_sites context.db payload context.process in
      let payload = Response.Search.{ query = payload.query; results } in
      let action = Response.Action.Search { payload } in
      { version; action; correlation_id }

let extract_version json = json |> Yojson.Safe.Util.member "version" |> Message.Version.t_of_yojson
