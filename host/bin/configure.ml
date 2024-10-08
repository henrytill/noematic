let home_dir () = Sys.getenv "HOME"

module Platform = struct
  type t =
    | Linux
    | Darwin
    | Win32
    | Cygwin
    | Other

  let current () =
    match Sys.os_type with
    | "Unix" -> (
        match (Unix_ext.uname ()).sysname with
        | "Linux" -> Linux
        | "Darwin" -> Darwin
        | _ -> Other)
    | "Win32" -> Win32
    | "Cygwin" -> Cygwin
    | _ -> Other
end

module Host_manifest = struct
  open Ppx_yojson_conv_lib.Yojson_conv.Primitives

  let name = "com.github.henrytill.noematic"
  let file = Printf.sprintf "%s.json" name
  let description = "Search your backlog"
  let ty = "stdio"

  let host_binary_path prefix =
    let host_binary_name = "noematic-host" in
    List.fold_left Filename.concat prefix [ "bin"; host_binary_name ]

  module Path = struct
    type t = {
      linux : string list;
      darwin : string list;
      default : string list;
    }

    let for_platform (path : t) (default_dir : string) : Platform.t -> string = function
      | Linux -> List.fold_left Filename.concat (home_dir ()) path.linux
      | Darwin -> List.fold_left Filename.concat (home_dir ()) path.darwin
      | Other -> List.fold_left Filename.concat default_dir path.default
      | Win32 | Cygwin -> failwith "unimplemented"
  end

  module Firefox = struct
    let allowed_extensions = [ "henrytill@gmail.com" ]

    type t = {
      name : string; [@default name]
      description : string; [@default description]
      path : string;
      ty : string; [@default ty] [@key "type"]
      allowed_extensions : string list; [@default allowed_extensions]
    }
    [@@deriving make, yojson]

    let path : Path.t =
      {
        linux = [ ".mozilla"; "native-messaging-hosts"; file ];
        darwin = [ "Library"; "Application Support"; "Mozilla"; "NativeMessagingHosts"; file ];
        default = [ "manifests"; "mozilla"; file ];
      }
  end

  module Chromium = struct
    let allowed_origins = [ "chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/" ]

    type t = {
      name : string; [@default name]
      description : string; [@default description]
      path : string;
      ty : string; [@default ty] [@key "type"]
      allowed_origins : string list; [@default allowed_origins]
    }
    [@@deriving make, yojson]

    let path : Path.t =
      {
        linux = [ ".config"; "chromium"; "NativeMessagingHosts"; file ];
        darwin = [ "Library"; "Application Support"; "Chromium"; "NativeMessagingHosts"; file ];
        default = [ "manifests"; "chromium"; file ];
      }
  end

  let write_json path json =
    let oc = open_out path in
    Yojson.Safe.pretty_to_channel oc json;
    close_out oc

  let write prefix () =
    let path = host_binary_path prefix in
    let platform = Platform.current () in
    let default_dir = Sys.getcwd () in
    (* Firefox *)
    let firefox_json = Firefox.make ~path () |> Firefox.yojson_of_t in
    let firefox_path = Path.for_platform Firefox.path default_dir platform in
    Unix_ext.mkdir_all (Filename.dirname firefox_path) 0o755;
    write_json firefox_path firefox_json;
    print_endline (Printf.sprintf "Firefox host manifest written to: %s" firefox_path);
    print_endline "Firefox host manifest contents:";
    print_endline (Yojson.Safe.pretty_to_string firefox_json);
    (* Chromium *)
    let chromium_json = Chromium.make ~path () |> Chromium.yojson_of_t in
    let chromium_path = Path.for_platform Chromium.path default_dir platform in
    Unix_ext.mkdir_all (Filename.dirname chromium_path) 0o755;
    write_json chromium_path chromium_json;
    print_endline (Printf.sprintf "Chromium host manifest written to: %s" chromium_path);
    print_endline "Chromium host manifest contents:";
    print_endline (Yojson.Safe.pretty_to_string chromium_json)
end

let default_prefix () =
  let ret =
    let path = Filename.concat (Filename.dirname Sys.executable_name) ".." in
    match Filename_ext.realpath_opt path with
    | Some resolved_path -> resolved_path
    | None -> path
  in
  if not (Sys.file_exists ret) then
    failwith (Printf.sprintf "Directory does not exist: %s\n" ret);
  ret

let prefix = ref (default_prefix ())
let spec_list = [ ("--prefix", Arg.Set_string prefix, "Installation prefix") ]
let usage_msg = "Usage: configure [--prefix <path>]"

let () =
  Arg.parse spec_list ignore usage_msg;
  Host_manifest.write !prefix ()
