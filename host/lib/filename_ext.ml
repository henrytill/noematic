include Stdlib.Filename

external unsafe_realpath : string -> string = "caml_realpath"

let realpath = unsafe_realpath
let realpath_opt path = try Some (unsafe_realpath path) with Sys_error _ -> None
