[![codecov](https://codecov.io/github/Halmela/fringe-vs-astar/graph/badge.svg?token=7DFEU4IESG)](https://codecov.io/github/Halmela/fringe-vs-astar)

# fringe-vs-astar
Fringe-haun ja A*-haun toteuttaminen ja vertailu; HY TKT algoritmi-harjoitustyö

## Asennus
Kloonaa repositorio ja aja sen juuressa `cargo build --release`.
Tämä luo binäärin `target/release/fringe-vs-astar` jonka voi siirtää minne haluaa.

## Käyttö
```bash
$ fringe-vs-astar --help
Pathfinder comparison. Currently only A* is supported

Usage: fringe-vs-astar [OPTIONS] <MODE> <MAP FILE>

Arguments:
  <MODE>      [possible values: print, solve]
  <MAP FILE>  Path to a file that contains a map

Options:
  -p, --problem-file <PROBLEM FILE>
          Path to a file that contains a set of problems. Default is MAP FILE.scen(ario)
  -n, --problem-number <PROBLEM NUMBER>
          1 indexed indentifier for a problem
  -s, --silent
          Suppress drawing of maps. Suggested for large maps
  -h, --help
          Print help
  -V, --version
          Print version
```


## Dokumentaatio
- [Määrittelydokumentti](/docs/m%C3%A4%C3%A4rittely.md)
- [Testausdokumentti](/docs/testaus.md)

### Viikkoraportit
- [Viikko 1](/docs/Viikkoraportti%201.md)
- [Viikko 2](/docs/Viikkoraportti%202.md)
- [Viikko 3](/docs/Viikkoraportti%203.md)

### Sisäinen dokumentaatio
```bash
$ cargo doc --open
```
