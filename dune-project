(lang dune 3.16)

(name noematic)

(generate_opam_files true)

(source
 (github henrytill/noematic))

(authors "Henry Till <henrytill@gmail.com>")

(maintainers "Henry Till <henrytill@gmail.com>")

(license ISC)

(implicit_transitive_deps false)

(package
 (name noematic)
 (synopsis "A short synopsis")
 (description "A longer description")
 (depends
  (ocaml
   (>= 5.2.0))
  dune
  (alcotest
   (and
    :with-test
    (= 1.8.0)))
  (fmt :with-test)
  ppx_deriving
  (ppx_expect
   (and
    :with-test
    (= v0.17.0)))
  (ppx_yojson
   (= 1.3.0))
  (ppx_yojson_conv
   (= v0.17.0))
  (semver
   (= 0.2.1))
  (uri
   (= 4.4.0)))
 (tags
  (topics "to describe" your project)))
