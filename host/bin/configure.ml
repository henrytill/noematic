let home_dir () = Sys.getenv "HOME"

let default_prefix () =
  let ret =
    let path = Filename.concat (Filename.dirname Sys.executable_name) ".." in
    match Filename_ext.realpath path with
    | Some resolved_path -> resolved_path
    | None -> path
  in
  if not (Sys.file_exists ret) then
    failwith (Printf.sprintf "Directory does not exist: %s\n" ret);
  ret

module Host_manifest = struct
  open Ppx_yojson_conv_lib.Yojson_conv.Primitives

  let name = "com.github.henrytill.noematic"
  let description = "Search your backlog"
  let _type = "stdio"

  let host_binary_path prefix =
    let host_binary_name = "noematic" in
    List.fold_left Filename.concat prefix [ "bin"; host_binary_name ]

  module Firefox = struct
    let allowed_extensions = [ "henrytill@gmail.com" ]

    type t = {
      name : string; [@default name]
      description : string; [@default description]
      path : string;
      _type : string; [@default _type] [@key "type"]
      allowed_extensions : string list; [@default allowed_extensions]
    }
    [@@deriving make, yojson]

    let path ~name () =
      match Sys.os_type with
      | "Unix" ->
          let file = Printf.sprintf "%s.json" name in
          let path = [ ".mozilla"; "native-messaging-hosts"; file ] in
          List.fold_left Filename.concat (home_dir ()) path
      | _ ->
          let file = Printf.sprintf "%s.firefox.json" name in
          Filename.concat (Sys.getcwd ()) file
  end

  module Chromium = struct
    let allowed_origins = [ "chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/" ]

    type t = {
      name : string; [@default name]
      description : string; [@default description]
      path : string;
      _type : string; [@default _type] [@key "type"]
      allowed_origins : string list; [@default allowed_origins]
    }
    [@@deriving make, yojson]

    let path ~name () =
      match Sys.os_type with
      | "Unix" ->
          let file = Printf.sprintf "%s.json" name in
          let path = [ ".config"; "chromium"; "NativeMessagingHosts"; file ] in
          List.fold_left Filename.concat (home_dir ()) path
      | _ ->
          let file = Printf.sprintf "%s.chromium.json" name in
          Filename.concat (Sys.getcwd ()) file
  end

  let write_json path json =
    let oc = open_out path in
    Yojson.Safe.pretty_to_channel oc json;
    close_out oc

  let write prefix () =
    let path = host_binary_path prefix in
    (* Firefox *)
    let firefox_json = Firefox.make ~path () |> Firefox.yojson_of_t in
    let firefox_path = Firefox.path ~name () in
    Unix_ext.mkdir_all (Filename.dirname firefox_path) 0o755;
    write_json firefox_path firefox_json;
    print_endline (Printf.sprintf "Firefox host manifest written to: %s" firefox_path);
    print_endline "Firefox host manifest contents:";
    print_endline (Yojson.Safe.pretty_to_string firefox_json);
    (* Chromium *)
    let chromium_json = Chromium.make ~path () |> Chromium.yojson_of_t in
    let chromium_path = Chromium.path ~name () in
    Unix_ext.mkdir_all (Filename.dirname chromium_path) 0o755;
    write_json chromium_path chromium_json;
    print_endline (Printf.sprintf "Chromium host manifest written to: %s" chromium_path);
    print_endline "Chromium host manifest contents:";
    print_endline (Yojson.Safe.pretty_to_string chromium_json)
end

let prefix = ref (default_prefix ())
let spec_list = [ ("--prefix", Arg.Set_string prefix, "Installation prefix") ]
let usage_msg = "Usage: configure [--prefix <path>]"

let () =
  Arg.parse spec_list ignore usage_msg;
  Host_manifest.write !prefix ()
