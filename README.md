# SQL transaction test

[![BuildAndTest](https://github.com/PragmaTwice/sql-transaction-test/workflows/BuildAndTest/badge.svg)](https://github.com/PragmaTwice/sql-transaction-test/actions?query=workflow%3ABuildAndTest)
[![codecov](https://codecov.io/gh/PragmaTwice/sql-transaction-test/branch/master/graph/badge.svg?token=TZ57Z0311X)](https://codecov.io/gh/PragmaTwice/sql-transaction-test)

*test SQL transaction by running all merged permutation from two SQL files*

:closed_book: [Documentation](https://sql-transaction-test.surge.sh/sql_transaction_test/)

## Quick Start

```shell
$ RUST_LOG=debug cargo run -- -u mysql://username:password@host:port/db-name asset/a.sql asset/b.sql 
```
You can check the result of this command in step "Run example" from the latest workflow run of [Actions](https://github.com/PragmaTwice/sql-transaction-test/actions?query=workflow%3ABuildAndTest), where a TiDB service was pulled up in CI for testing.

## Functions
The two SQL files will be split by newlines, trying all cases where they are cross-permutated while keeping the internal order of each file unchanged. 

The sequence of SQL statements under each permutation will be submitted to the server.

## Features
- permutation calculation by bit vector
- data traversing with iterator patterns
- unit testing and real-world simulation in CI
