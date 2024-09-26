include Unix

let rec mkdir_all path perms =
  if path = "." || path = "/" then
    ()
  else
    let parent = Filename.dirname path in
    if not (Sys.file_exists parent) then
      mkdir_all parent perms;
    if not (Sys.file_exists path) then
      try mkdir path perms with Unix_error (EEXIST, _, _) -> ()
    else if not (Sys.is_directory path) then
      failwith (Printf.sprintf "Error: %s exists but is not a directory" path)
