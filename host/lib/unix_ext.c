#include <caml/alloc.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>
#include <sys/utsname.h>

CAMLprim value caml_uname(value unit) {
  CAMLparam1(unit);
  CAMLlocal1(result);
  struct utsname buf;

  if (uname(&buf) == -1) {
    caml_failwith("uname failed");
  }

  result = caml_alloc(5, 0);
  Store_field(result, 0, caml_copy_string(buf.sysname));
  Store_field(result, 1, caml_copy_string(buf.nodename));
  Store_field(result, 2, caml_copy_string(buf.release));
  Store_field(result, 3, caml_copy_string(buf.version));
  Store_field(result, 4, caml_copy_string(buf.machine));

  CAMLreturn(result);
}

