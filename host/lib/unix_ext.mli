include module type of Unix

val mkdir_all : string -> file_perm -> unit

type utsname = {
  sysname : string;
  nodename : string;
  release : string;
  version : string;
  machine : string;
}

val uname : unit -> utsname
