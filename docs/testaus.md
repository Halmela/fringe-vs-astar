# Testausdokumentti
[![codecov](https://codecov.io/github/Halmela/fringe-vs-astar/graph/badge.svg?token=7DFEU4IESG)](https://codecov.io/github/Halmela/fringe-vs-astar)


Testit voi ajaa komennolla `cargo test`.
Tämä sisältää testausmoduuleissa olevat testit ja dokumentaatiossa olevat testit.
Olen tehnyt yksikkötestit parille helpommin pieleen menevälle funktiolle.

Ei ole ollut kauhean kätevää, mutta reitinhaun oikeellisuuden olen tarkistanut ajamalla parista kartasta kaikki ongelmat
ja varmistanut omin silmin, että virhemarginaali skenaariotiedoston pituuden ja algoritmin antaman reitin välillä on alle
0.000001.
Koska tässä testataan vain oikeellisuutta, eikä nopeutta, olen ajanut ne komennoilla
```bash
cargo run -- -s a-star benchmarks/Berlin_1_256.map | less
cargo run -- -s fringe benchmarks/Berlin_1_256.map | less
```
`less`illä voi rauhassa selailla syötteitä ja etsiä "Difference".

Reittien "hyvyyttä" olen katsonut silmillä myös.
Jos ajaa ohjelman ilman `-s`-lippua, printtautuu kartta ja siihen piirretty reitti.

Tyyppiturvallisuuteen olen luottanut paljon.

Nopeusregressioita olen yrittänyt havaita ajamalla
```bash
cargo build --release
target/release/fringe-vs-astar -s compare benchmarks/Berlin_0_1024.map
```
Aikasarjadataa en ole harmikseni kerännyt.


`benchmarks`-kansiosta löytyy iso kasa karttoja, jotka ovat kopioitu [Shortest Path Lab](https://bitbucket.org/shortestpathlab/benchmarks/src/master/grid-maps/)ilta,
joka taas on peilannut osan niistä [Moving AI](https://www.movingai.com/benchmarks/grids.html)lta.
Olen oikeastaan rakentanut koko ohjelmani näillä standardeilla tehtyjen karttojen ja ongelmien ympärille.

