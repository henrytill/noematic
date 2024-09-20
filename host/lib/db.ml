module Schema_version = struct
  type t = Semver.t

  let of_string = Semver.of_string
  let to_string = Semver.to_string
  let compare = Semver.compare
  let equal = Semver.equal
  let current = (0, 1, 0)
end

let version_table_exists db =
  let open Sqlite3 in
  let stmt_string =
    "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_version')"
  in
  let stmt = prepare db stmt_string in
  match step stmt with
  | Rc.ROW -> column_bool stmt 0
  | _ -> false

let version_exists db =
  let open Sqlite3 in
  let stmt_string = "SELECT EXISTS (SELECT 1 FROM schema_version)" in
  let stmt = prepare db stmt_string in
  match step stmt with
  | Rc.ROW -> column_bool stmt 0
  | _ -> false

let get_latest_version db =
  let open Sqlite3 in
  let stmt_string =
    {|SELECT major, minor, patch
FROM schema_version
ORDER BY applied_at DESC
LIMIT 1
|}
  in
  let stmt = prepare db stmt_string in
  match step stmt with
  | Rc.ROW ->
      let major = column_int stmt 0 in
      let minor = column_int stmt 1 in
      let patch = column_int stmt 2 in
      Some (major, minor, patch)
  | _ -> None

let get_version db =
  if version_table_exists db then
    if version_exists db then
      get_latest_version db
    else
      None
  else
    None

let migrate _db _from _to = ()

let insert_version db version =
  let open Sqlite3 in
  let stmt_string = "INSERT INTO schema_version (major, minor, patch) VALUES (?, ?, ?)" in
  let stmt = prepare db stmt_string in
  let major, minor, patch = version in
  Rc.check (bind_int stmt 1 major);
  Rc.check (bind_int stmt 2 minor);
  Rc.check (bind_int stmt 3 patch);
  Rc.check (step stmt)

let init_tables db =
  let open Sqlite3 in
  Rc.check (exec db "BEGIN TRANSACTION");
  begin
    try
      match get_version db with
      | Some version when Schema_version.(equal version current) -> ()
      | Some version when Schema_version.(compare version current) < 0 ->
          migrate db version Schema_version.current;
          insert_version db Schema_version.current
      | Some version ->
          failwith (Printf.sprintf "Invalid schema version: %s" (Schema_version.to_string version))
      | None ->
          Rc.check (exec db Strings.create_sql);
          insert_version db Schema_version.current
    with exn ->
      Rc.check (exec db "ROLLBACK TRANSACTION");
      raise exn
  end;
  Rc.check (exec db "END TRANSACTION")

let upsert db save_payload =
  let open Sqlite3 in
  let stmt_string =
    {|INSERT INTO sites (url, title, inner_text)
VALUES (?, ?, ?)
ON CONFLICT (url) DO UPDATE SET
    title = excluded.title,
    inner_text = excluded.inner_text,
    updated_at = CURRENT_TIMESTAMP
|}
  in
  let stmt = prepare db stmt_string in
  let open Message.Request.Save in
  let url = Message.Uri_ext.to_string save_payload.uri in
  let title = Message.Title.to_string save_payload.title in
  let inner_text = Message.Inner_text.to_string save_payload.inner_text in
  Rc.check (bind_text stmt 1 url);
  Rc.check (bind_text stmt 2 title);
  Rc.check (bind_text stmt 3 inner_text);
  Rc.check (step stmt)

let search_sites db search_payload process =
  let open Sqlite3 in
  let stmt_string =
    {|SELECT s.url, s.title, snippet(sites_fts, 2, '<b>', '</b>', '...', 40)
FROM sites_fts
JOIN sites s ON sites_fts.rowid = s.id
WHERE sites_fts MATCH ?
ORDER BY rank
LIMIT ? OFFSET ?
|}
  in
  let stmt = prepare db stmt_string in
  let open Message.Request.Search in
  let query = process search_payload.query in
  let limit = succ search_payload.page_length (* extra row for has_more *) in
  let offset = search_payload.page_num * search_payload.page_length in
  Rc.check (bind_text stmt 1 query);
  Rc.check (bind_int stmt 2 limit);
  Rc.check (bind_int stmt 3 offset);
  let rec collect_results count acc =
    if count > search_payload.page_length then
      (List.rev (List.tl acc), true)
    else
      match step stmt with
      | Rc.ROW ->
          let open Message in
          let uri = Uri_ext.of_string (column_text stmt 0) in
          let title = Title.of_string (column_text stmt 1) in
          let snippet = Snippet.of_string (column_text stmt 2) in
          let site = Response.Site.{ uri; title; snippet } in
          collect_results (succ count) (site :: acc)
      | Rc.DONE -> (List.rev acc, false)
      | _ -> failwith "unexpected rc"
  in
  collect_results 0 []
