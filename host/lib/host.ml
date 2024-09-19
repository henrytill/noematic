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

let handle_request (context : Context.t) (request : Message.Request.t) : Message.Response.t list =
  let open Message in
  let version = request.version in
  let correlation_id = request.correlation_id in
  match request.action with
  | Request.Action.Save { payload } ->
      Db.upsert context.db payload;
      let action = Response.Action.Save { payload = () } in
      { version; action; correlation_id } :: []
  | Request.Action.Search { payload } ->
      let query = payload.query in
      let page_num = payload.page_num in
      let results, has_more = Db.search_sites context.db payload context.process in
      let page_length = List.length results in
      let header =
        let payload = Response.Search.{ query; page_num; page_length; has_more } in
        let action = Response.Action.Search { payload } in
        Response.{ version; action; correlation_id }
      in
      let sites =
        List.map
          (fun payload ->
            let action = Response.Action.Site { payload } in
            Response.{ version; action; correlation_id })
          results
      in
      header :: sites

let extract_version json = json |> Yojson.Safe.Util.member "version" |> Message.Version.t_of_yojson
