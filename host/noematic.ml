let input_buffer_length = 1024

let read_length ic =
  let bytes = Bytes.create 4 in
  match input ic bytes 0 4 with
  | 4 -> Some (Bytes.get_int32_ne bytes 0)
  | _ -> None

let read_message ic length =
  let length = Int32.to_int length in
  let message_buffer = Buffer.create length in
  let input_buffer = Bytes.create input_buffer_length in
  let rec go rem_bytes =
    if rem_bytes > 0 then
      let bytes_to_read = min input_buffer_length rem_bytes in
      let bytes_read = input ic input_buffer 0 bytes_to_read in
      if bytes_read > 0 then (
        Buffer.add_subbytes message_buffer input_buffer 0 bytes_read;
        go (rem_bytes - bytes_read))
      else
        raise (Failure "Unexpected end of file")
  in
  go length;
  Buffer.to_bytes message_buffer

let handle_message oc length message =
  output_string oc (Bytes.to_string message);
  flush oc;
  let length_bytes =
    begin
      let bytes = Bytes.create 4 in
      Bytes.set_int32_ne bytes 0 length;
      bytes
    end
  in
  output_bytes stdout (Bytes.cat length_bytes message);
  flush stdout

let () =
  let file = Filename.temp_file "noematic-" ".txt" in
  let oc = open_out file in
  try
    while true do
      match read_length stdin with
      | None -> raise End_of_file
      | Some length ->
          let message = read_message stdin length in
          handle_message oc length message
    done
  with End_of_file -> close_out_noerr oc
