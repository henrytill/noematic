let buffer_size = 1024

let read_length chan =
  let buffer = Bytes.create 4 in
  match input chan buffer 0 4 with
  | 4 -> Some (Bytes.get_int32_ne buffer 0)
  | _ -> None

let read_json_message chan length =
  let length = Int32.to_int length in
  let message_buffer = Buffer.create length in
  let read_buffer = Bytes.create buffer_size in
  let rec go rem_bytes =
    if rem_bytes > 0 then
      let bytes_to_read = min buffer_size rem_bytes in
      let bytes_read = input chan read_buffer 0 bytes_to_read in
      if bytes_read > 0 then (
        Buffer.add_subbytes message_buffer read_buffer 0 bytes_read;
        go (rem_bytes - bytes_read))
      else
        raise (Failure "Unexpected end of file")
  in
  go length;
  Buffer.to_bytes message_buffer

let handle_message oc length json_message =
  output_string oc (Bytes.to_string json_message);
  flush oc;
  let length_buf = Bytes.create 4 in
  Bytes.set_int32_ne length_buf 0 length;
  print_bytes (Bytes.cat length_buf json_message);
  flush stdout

let main () =
  let tmp = Filename.temp_file "noematic-" ".txt" in
  let oc = open_out tmp in
  try
    while true do
      match read_length stdin with
      | None -> raise End_of_file
      | Some length ->
          let json_message = read_json_message stdin length in
          handle_message oc length json_message
    done
  with End_of_file -> close_out_noerr oc

let () = main ()
