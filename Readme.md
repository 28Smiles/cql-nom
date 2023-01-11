[![Crates](https://badgen.net/crates/v/cql-nom)](https://crates.io/crates/cql-nom)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/28Smiles/cql-nom/actions/workflows/ci.yml/badge.svg)](https://github.com/28Smiles/cql-nom/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/28Smiles/cql-nom/badge.svg)](https://coveralls.io/github/28Smiles/cql-nom)
[![Latest Stable](https://img.shields.io/github/v/release/28Smiles/cql-nom?label=latest%20stable)](https://github.com/28Smiles/cql-nom/releases/latest)
[![Latest Release](https://img.shields.io/github/v/release/28Smiles/cql-nom?include_prereleases&label=latest%20release)](https://github.com/28Smiles/cql-nom/releases)

# CQL-Nom - CQL Parser

[CQL-Nom](https://github.com/28Smiles/cql-nom) is a parser for the [Cassandra Query Language](https://cassandra.apache.org/doc/latest/cql/) 
(CQL) written in Rust, using the [nom](https://github.com/rust-bakery/nom) parser combinator library.
It currently supports the following CQL Statements:

 - [x] `CREATE TABLE`
 - [x] `CREATE TYPE`
 - [ ] `CREATE MATERIALIZED VIEW`
 - [ ] `CREATE INDEX`

For now those are all statements planned to be supported. If you have any suggestions, feel free to open an issue.
