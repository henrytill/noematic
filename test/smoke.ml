let hello = Jv.of_string "Hello, world!"

let add x y =
  let x = Jv.to_int x in
  let y = Jv.to_int y in
  Jv.of_int (x + y)

let () = Jv.set Jv.global "smoke" (Jv.obj [| ("hello", hello); ("add", Jv.callback ~arity:2 add) |])
