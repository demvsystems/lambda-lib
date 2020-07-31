# lambda-lib

Zentrale Lib für Lambda übergreifende Funktionalitäten

## Alle Lambdas betreffendes Makefile
Funktioniert nur, wenn die Lambdas im selben Ordner wie die `lambda-lib` liegen

 - `update_all`: Führt `cargo update` in allen Lambdas aus
 - `build_all`: Führt `cargo build` in allen Lambdas aus
 - `test_all`: Führt `make test` in allen Lambdas aus
 - `aws_all`: Führt `make aws` in allen Lambdas aus
