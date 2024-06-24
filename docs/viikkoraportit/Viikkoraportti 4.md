# Viikkoraportti 4

Pistin oikeellisuuden testaamisen ja benchmarkaamisen kuntoon.
A* sai kertaluokan nopeutuksen, Fringe pieniä parannuksia.

A* ratkesi hylkäämällä yrityksen tallentaa ylimääräistä tietoa keon sisällöstä.
Kekoon pistäminen ja sieltä pois ottaminen ovat verrattain halpoja operaatioita, joten niitä voi tehdä myös "turhaan".
Pidin ennen tilaa keon sisällöstä, jotta sieltä voisi poistaa alkioita tarvittaessa, 
mutta tämä on hitaampaa kuin ylimääräisten operaatioiden tekeminen.

Fringe on hitaampi nyt.
Sen yhden paperin mukaan, joka on ainoa lähteeni aiheesta, tämän ei ehkä pitäisi olla näin.
Toisaalta siinä käytettiin tietorakenteena kahteen suuntaan linkattua listaa, minulla taas on kaksi jonoa.
Tässäkin poisto olisi hidas operaatio, vaikka kuinka yrittäisi pitää tilaa yllä, joten annan listojen täyttyä.
Keksisinpä tavan lisätä vähemmän altioita myöhemmin-jonoon...

Erilaiset heuristiikat (esim. ALT), mutta en ole vielä mennyt siihen.
Haluaisin saada Fringen nopeampaan kuntoon, uskon jonkin ruman säädön olevan ihan kulman takana.

Optimointi on nyt helpompaa ja turvallisempaa, kun ensin testata oikeellisuuden ja 
voin katsella benchaus-tuloksia ja vertailla niitä menneisyyteen.
