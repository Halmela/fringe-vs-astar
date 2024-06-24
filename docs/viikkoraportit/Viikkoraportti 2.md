# Viikkoraportti 2

## A*
Implementoin A*:n ensin funktiona, sitten eristin sen omaksi tietueekseen.
Suunnitelmissa olisi vakiinnuttaa sen pohjalta trait, jonka myös Fringe-haku tulee toteuttamaan.

Seurasin ensin [Wikipediasta löytyvää toteutusta](https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode), 
sen jälkeen enemmän [Red Blob Gamesin python-versiota](https://www.redblobgames.com/pathfinding/a-star/implementation.html). 

Aluksi sisäiset tietorakenteet (kaikki standardikirjastosta) olivat 
- hajautustaulu verkon tilalle
- binäärikeko prioriteettien tallentamiseen
- hajautustaulu vanhemmuuksille
- hajautustaulu alimmalle löydetylle painolle

Nämä osoittautuivat flamegraphien (liekkikaaviot?) perusteella pullonkauloiksi.
Ensin vaihdoin verkon esityksen taulukoksi (`taulu[x][y]`).
Sen jälkeen yhdistin vanhemmuudet ja alimman löydetyn painon hajautustaulut yhdeksi taulukoksi.

Binäärikeon nopeusongelmat paistoivat kovaa läpi, koska sinne lisättiin paljon solmuja, osa useaan otteeseen.
Binäärikeon hitaus kasvaa sen koon myötä, joten aloin vähentämään sinne lisättyjen solmujen määrää.
Tallensin alimman löydetyn prioriteetin jo valmiiksi olemassa olevaan taulukkoon.
Ennen kekoon asettamista uutta prioriteettia verrattiin vanhaan alimpaan löytyneeseen ja niistä alempaa käytettiin
(jos uusi oli pienempi, vanhempi poistettiin keosta).
Sain tällä kertaluokkaa parempia nopeuksia, mutta nyt pullonkaula oli poisto-operaatio, 
jossa koko keko käytiin läpi, rakennettiin uudelleen ilman poistettavaa alkiota ja vasta jälkeen lisättiin uusi.

Uudet nopeusparannukset tulivat muokkaamalla poisto-operaatio käymään koko keko läpi, 
muokkaamalla halutulle alkiolle uusi arvo ja sen jälkeen rakentamalla uudelleen.
Tässä vaiheessa suurimman karttani viimeinen (vaikein) kesti 2min23s läppärilläni.

Eriytin keon ja sen käsittelyn omaksi Frontier-tietueeksi, jonka seurauksena itse algoritmi on mielestäni aika helposti luettavissa.

En usko, että voin parantaa binäärikeko-toteutusta lisää.
Olen tutkinut rakenteita, jotka tukevat painoarvon muuttamista ja päätynyt Pairing heap (parittava keko?)-rakenteeseen.
Totetutan sen myöhemmässä vaiheessa. 


### Korrektius
Olen huumassani jättänyt testit tekemättä ja katsonut vain karttojen printtejä ja toivottuja pituuksia.
Vaikuttaa ihan oikealta nyt, aiemmin ei niinkään.
Alkuvaiheessa loin väärän kokoisen taulukon, jota myös indeksoin väärin, jonka tuloksena oli hassuja karttoja väärillä naapureilla.

Useimmissa kartoissa A*:n löytämän reitin pituuden ja skenaario-tiedostosta löytyvän pituuden ero menee liukulukujen virhemarginaaliin,
mutta `adaptive-depth-1`:ssa virhemarginaali on jopa 0,755665[^virhe]. 
Tämä on vähän pelottavaa.

[^virhe]: Virheen koon laskin pistämällä tulosteen tiedostoon `result` ja ajamalla komentorivillä komennon
  ```bash
  rg -n Difference result | sed 's/\:.*//g' | xargs -i expr 1 + {} | xargs -i sed -e "{}q;d" result | sed -e 's/\t//g' | awk '{s+=$1}END{print s/NR}'
  ```
  Tähän tarvitsee komentorivityökalun [ripgrep](https://github.com/BurntSushi/ripgrep).


## CLI
Komentorivikäyttöliittymä ratkaisee suurimmat murheeni toistaiseksi.
Sillä voi printata kartan tai ongelman, ja ratkaista tiedostosta tietyn ongelman tai kaikki ongelmat.
Ongelman antaminen komentoriviltä toisi vähän vaivaa, joten en nyt keskity siihen.
Ongelmat luetaan joko käyttäjän antamasta tiedostosta tai  `.scen`- tai `.scenario`-päätteisestä tiedostosta, 
jolla on muuten sama polku kuin karttatiedostolla.

## Rakenne
Projektin koko ja siten kompleksisuus on kasvanut reilusti, joten olen erotellut eri osia pienempiin moduuleihin.
Hajosin hieman jatkuvaan päätiedoston muokkaamiseen, kun halusin ajaa eri karttoja ja ongelmia, joten loin konteksti-tietueen, 
joka hallitsee kartan lukua, verkon luomista kartan perusteella, ongelmien lukemista ja luomista, ja lopulta reitinhaun aloittamista ja tulostan tulostamista. 

Aiempi halukkuuteni tehdä traiteja liian aikaisin hieman potkii perseelle, mutta ainakin voin nyt helposti osoittaa mitkä toteutukset toimivat ja mitkä eivät.
Samaa rakennetta aion toki käyttää eri reitinhakualgoritmien valitsemiseen, eli otan tämän treeninä, jonka välivaiheet voin joskus pyyhkiä pois, jos niin haluan.


## Testit
Puuttuvat.

## Tulevaisuus
Ensi viikolla aion toteuttaa Fringe Searchin.
