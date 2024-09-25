include Stdlib.Filename

external unsafe_realpath : string -> string = "caml_realpath"

let realpath path = try Some (unsafe_realpath path) with Sys_error _ -> None
