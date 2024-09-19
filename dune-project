(lang dune 3.16)

(using dune_site 0.1)

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
 (sites
  (share firefox)
  (share chromium))
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
  (re
   (= 1.12.0))
  (semver
   (= 0.2.1))
  (sqlite3
   (= 5.2.0))
  (uri
   (= 4.4.0))
  (xdg
   (= 3.16.0)))
 (tags
  (topics "to describe" your project)))
