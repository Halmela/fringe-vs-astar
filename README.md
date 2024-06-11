[![codecov](https://codecov.io/github/Halmela/fringe-vs-astar/graph/badge.svg?token=7DFEU4IESG)](https://codecov.io/github/Halmela/fringe-vs-astar)

# fringe-vs-astar
Fringe-haun ja A*-haun toteuttaminen ja vertailu; HY TKT algoritmi-harjoitustyö

## Asennus
- Asenna Rustin työkalut: https://www.rust-lang.org/tools/install
- Kloonaa repositorio ja aja sen juuressa `cargo build --release`.
- Tämä luo binäärin `target/release/fringe-vs-astar`, jonka voi siirtää minne haluaa.

Kehitysversion voi ajaa komennolla `cargo run -- [KOMENNOT]`

## Käyttö
```bash
$ fringe-vs-astar --help
Pathfinders for gridmaps

Usage: fringe-vs-astar [OPTIONS] <MODE> <MAP FILE>

Arguments:
  <MODE>      How program is executed. print-map prints the map ; print prints the map with problems ; a-star solves using A* ; fringe solves using Fringe Search ; compare compares a-star and fringe [possible values: print, print-map, a-star, fringe, compare]
  <MAP FILE>  Path to a file that contains a map

Options:
  -p, --problem-file <PROBLEM FILE>
          Path to a file that contains a set of problems. Default is MAP FILE.scen(ario)
  -n, --problem-number <PROBLEM NUMBER>
          1 indexed indentifier for a problem
  -s, --silent...
          Suppress output. First removes printing of maps, second removes printing of problems, third removes printing of everything
  -h, --help
          Print help
  -V, --version
```

## Testit
```bash
$ cargo test 
```

## Benchmark
```bash
$ cargo bench
```


## Dokumentaatio
- [Määrittelydokumentti](/docs/m%C3%A4%C3%A4rittely.md)
- [Testausdokumentti](/docs/testaus.md)
- [Toteutusdokumentti](/docs/toteutus.md)

### Viikkoraportit
- [Viikko 1](/docs/Viikkoraportti%201.md)
- [Viikko 2](/docs/Viikkoraportti%202.md)
- [Viikko 3](/docs/Viikkoraportti%203.md)
- [Viikko 4](/docs/Viikkoraportti%204.md)

### Sisäinen dokumentaatio
```bash
$ cargo doc --open
```


