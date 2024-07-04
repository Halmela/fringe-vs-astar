# Testausdokumentti
[![codecov](https://codecov.io/github/Halmela/fringe-vs-astar/graph/badge.svg?token=7DFEU4IESG)](https://codecov.io/github/Halmela/fringe-vs-astar)


Testit voi ajaa komennolla `cargo test`.
Tämä sisältää testausmoduuleissa olevat testit ja dokumentaatiossa olevat testit.

Olen tehnyt yksikkötestit parille helpommin pieleen menevälle funktiolle.
Päästä-päähän testaus toimii ajamalla jokaisen `lak104d.map.scen` ongelman ja vertaamalla odotettua tulosta saatuun tulokseen.
Testi epäonnistuu, jos jonkin ongelman virhemarginaali on liian suuri ja silloin printataan kyseisen ongelman numero.
Kyseinen kartta on sen verran pieni, että sitä voi lähteä debuggaamaan `-ssss`-tilassa.


Nopeusregressioita olen havainnut criterion-kirjaston avulla tehdyillä benchmarkeilla.
Ne voi ajaa komennolla `cargo bench`.
Näissä olen käyttänyt Berlin-karttoja eri kokoisina.
Hienot käppyrät näistä tuloksista löytyvät `target/criterion/report/index.html`.


Järeämmän, vähemmän formaalin testin suoritan isoimman kartan viimeisellä testillä:
```bash
cargo build --release
target/release/fringe-vs-astar compare -s -n 2000 maps/scene_sp_sax_04.map
```
