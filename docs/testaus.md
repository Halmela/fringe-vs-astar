# Testausdokumentti
[![codecov](https://codecov.io/github/Halmela/fringe-vs-astar/graph/badge.svg?token=7DFEU4IESG)](https://codecov.io/github/Halmela/fringe-vs-astar)


Testit voi ajaa komennolla `cargo test`.
Tämä sisältää testausmoduuleissa olevat testit ja dokumentaatiossa olevat testit.

Olen tehnyt yksikkötestit parille helpommin pieleen menevälle funktiolle.
Päästä-päähän testaus toimii ajamalla jokaisen `lak104d.map.scen` ongelman ja vertaamalla odotettua tulosta saatuun tulokseen.
Testi epäonnistuu, jos virhemarginaali on liian suuri.


Nopeusregressioita olen havainnut criterion-kirjaston avulla tehdyillä benchmarkeilla.
Ne voi ajaa komennolla `cargo bench`.
Näissä olen käyttänyt kartan `Berlin_1_256.map` ongelmaa 910, koska läppärini ehtii ajaa sen tarpeeksi monta kertaa tarpeeksi nopeasti,
jotta saan selkeitä tuloksia siitä.

Järeämmän, vähemmän formaalin testin suoritan isoimman kartan viimeisellä testillä:
```bash
cargo build --release
target/release/fringe-vs-astar compare -s -n 2000 maps/scene_sp_sax_04.map
```
