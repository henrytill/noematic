#!/usr/bin/env bash

export TERM=dumb
export OPAMCONFIRMLEVEL=yes

opam install -t --deps .
