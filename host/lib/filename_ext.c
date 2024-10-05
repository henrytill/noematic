#define _DEFAULT_SOURCE
#include <caml/alloc.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>

CAMLprim value caml_realpath(value path) {
  CAMLparam1(path);
  char *result = realpath(String_val(path), NULL);
  if (result == NULL) {
    caml_failwith("realpath failed");
  }
  CAMLlocal1(caml_result);
  caml_result = caml_copy_string(result);
  free(result);
  CAMLreturn(caml_result);
}
