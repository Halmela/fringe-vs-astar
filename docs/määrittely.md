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
### A*
A* etsii lyhimmän reitin prioriteettijonon ja heuristiikan perusteella.
Sen aikavaatimus on $O(b^d)$, missä $b$ on solmun haarautuvuus (ruudukossa se on $max 7$)
ja $d$ on ratkaisun syvyys (lyhimmän reitin pituus).
Valittu heuristiikka vaikuttaa $b$:hen dramaattisesti, aion käyttää [oktiilietäisyyttä](https://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#diagonal-distance).  

 
### Fringe search
Fringe Search (FS) kuuluu samaan perheeseen kuin A*, 
suurimpana erona on etsittävien solmujen tallentaminen kahteen jonoon keon sijasta.
A* saattaa löytää maalisolmun pienemmällä iteraatiomäärällä kuin FS, 
mutta FS:n sisäiset tietorakenteet ovat tehokkaampia.
En löytänyt tietoa FS:n aikavaatimuksesta, mutta oletan sen olevan sama kuin A*:lla, eli $O(b^d)$.
FS:n muistivaatimus on pienempi.

## Ohjelma
Ohjelma kykenee 
- lukemaan `.map`-tiedoston ja luomaan sen pohjalta verkon
- lukemaan `.map.scen`-tiedoston ja luomaan sen pohjalta sarjan testejä
- esittämään kartan ja reitin (sekä reitin löytämiseen tarvitut ruudut), mikäli se on tarpeeksi pieni 
- ajamaan, esittämään ja vertailemaan kahden eri algoritmin tuloksia
- ajamaan parametrien perusteella joko kokonaisen testipatterin, tietyn "ämpärin" testejä tai tietyn numeron


## Testisyötteet
Ohjelman testaukseen käytän syötteenä [erinäisiä karttoja Shortest Paths Labilta](https://bitbucket.org/shortestpathlab/benchmarks/src/master/grid-maps/).
Sieltä löytyy niin pieniä kuin suuriakin karttoja ja jokaiselle on määritelty läjä ongelmia ja ratkaisuja.
Tulen käyttämään pelkkiä grid mapeja.

Toiminnallisuuden oikeellisuuden varmistamiseen alkuvaiheissa ja esittelyssä aion käyttää local-kansiosta löytyviä pieniä testejä,
loppuvaiheessa algoritmien tehokkuuden vertailuun aion käyttää Iron Harvest -settiä, jonka suurin kartta `scene_sp_sax_04.map` on  6336x5728 ruutua.

## Opinto-ohjelma
Tämä on osa tietojenkäsittelytieteen kandidaattiopintoja.

