open Noematic

let exec_subcommand subcommand args =
  let directory = Filename.dirname Sys.executable_name in
  let executable = Printf.sprintf "noematic-%s" subcommand in
  let executable_path = Filename.concat directory executable in
  if Sys.file_exists executable_path then
    Unix.execv executable_path (Array.of_list (executable :: args))
  else (
    Printf.eprintf "Error: %s not found in %s\n" executable directory;
    exit 1)

let handle_subcommand argv =
  match Array.to_list argv with
  | _ :: ("configure" as subcommand) :: args -> exec_subcommand subcommand args
  | _ -> ()

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

let rec mkdir_p path perms =
  if path = "." || path = "/" then
    ()
  else
    let parent = Filename.dirname path in
    if not (Sys.file_exists parent) then
      mkdir_p parent perms;
    if not (Sys.file_exists path) then
      try Unix.mkdir path perms with Unix.Unix_error (Unix.EEXIST, _, _) -> ()
    else if not (Sys.is_directory path) then
      failwith (Printf.sprintf "Error: %s exists but is not a directory" path)

let context_of_args (args : Args.t) : Host.Context.t =
  if args.test then
    Host.Context.in_memory ()
  else
    let db_path = get_db_path () in
    mkdir_p (Filename.dirname db_path) 0o755;
    Host.Context.persistent db_path

let rec process_messages context ic oc =
  let length = Protocol.read_length stdin in
  let request_json = Protocol.read ic length in
  let version = Host.extract_version request_json in
  if not (Message.Version.equal version Message.Version.expected) then
    failwith "Unsupported version";
  let request = Message.Request.t_of_yojson request_json in
  let response = Host.handle_request context request in
  let response_json = Message.Response.yojson_of_t response in
  Protocol.write oc response_json;
  process_messages context ic oc

let () =
  handle_subcommand Sys.argv;
  let args = Args.of_argv Sys.argv in
  let context = context_of_args args in
  let in_channel = stdin in
  let out_channel = stdout in
  try process_messages context in_channel out_channel
  with End_of_file ->
    Host.Context.close context;
    close_in_noerr in_channel;
    close_out_noerr out_channel
