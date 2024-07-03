# Toteutusdokumentti

## Ohjelman yleisrakenne
Ohjelmaa käsitellään komentorivikäyttöliittymän (CLI) kautta.
CLIn keräämät tiedot annetaan eteenpäin Kontekstille, joka rakentaa Kartan ja sen pohjalta Verkon.
Konteksti kerää myös tiedot ongelmista ja tallettaa ne.

Kontekstilla on viisi tilaa, joihin sen ajo voi mennä:
- print
  - Tulosta Kartta ja mahdolliset ongelmat
- print-map
  - Tulosta Kartta
- a-star
  - Ratkaise ongelma tai ongelmia Verkosta A*-haulla.
- fringe
  - Ratkaise ongelma tai ongelmia Verkosta Fringe-haulla.
- compare
  - Ratkaise ongelma tai ongelmia Verkosta molempia hakuja käyttäen ja vertaile niiden aikoja keskenään.


Verkon luomisen jälkeen yksittäisiä solmuja käsitellään `u32`-tyyppinä, joka on 32-bittinen etumerkitön kokonaisluku. 
Olen antanut sille aliaksen `Node`.
Apufunktioilla saa muutettua solmun kartan koordinaatiksi ja takaisin.

Hakemisto ja moduulirakenne on hieman sekava tällä hetkellä, mutta pääpiirteittäin näin:
- [src](src) sisältää itse sovelluksen toimintaan tarvittavat kilkkeet
- [src/algorithms](src/algorithms) sisältää Fringen ja A*:n sekä muuta algoritmien toimintaan liittyvää. Molemmille hauille on omat moduulit, jotka sisältävät niille spesifejä rakenteita
- [src/structures](src/structures) sisältää yleiset tietorakenteet
- [tests](tests) sisältää testit
- [benches](benches) sisältää benchmarkkaukset
- [maps](maps) sisältää kartat

## Saavutetut aika- ja tilavaativuudet (esim. O-analyysit pseudokoodista)
Fringe-haulle ei löydy O-analyysia kirjallisuudesta, mutta uskoisin sen olevan sama kuin A*:n, 
eli  $O(b^d)$, missä $b$ on solmun haarautuvuus (ruudukossa se on $max 8$)
ja $d$ on ratkaisun syvyys (lyhimmän reitin pituus).

### A*
A*:n [päätietorakenne](src/structures/frontier.rs) on Rustin standardikirjastoista löytyvä binäärikeko, 
jonka työntö on $O(~1)$ ja pienimmän otto on $O(log n)$.
Sitä avustamassa on taulukko, josta löytyy jokaisesta solmusta löydettyjä arvoja.

Isoin optimointi varmaankin on, että kun solmu lisätään kekoon, se merkitään avonaiseksi, ja kun se otetaan pois, se merkitään suljetuksi.
Jo suljettuja solmuja ei enää käsitellä uudestaan.
Tämä on reilusti nopeampaa kuin solmun hakeminen keosta ja sen jälkeen poistaminen, vaikka tämä kasvattaakin kekoa.


### Fringe
Tämän tietorakenteen (`Fringe`) toteutus on elänyt paljon, joten selitän vaiheet pääpiirteittäin, jotta lukija voisi ymmärtää nykyistä tilannetta.
Vakiona on pysynyt itse algoritmi, ja löydettyjen solmujen eri arvojen tallentaminen muistiin (`Cache`).

Fringen päätietorakenteena on kaksi jonoa, joiden päistä otetaan tai laitetaan $O(1)$ ajassa.
Näitä operaatioita kuitenkin tapahtuu reilusti, varmaan useamman $n$:n verran, alati kasvavan määrän.

