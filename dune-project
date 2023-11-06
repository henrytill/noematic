(lang dune 3.11)

(name noematic)

(generate_opam_files true)

(source
 (github henrytill/noematic))

(authors "Henry Till <henrytill@gmail.com>")

(maintainers "Henry Till <henrytill@gmail.com>")

(license ISC)

(package
 (name noematic)
 (synopsis "A short synopsis")
 (description "A longer description")
 (allow_empty)
 (depends
  ocaml
  dune
  (js_of_ocaml
   (>= 5.4.0)))
 (tags
  (topics "to describe" your project)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project
