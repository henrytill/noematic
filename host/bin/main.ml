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
  | _ :: ("configure" as subcommand) :: args | _ :: ("host" as subcommand) :: args ->
      exec_subcommand subcommand args
  | _ -> ()

let () = handle_subcommand Sys.argv
