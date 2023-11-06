FROM mcr.microsoft.com/devcontainers/base:debian

ARG DEFAULT_SWITCH=4.14.1

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -qq && \
    apt-get install -yqq ocaml opam pkg-config libgmp-dev nodejs npm

USER vscode

RUN opam init --disable-sandboxing --auto-setup -q && \
    opam switch create $DEFAULT_SWITCH && \
    opam switch set $DEFAULT_SWITCH && \
    eval $(opam env)

RUN opam install -yq ocaml-lsp-server ocamlformat odig dune
