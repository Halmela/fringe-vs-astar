# Määrittelydokumentti

## Ydin
Vertailen harjoitustyössäni keskenään kahta lyhimmän reitin löytävää algoritmia, Fringe Searchia ja A*:a.
Reitinhaku toteutetaan 8-suuntaisesti ruudukossa, jossa osa ruuduista on kuljettavissa ja osa on ei.


## Kielet
Ohjelma kirjoitetaan Rust-ohjelmointikielellä.
Luon sen sisäisillä työkaluilla mahdollisimman suuren osan dokumentaatiosta ja testauksista.
Kattavuuden seurantaan käytän [cargo-llvm-cov](https://lib.rs/crates/cargo-llvm-cov)-työkalua.

Dokumentaation kirjoitan suomeksi.

Voin vertaisarvioida suomeksi tai englanniksi kirjoitettua dokumentaatiota.
Osaan Rustia, Haskellia, JavaScriptia ja Pythonia.
Ymmärrän C++:aa, mutten sujuvasti.
En halua koskea Javaan.


## Algoritmit
Käytän molemmissa algoritmeissa heuristiikkana diagonaalietäisyyttä[^1].
Koska kulmien yli mennessä matka on √2, kyseessä on tarkemmin oktiilietäisyys.


### A*

### Fringe search
[Fringe searchin esittely](https://webdocs.cs.ualberta.ca/~holte/Publications/fringe.pdf)


## Testisyötteet
Ohjelman testaukseen käytän syötteenä [erinäisiä karttoja Shortest Paths Labilta](https://bitbucket.org/shortestpathlab/benchmarks/src/master/grid-maps/).
Sieltä löytyy niin pieniä kuin suuriakin karttoja ja jokaiselle on määritelty läjä ongelmia ja ratkaisuja.
Tulen käyttämään pelkkiä grid mapeja.

Toiminnallisuuden oikeellisuuden varmistamiseen alkuvaiheissa ja esittelyssä aion käyttää local-kansiosta löytyviä testejä,
loppuvaiheessa algoritmien tehokkuuden vertailuun aion käyttää Iron Harvest -settiä, jonka suurin kartta `scene_sp_sax_04.map` on  6336x5728 ruutua.

## Opinto-ohjelma
Tämä on osa tietojenkäsittelytieteen kandidaattiopintoja.

[^1]: (https://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#diagonal-distance)
