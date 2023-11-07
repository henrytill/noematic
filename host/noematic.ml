let () =
  let buffer_size = 1024 in
  let buffer = Bytes.create buffer_size in
  let tmp = Filename.temp_file "noematic-" ".txt" in
  let oc = open_out tmp in
  try
    while true do
      let bytes_read = input stdin buffer 0 buffer_size in
      if bytes_read = 0 then raise End_of_file;
      output oc buffer 0 bytes_read;
      flush oc;
      output stdout buffer 0 bytes_read;
      flush stdout
    done
  with End_of_file -> close_out_noerr oc
