external js_expr : string -> 'a = "caml_js_expr"

let hello = Jv.of_string "Hello, world!"

let add x y =
  let x = Jv.to_int x in
  let y = Jv.to_int y in
  Jv.of_int (x + y)

let () =
  let exports = js_expr "exports" in
  Jv.set exports "hello" hello;
  Jv.set exports "add" (Jv.callback ~arity:2 add)