[Kirjallisuudessa](https://webdocs.cs.ualberta.ca/~holte/Publications/fringe.pdf) tämä toteutettiin kahteen suuntaan linkitettynä listana, 
mutta kieleni valinnasta johtuen en halunnut toteuttaa sitä kyseisellä tavalla. Raakojen osoittimien pyörittely on asia, jota Rustissa nimenomaan pyritään välttämään,
ja standardikirjastosta löytyvästä [LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)-rakenteesta ei löytynyt valmiiksi tehokkaita funktioita solmujen poistamiseen listan keskeltä 
(tätä kirjoittaessa ymmärrän, miten tämän voisi saavuttaa, mutta en koe sitä silti sen arvoisena), 
joten päädyin käyttämään kahta [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)-rakennetta, joka toimivat samaan aikaan pinona ja jonona.
Yhdellä läpikäynnillä mennään koko now läpi, uudet löydetyt solmut lisätään kärkeen, liian kalliit solmut menevät lateriin ja kierroksen lopuksi later muutetaan now'ksi.

Seuraavaksi keksin tallentaa, onko solmu juuri sillä hetkellä käsiteltävänä. Tämän ansiosta ei tarvitse poistaa solmuja, voi vain jättää ne käsittelemättä.
Alkion poistaminen oli aiemmin $O(n)$-operaatio.

Tästä seuraava versio oli muuttaa molemmat listat [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)-listoiksi, joka toimii myös pinona.
Jossain vaiheessa ymmärsin, ettei kumpaankaan käsiteltävään listaan oikeasti edes lisätä solmuja molempiin päihin ja siksi jonomaisuus oli turhaa.
> Myöhemmin palasin `VecDeque`en, koska keksin tavan käsitellä molempia listoja samassa rakenteessa, mutta se ei ollut elegantti ja jäi vain lyhytaikaiseksi kokeiluksi

Tässä vaiheessa jokainen operaatio oli erittäin nopea, mutta niiden määrä oli massiivinen.
Yhden iteraation aikana käsitellään jokainen later-listan solmu, mutta pahimmillaan vain yksi niistä laajennetaan.
Jokaisen haun tilan printtaaminen selkeytti tämän ymmärtämistä, koska siinä _selkeästi_ näkyi, että turhiakin solmuja käydään läpi uudestaan ja uudestaan.

#### Ämpärit

Algoritmin esitelleessä paperissa mainittiin ohimennen
> Fringe Search could be modified to traverse the fringe in a different order, for example, by using buckets to partially sort the fringe.

Tein pari dokumentoimatonta apufunktiota[^1], joiden avulla tarkastelin millaisia g-arvojakaumia fringestä löytyy eri hetkinä.
Havaitsin, että:
1. Samaan aikaan fringessä on maksimissaan kokonaislukuosaltaan neljää erilaista arvoa (ainakin tällä heuristiikalla)
2. Kokonaislukuarvo on monotonisesti kasvava
3. Jotkut kokonaisluvut skipataan (mutta ne eivät myöskään myöhemmin ilmesty)

Näiden pohjalta päädyin käyttämään 8 eri ämpäriä, koska se on seuraava 2:n potenssi ja siten sille voi tehdä tehokasta jakojäännöslaskentaa.
Tein auttavan `Bucket`-enumin, jolla on tasan 8 arvoa, ja jonka voi luoda g-arvon pohjalta ja jota voi käyttää indeksinä.
Sen jälkeen korvasin later-listan 8 ämpärillä, joista vain nykyinen tyhjennetään now-listaksi kierroksen päättyessä.
Iso kasa solmuista käydään vieläkin monta kertaa turhaan läpi, mutta ainakaan kaikkia ei käydä. Sain tällä muistaakseni aika mielyttävän nopeutuksen.

Olin jo lopettamassa rakenteen kehittelyä, mutta sitten seuraavat faktat iskivät mieleeni:
1. Iso osa ämpäreistä on tyhjinä 
2. Now-lista on vielä erillinen rakenne
3. Ämpäreitä kopioidaan ympäriinsä ja se on yleensä hidasta
4. Later-listan ei tarvitse olla lajiteltu millään tavalla (paperissa annettu ainoa ehto oli, että ensimmäistä kertaa löydetyt solmut on käsiteltävä mahdollisimman pian)

Jotenkin tämä ajoi minut yhdistämään kaikki ämpärit yhdeksi pitkäksi vektoriksi (`Buckets`) ja tauluksi, jossa ylläpidetään jokaisen ämpärin senhetkistä indeksiä (`Indexes`).
`Buckets` allokoi muistin kerran ja täyttää sen `None`-arvolla, koko on tarpeeksi iso pitämään ainakin Verkon jokaisen solmun sisällään.
Now-lista tallennetaan aiemman kierroksen varmasti tyhjään ämpäriin, kierroksen loputtua nykyinen ämpäri muuttuu pseudo-now-listaksi, jonka perään rakennetaan uusi ämpärilista.
Pseudo-now'sta poistaminen tapahtuu vaihtamalla alkion paikkaa ensin pseudo-now'n viimeisen solmun kanssa ja sen jälkeen uudelleen ämpärin viimeisen solmun kanssa, 
tyhjentämällä se ja päivittämällä indeksit.

```
  [a,b,c|d,e,f] -> [c,b,a|d,e,f] -> [c,b,f|d,e,a] -> [c,b,f|d,e,_] -> [c,b|f,d,e,_]
```
Kahden alkion paikan vaihtaminen on $O(1)$ operaatio, ja tämä ei jätä yhtäkään listojen sisäistä alkiota tyhjäksi.

Tämä indeksikikkailu on erittäin hauras, mutta se toimii.


Yhdessä vaiheessa kokeilin jonojen pitämistä aina järjestettynä ja f_limitin aggressiivista nostamista, mutta en saanut sillä suurempia tuloksia.
Ehkä jos sen yhdistäisi nykyiseen implementaatioon ja sallisi tyhjien alkioiden sisällytyksen listoissa.


## Suorituskyky- ja O-analyysivertailu (mikäli sopii työn aiheeseen)
Pienemmillä kartoilla Fringe on ihan vähän nopeampi kuin A*, mutta A* alkaa voittaa isommilla kartoilla, joskin nykyään vähemmän kuin ennen (ks. tämän tiedoston versiohistoria).
Isoimman kartan pisin testi:
```bash
$ cargo run --release -- compare -n 2000 maps/scene_sp_sax_04.map

Loading map maps/scene_sp_sax_04.map
Map loaded, creating graph
Comparing A* and Fringe search
Problem 2000:
	(3411, 4166) -> (3381, 533)	7712.593

Solving using A*
Solved in 1.469222516s
Solving using Fringe search
Solved in 5.794926648s
A* was 4.325704132s faster than Fringe search
```

Kannattaa ajella eri kokoisten karttojen viimeisiä testejä (`cargo run -- print [kartta] | tail`), jotta saa näppituntumaa tähän.


A*:n varmaan voisi pistää johonkin $O(n log n)$-kategoriaan, koska siinä ajetaan iso kasa $O(log n)$-operaatioita,
joskaan ei ihan koko kartan koolla. Virallisesti se on kai $O(d^b)$.

Fringestä en ihan tiedä, varsinkaan omien säätöjeni kanssa. Se ei yhdelläkään kierroksella mene jokaista solmua läpi, mutta samoja ämpäreitä kuitenkin junnataan monta kertaa.
Kaipa se on sama kuin A*.


## Työn mahdolliset puutteet ja parannusehdotukset
CLI:n voisi jatkaa TUI:ksi.
Printit ovat ihan nättejä, mutta niiden selailu isommilla kartoilla vaatii erillisiä komentorivityökaluja, samoin "animaatioiden".
Teknistä velkaa on varsinkin kontekstiin ja solveriin liittyen.
CLI ei myöskään ole kauhean intuitiivinen eri -s -lippuineen.

## Laajojen kielimallien (ChatGPT yms.) käyttö. Mainitse mitä mallia on käytetty ja miten. Mainitse myös mikäli et ole käyttänyt. Tämä on tärkeää!

<details>
<summary>
Kysyin ChatGPT4o:lta aika-avaruuksista, koska en oikein löytänyt tietoa muualta. 
Tekstistä voi päätellä, että sekään ei oikein tiedä.
</summary>
> what is the time complexity of Fringe Search?



Fringe Search is a best-first search algorithm designed to be memory-efficient while maintaining optimal performance. It is particularly useful for finding shortest paths in graphs. The time complexity of Fringe Search is determined by the number of nodes it expands, similar to other heuristic search algorithms like A*.

### Time Complexity of Fringe Search

1. **Worst-case Time Complexity**:
   - The worst-case time complexity of Fringe Search is \(O(b^d)\), where \(b\) is the branching factor (the average number of successors per state), and \(d\) is the depth of the shallowest goal node.
   - This complexity arises because, in the worst case, the algorithm may have to explore all nodes up to depth \(d\) to find the goal.

2. **Best-case Time Complexity**:
   - In the best case, the time complexity is much lower, potentially as low as \(O(d)\) if the goal is found early in the search process.

### Factors Influencing Time Complexity

- **Heuristic Quality**:
  - The efficiency of Fringe Search heavily depends on the quality of the heuristic used. A better heuristic (closer to the true cost) can significantly reduce the number of nodes expanded, thereby improving performance.
  - If the heuristic is perfect (equal to the true cost to reach the goal), the time complexity can be close to linear, \(O(d)\).

- **Branching Factor**:
  - The branching factor \(b\) greatly influences the number of nodes that need to be explored. A higher branching factor results in exponentially more nodes to consider.

- **Depth of the Goal Node**:
  - The depth \(d\) of the goal node in the search space determines how deep the algorithm needs to go. The deeper the goal, the more nodes need to be expanded.

### Comparison with A*

- **Similarities**:
  - Both Fringe Search and A* are heuristic search algorithms that aim to find the shortest path.
  - The worst-case time complexity of both algorithms is \(O(b^d)\).

- **Differences**:
  - Fringe Search is designed to be more memory-efficient than A*. A* maintains an open list of all nodes that need to be explored, which can grow very large. In contrast, Fringe Search uses an iterative approach with a limited memory footprint.

### Practical Considerations

- **Memory Efficiency**:
  - Fringe Search is particularly advantageous in scenarios where memory is a limiting factor. It requires less memory than A* by not keeping all nodes in memory simultaneously.

- **Iterative Approach**:
  - Fringe Search uses a series of depth-limited searches, refining the depth limit in each iteration. This iterative deepening helps manage memory usage more effectively compared to A*'s open list.

### Summary

- **Worst-case Time Complexity**: \(O(b^d)\)
- **Best-case Time Complexity**: Potentially \(O(d)\) with a perfect heuristic
- **Factors**: Heuristic quality, branching factor \(b\), and depth \(d\) of the goal node

Fringe Search offers a balance between time efficiency and memory usage, making it a practical choice for large search spaces where memory constraints are significant.
</details

## Viitteet
https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#time-complexity

[^1]: (https://github.com/Halmela/fringe-vs-astar/blob/cd1d4726e56ccd508d225d421d5cc27325211817/src/algorithms/fringe.rs#L105-L130)
