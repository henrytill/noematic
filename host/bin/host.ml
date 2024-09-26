open Noematic

module Args = struct
  type t = { test : bool }

  let of_argv argv =
    let[@warning "-23"] rec parse acc = function
      | [] -> acc
      | "-test" :: rest -> parse { acc with test = true } rest
      | _ :: rest -> parse acc rest
    in
    parse { test = false } (List.tl (Array.to_list argv))
end

let get_db_path () =
  let xdg = Xdg.create ~env:Sys.getenv_opt () in
  let data_dir = Xdg.data_dir xdg in
  let data_dir = Filename.concat data_dir "noematic" in
  Filename.concat data_dir "db.sqlite3"

let context_of_args (args : Args.t) : Host.Context.t =
  if args.test then
    Host.Context.in_memory ()
  else
    let db_path = get_db_path () in
    Unix_ext.mkdir_all (Filename.dirname db_path) 0o755;
    Host.Context.persistent db_path

let rec process_messages context ic oc =
  let request_json = Protocol.read_length ic |> Protocol.read ic in
  let request_version = Host.extract_version request_json in
  if not (Message.Version.equal request_version Message.Version.expected) then
    failwith "Unsupported version";
  let request = Message.Request.t_of_yojson request_json in
  let responses = Host.handle_request context request in
  let write_json = Fun.compose (Protocol.write oc) Message.Response.yojson_of_t in
  List.iter write_json responses;
  process_messages context ic oc

let () =
  let args = Args.of_argv Sys.argv in
  let context = context_of_args args in
  let in_channel = stdin in
  let out_channel = stdout in
  let finally () =
    Host.Context.close context;
    close_in_noerr in_channel;
    close_out_noerr out_channel
  in
  try process_messages context in_channel out_channel with
  | End_of_file -> finally ()
  | exn ->
      finally ();
      raise exn
