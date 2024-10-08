let read_length ic =
  let length_bytes = Bytes.create 4 in
  really_input ic length_bytes 0 4;
  Bytes.get_int32_ne length_bytes 0

let read ic length =
  let length = Int32.to_int length in
  let message_bytes = Bytes.create length in
  really_input ic message_bytes 0 length;
  let message_string = Bytes.to_string message_bytes in
  Yojson.Safe.from_string message_string

let write oc message =
  let message_string = Yojson.Safe.to_string message in
  let message_length = String.length message_string in
  let message_length_bytes = Bytes.create 4 in
  Bytes.set_int32_ne message_length_bytes 0 (Int32.of_int message_length);
  output_bytes oc message_length_bytes;
  output_string oc message_string;
  flush oc
