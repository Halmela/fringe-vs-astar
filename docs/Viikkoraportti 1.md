# Viikkoraportti 1
Tällä viikolla olen pienen pallottelun ja ohjaajan konsultoinnin jälkeen päätynyt vertailemaan fringe- ja A*-hakuja keskenään.
Aluksi ajattelemani aiheet olivat parametrisoidun L-systeemin tai Brodal-jonon toteuttaminen, 
mutta ne eivät oikein sopineet tämän kurssin aiheiksi.
Käytin alkuviikosta niiden tutkimiseen muutamia tunteja.
Loppuviikosta olen tutustunut Fringen ja A*:n toimintaan.

Aion toteuttaa Fringen seuraamalla sen esittelevässä alkuperäisessä artikkelissa[^1] määriteltyä pseudokoodia.
Mikäli jää aikaa, toteutan myös samassa paperissa mainittuja A*-optimointeja.

Käytän molemmissa algoritmeissa heuristiikkana diagonaalietäisyyttä[^2].
Koska kulmien yli mennessä matka on √2, kyseessä on tarkemmin oktiilietäisyys.
Esteen "läpi" ei voi mennä diagonaalisesti, vaan se täytyy kiertää.

Pyöräillessäni yksi yö kotiin, mieleeni juolahti ajatus kartan/verkon indeksoinnista Hilbert-käyrän mukaisesti.
Hilbert-käyrä täyttää neliön (ja nykyään myös suorakulmion) täysin, ja tarjoaa siten nD -> 1D projektion,
joka kuitenkin säilyttää verrattain eri indeksien läheisyyden.
Jos tieto tallennetaan 2D-matriisina tai `idx := x + y*leveys`-listana, 
osoitin saattaa joutua pomppimaan välillä yhtä paljon kuin mitä kartan leveys on, 
mutta Hilbertin käyrällä indeksoimalla tämä joissain tapauksissa pienenisi huomattavasti (ainakin nopean päättelyni mukaan).

Olen käyttänyt muutaman tunnin tämän tutkimiseen, ja pistänyt joitain lupaavia papereita talteen, 
mutta hillitsen itseni ja harkitsen sen implementointia vasta, kun olen saanut itse algoritmit toimimaan 
JA minulla on selkeä tapa benchmarkata eri implementaatioiden vaikutusta ohjelman nopeuteen.

Tutkin erilaisia Rustille sopivia projektinhallintatyökaluja.
Aion käyttää GitHub Actionsia päämoottorina, koska silloin minun ei tarvitse pistää erikseen runneria pystyyn.
Rustin työkaluista löytyy valmiiksi dokumentointiin ja testaamiseen liittyviä työkaluja, joten käytän niitä.
Benchmarkkaamiseen harkitsen [Bencher](https://github.com/marketplace/actions/bencher-cli)in käyttöä.
(Flamegraph)[https://github.com/flamegraph-rs/flamegraph] on varmaankin myös hyödyllinen.
 
Olen hieman koodaillut yksinkertaisia palasia ohjelmalle. 
Rust on tuttu kieli, mutta hieman täytyy taas palauttaa muistiin.
Muistinhallinta on rankkaa.

Olen myös leikitellyt ajatuksella algoritmien säikeistämisestä.
A*:ta on yritetty säikeistää vaihtelevin tuloksin, mutta Fringen kanssa tilanne voi olla eri.
Hilbertin käyrän käyttäminen ainakin helpottaisi tilannetta, koska sen lohkominen on helppoa.

## Ajatuksia projektin tilanteesta:
- Hyppäsin ehkä turhan nopeasti määrittelemään traiteja.
Yleensä ensin tehdään implementaatio ja sitten eristetään sen toiminnallisuus traitiksi.
- Tiedoston voi lukea tavuina Stringin sijaan. 
Se voisi tuoda vähän nopeutta, mutta tiedoston luku ei ole kuuma reitti, joten en välitä hirveästi tässä vaiheessa.
- Tiedoston luvun virheenhallinta ei ole hyvässä jamassa, siellä on useampi `.unwrap()`.
- Olen hieman rönsyillyt eri keskittymiskohteiden suhteen
- Ihan ok alku tho

[^1](https://webdocs.cs.ualberta.ca/~holte/Publications/fringe.pdf)
[^2](https://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html#diagonal-distance)
