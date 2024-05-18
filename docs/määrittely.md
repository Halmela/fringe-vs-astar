# Määrittelydokumentti

## Ydin
Vertailen harjoitustyössäni keskenään kahta lyhimmän reitin löytävää algoritmia, Fringe Searchia ja A*:a.
Reitinhaku toteutetaan 8-suuntaisesti ruudukossa, jossa osa ruuduista on kuljettavissa ja osa on ei.


## Kielet
Ohjelma kirjoitetaan Rust-ohjelmointikielellä.
Luon sen sisäisillä työkaluilla mahdollisimman suuren osan dokumentaatiosta ja testauksista.

Työhöni liittyvät dokumentit kirjoitan suomeksi, ohjelman sisäisen dokumentaation kirjoitan englanniksi.

Voin vertaisarvioida suomeksi tai englanniksi kirjoitettuja töitä.
Osaan Rustia, Haskellia, JavaScriptia ja Pythonia.
Ymmärrän C++:aa, mutten sujuvasti.
En halua koskea Javaan.


## Algoritmit
- A*
- Fringe search


## Ohjelma
Ohjelma kykenee 
- lukemaan `.map`-tiedoston ja luomaan sen pohjalta verkon
- lukemaan `.map.scen`-tiedoston ja luomaan sen pohjalta sarjan testejä
- esittämään kartan ja reitin (sekä reitin löytämiseen tarvitut ruudut), mikäli se on tarpeeksi pieni 
- esittämään ja vertailemaan kahden eri algoritmin tuloksia
- ajamaan parametrien perusteella joko kokonaisen testipatterin, tietyn "ämpärin" testejä tai tietyn numeron

## Testisyötteet
Ohjelman testaukseen käytän syötteenä [erinäisiä karttoja Shortest Paths Labilta](https://bitbucket.org/shortestpathlab/benchmarks/src/master/grid-maps/).
Sieltä löytyy niin pieniä kuin suuriakin karttoja ja jokaiselle on määritelty läjä ongelmia ja ratkaisuja.
Tulen käyttämään pelkkiä grid mapeja.

Toiminnallisuuden oikeellisuuden varmistamiseen alkuvaiheissa ja esittelyssä aion käyttää local-kansiosta löytyviä testejä,
loppuvaiheessa algoritmien tehokkuuden vertailuun aion käyttää Iron Harvest -settiä, jonka suurin kartta `scene_sp_sax_04.map` on  6336x5728 ruutua.

## Opinto-ohjelma
Tämä on osa tietojenkäsittelytieteen kandidaattiopintoja.

