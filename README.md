# lambda-lib

Zentrale Lib für Lambda übergreifende Funktionalitäten

## Alle Lambdas betreffendes Makefile
Funktioniert nur, wenn die Lambdas im selben Ordner wie die `lambda-lib` liegen

 - `update`: Führt `cargo update` in allen Lambdas aus
 - `build`: Führt `cargo build` in allen Lambdas aus
 - `test`: Führt `make test` in allen Lambdas aus
 - `aws`: Führt `make aws` in allen Lambdas aus
