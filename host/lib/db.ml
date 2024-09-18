module Schema_version = struct
  type t = Semver.t

  let of_string = Semver.of_string
  let to_string = Semver.to_string
  let compare = Semver.compare
  let equal = Semver.equal
  let current = (0, 1, 0)
end

let create_sql = Strings.create_sql

let select_version_table_exists =
  "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_version')"

let select_version_exists = "SELECT EXISTS (SELECT 1 FROM schema_version)"

let select_latest_version =
  {|SELECT major, minor, patch
FROM schema_version
ORDER BY applied_at DESC
LIMIT 1
|}

let version_table_exists_p db =
  let stmt = Sqlite3.prepare db select_version_table_exists in
  match Sqlite3.step stmt with
  | Sqlite3.Rc.ROW -> Sqlite3.column_bool stmt 0
  | _ -> false

let version_exists_p db =
  let stmt = Sqlite3.prepare db select_version_exists in
  match Sqlite3.step stmt with
  | Sqlite3.Rc.ROW -> Sqlite3.column_bool stmt 0
  | _ -> false

let get_latest_version db =
  let stmt = Sqlite3.prepare db select_latest_version in
  match Sqlite3.step stmt with
  | Sqlite3.Rc.ROW ->
      let major = Sqlite3.column_int stmt 0 in
      let minor = Sqlite3.column_int stmt 1 in
      let patch = Sqlite3.column_int stmt 2 in
      Some (major, minor, patch)
  | _ -> None

let get_version db =
  if version_table_exists_p db then
    if version_exists_p db then
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
  Rc.check (step stmt);
  version

let init_tables db =
  match get_version db with
  | Some version when Schema_version.equal version Schema_version.current -> ()
  | Some version when Schema_version.compare version Schema_version.current < 0 ->
      let () = migrate db version Schema_version.current in
      ignore (insert_version db Schema_version.current)
  | Some version -> failwith ("invalid schema version: " ^ Schema_version.to_string version)
  | None ->
      Sqlite3.Rc.check (Sqlite3.exec db create_sql);
      ignore (insert_version db Schema_version.current)

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
  Rc.check (bind_text stmt 1 Message.(Uri_ext.to_string (Request.Save.uri save_payload)));
  Rc.check (bind_text stmt 2 Message.(Title.to_string (Request.Save.title save_payload)));
  Rc.check (bind_text stmt 3 Message.(Inner_text.to_string (Request.Save.inner_text save_payload)));
  Rc.check (step stmt)

let search_sites db search_payload process =
  let open Sqlite3 in
  let stmt_string =
    {|SELECT s.url, s.title, snippet(sites_fts, 2, '<b>', '</b>', '...', 40)
FROM sites_fts
JOIN sites s ON sites_fts.rowid = s.id
WHERE sites_fts MATCH ?
ORDER BY rank
|}
  in
  let stmt = prepare db stmt_string in
  let query_str = process (Message.Request.Search.query search_payload) in
  Rc.check (bind_text stmt 1 query_str);
  let rec collect_results acc =
    match step stmt with
    | Rc.ROW ->
        let open Message in
        let uri = Uri_ext.of_string (column_text stmt 0) in
        let title = Title.of_string (column_text stmt 1) in
        let snippet = Snippet.of_string (column_text stmt 2) in
        let site = Response.site ~uri ~title ~snippet in
        collect_results (site :: acc)
    | Rc.DONE -> List.rev acc
    | _ -> failwith "unexpected rc"
  in
  collect_results []
