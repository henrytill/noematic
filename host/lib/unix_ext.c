#include <caml/alloc.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>
#include <sys/utsname.h>

CAMLprim value caml_uname(value unit) {
  CAMLparam1(unit);
  struct utsname buf;
  const int rc = uname(&buf);
  if (rc == -1) {
    caml_failwith("uname failed");
  }
  CAMLlocal1(caml_result);
  caml_result = caml_alloc(5, 0);
  Store_field(caml_result, 0, caml_copy_string(buf.sysname));
  Store_field(caml_result, 1, caml_copy_string(buf.nodename));
  Store_field(caml_result, 2, caml_copy_string(buf.release));
  Store_field(caml_result, 3, caml_copy_string(buf.version));
  Store_field(caml_result, 4, caml_copy_string(buf.machine));
  CAMLreturn(caml_result);
}
