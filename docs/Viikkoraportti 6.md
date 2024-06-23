# Viikkoraportti 6

Fringen tunkkaamista ja printtausten saamista näteiksi. Refaktorointia.

Fringen muutin käyttämään ämpärimaista lajittelua arvion kokonaisluvun mukaan.
Kyseistä menetelmää mietittiin lähdepaperissa. 
Päädyin toteuttamaan sen, koska huomasin algoritmin testaavan samaa laajaa rintamaa kerta toisensa jälkeen "turhaan".

Ajattelin, että hyvä arvio olisi jakaa solmut ämpäreihin niiden g-arvon (matkan hinta + heuristiikka) kokonaisluku-osan mukaan.
Empiirisesti havaitsin, että jokaisessa haun vaiheessa käsitellään max 4 eri tällaista kokonaislukua.
Tämä motivoi minua määrittelemään ennalta neljä eri later-ämpäriä, joihin indeksoidaan ottamalla $$ämpäriluku mod 4$$. 
Joskus kuitenkin nämä luvut eivät ole toistensa seuraajia, joten jouduin nostamaan ämpäriluvun seuraavaan 2:n potenssiin eli 8:aan.
Nurkkatapausten hiomisen jälkeen sain hiottua hyvän määrän aikaa pois.


