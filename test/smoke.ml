open Js_of_ocaml

let hello = Js.string "Hello, world!"

let add x y =
  let x = Js.parseInt x in
  let y = Js.parseInt y in
  x + y

let exports =
  if Js.Unsafe.js_expr "typeof exports !== 'undefined'" then
    Js.Unsafe.js_expr "exports"
  else
    let exports = Js.Unsafe.obj [||] in
    Js.Unsafe.set Js.Unsafe.global "smoke" exports;
    exports

let () =
  Js.Unsafe.set exports "hello" hello;
  Js.Unsafe.set exports "add" (Js.wrap_callback add)
