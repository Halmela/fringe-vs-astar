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

Verkon luomisen jälkeen yksittäisiä solmuja käsitellään `usize`-tyyppinä, joka on etumerkitön kokonaisluku, 
joka soveltuu myös indeksointiin.
Sen koon määrittää, kuinka monta tavua tarvitaan viittaamaan mihin tahansa kohtaan muistissa 
(64-bittisissä järjetelmissä se on 8 tavua).
Apufunktioilla saa muutettua solmun kartan koordinaatiksi ja takaisin.
> Olen tällä hetkellä siirtymässä käyttämään 32-bittistä kokonaislukua, mutta vain Fringe tukee sitä tällä hetkellä.

Hakemisto ja moduulirakenne on hieman sekava tällä hetkellä, mutta pääpiirteittäin näin:
- [src](src) sisältää itse sovelluksen toimintaan tarvittavat kilkkeet
- [src/algorithms](src/algorithms) sisältää fringen ja A*:n sekä muuta algoritmien toimintaan liittyvää
- [src/structures](src/structures) sisältää käytetyt tietorakenteet (pl. fringen käyttämä cache, joka löytyy [src/algorithms/fringe](src/algorithms/fringe) 
- [tests](tests) sisältää testit
- [benches](benches) sisältää benchmarkkaukset
- [maps](maps) sisältää kartat

## Saavutetut aika- ja tilavaativuudet (esim. O-analyysit pseudokoodista)
Fringe-haulle ei löydy O-analyysia kirjallisuudesta, mutta uskoisin sen olevan sama kuin A*:n, 
eli  $O(b^d)$, missä $b$ on solmun haarautuvuus (ruudukossa se on $max 7$)
ja $d$ on ratkaisun syvyys (lyhimmän reitin pituus).

### A*
A*:n [päätietorakenne](src/structures/frontier.rs) on Rustin standardikirjastoista löytyvä binäärikeko, 
jonka työntö on $O(~1)$ ja pienimmän otto on $O(log n)$.
Sitä avustamassa on lista pienimmistä löydetyistä arvoista kullekin solmulle.

### Fringe
Fringen päätietorakenteena on kaksi jonoa, joiden päistä otetaan tai laitetaan $O(1)$ ajassa.
Näitä operaatioita kuitenkin tapahtuu reilusti, varmaan useamman $n$:n verran, alati kasvavan määrän.
Rakenteen optimointi keskittyy käsiteltävien solmujen määrän vähentämiseen.

## Suorituskyky- ja O-analyysivertailu (mikäli sopii työn aiheeseen)
Toistaiseksi A* on nopeampi kuin Fringe.
Tarkat speksit ilmestyvät joskus, mutta esim. koneellani suurimman kartan viimeisen reitin vertailu on tällainen: 
```bash
$ cargo build --release
$ target/release/fringe-vs-astar compare -s -n 2000 maps/scene_sp_sax_04.map
 Loading map maps/scene_sp_sax_04.map
Map loaded, creating graph
Using scenario file maps/scene_sp_sax_04.map.scen
Loading problem number 2000
Comparing A* and Fringe search
Problem 2000:
	(3411, 4166) -> (3381, 533)	7712.59278044
Solving using A*
Solved in 2.077095669s
Solving using Fringe search
Solved in 27.26211631s
A* was 25.185020641s faster than Fringe search
```

Alkuvaiheissa molemmilla rakenteilla kesti 2+ minuuttia ratkoa tämä.
A*:n ensimmäinen toteutus köhi jo Berlin256:ssa, joten pitkälle on tultu.

A*:n varmaan voisi pistää johonkin $O(n log n)$-kategoriaan, koska siinä ajetaan iso kasa $O(log n)$-operaatioita,
joskaan ei ihan koko kartan koolla.
Fringen kompleksisuutta en tältä istumalta saa arvioitua, koska yksikään löydetty solmu ei "katoa" mistään listasta,
vaan niitä käydään läpi uudelleen ja uudelleen, edes tarkistaen että voisiko tällä kerralla jatkaa etenemistä.


## Työn mahdolliset puutteet ja parannusehdotukset
CLI:n voisi jatkaa TUI:ksi.
Printit ovat ihan nättejä, mutta niiden selailu isommilla kartoilla vaatii erillisiä komentorivityökaluja.
Tietorakenteen tilan tulostaminen haun tietyssä pisteessä olisi myös nättiä.


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
