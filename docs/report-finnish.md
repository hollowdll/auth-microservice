# Template, will be deleted later

Note 1: These examples were written with VS Code that has the "Markdown PDF" extension installed. It also
can be used to preview the markdown output. Right click on the .md file tab and select "Open Preview". Test your Markdown markup also in GitHub.

Note 2: Sometimes you need more whitespace (a space, an empty line between blocks) to get some markdown markup to work

# Level 1 (=chapter) heading, with one hash

Normal text. *Italic text inside asterisk/star* 

## Level 2 (=subchapter) title, with two hashes (and so on...)

Normal text. **Strong text inside double asterisks/stars**

[Text to show on the link](https://www.w3.org/) 

## Bulleted items - unordered list with one star **and a space**

* Potato
* Carrot
* Onion

## Ordered list with just by giving all options the ordinal 1.  Easy to shuffle later!

1. ready
1. steady
1. go!

Inline code examples can be written inside backticks: `(life) => 42`

```
// code blocks with three backticks

if(true) {
    console.log('Hello, world!');
}

```


> Quotes you can add with the bigger than sign, so they look like this.


| code | country |
| :--: | :-----: |
| KEN  | Kenia   |
| FIN  | Finland |
| ABS  | Absurdia |

<details><summary>Summary visible for hideable Details</summary>

Hideable showable text for details.

More text for details.

</details>

<hr />

Look into the *Markdown* source code for how to add images linked from internet, this is the general architecture of the Finnish Finna service: 

![Alternative text for e.g. people with impaired vision](https://www.kiwi.fi/download/attachments/200048777/Ohjelmistoarkkitehtuuri.png?version=1&modificationDate=1607524001948&api=v2)


...or taken from local 'images' folder, like this complicated Wikipedia image of server architectures: 

![Wikipedia's sample picture of server architecture](images/1200px-Wikimedia_Server_Architecture_(simplified).svg.png)

Note: You can invoke the list of the other files in VS Code when you type ctrl+space or type in the dot/"the full stop" character  .

Three ways to write the Horizontal rule:
<hr />

---

***

This is how you can create the ...

**Table of Contents**

[Level 1 (=chapter) heading](#level-1-chapter-heading-with-one-hash) <br />
[Level 2 (=subchapter) title](#level-2-subchapter-title-with-two-hashes-and-so-on) <br />
[Bulleted items - unordered list with one start and a space](#bulleted-items---unordered-list-with-one-star-and-a-space) <br />

Open the markdown file in GitHub.com and use it to copy the link that you need in creation of the table of contents,
just keep the part starting with the #

### Link to second file in same folder
[Link to the second file](markdown_file2.md)


## One "official" source for markdown syntax

[CommonMark.org Markdown in 60 seconds](https://commonmark.org/help/)

[CommonMark.org Markdown tutorial in 10 minutes](https://commonmark.org/help/tutorial/)

Though, these are pieces [Markdown syntax that did not work](markdown_not_supported_syntax.md) in VS Code markdown preview nor in GitHub version of the Markdown.

# Ohjelmistokehityksen teknologioita - Seminaarityö

Autentikaatiomikropalvelu

Ohjelmistoarkkitehtuurit ja patternit

Juuso Hakala

4.12.2023

# 1 Johdanto

Seminaarityön tarkoitukseni oli rakentaa autentikaatiomikropalvelu hyödyntäen mikropalveluohjelmistoarkkitehtuuria. Tavoitteeni työn alussa oli saada palvelu Docker-kontiksi ja hyödyntää Dockeria myös kehityksessä. Lisäksi halusin kokeilla kontin julkaisua CSC:n Rahti-palvelun OpenShiftiin, ja harjoitella palvelun käyttöä ja sen ominaisuuksia.

Mikropalveluarkkitehtuurin ideana on jakaa sovellus pienempiin osiin, jolloin se koostuu erillisistä palveluista, jotka kommunikoivat toistensa kanssa tarvittaessa. Jokaista palvelua voidaan kehittää yksilöllisesti, ja käyttöönotto tapahtuu tyypillisesti hyödyntäen konttiteknologioita. (https://fi.wikipedia.org/wiki/Mikropalvelu)

Autentikointiin tarkoitukseni oli käyttää JSON Web Tokenia. Tiesin aiheesta jo hieman, joten ei tarvinnut aloittaa aiheen opiskelua nollasta. Palveluni tuli käyttämään sisäänkirjautumista, joten halusin tehdä pienen sovelluksen, jolla tätä pystyisi testata. Päädyin tekemään CLI-työkalun eli komentoriviohjelman, missä pystyy kirjautumaan sisään ja käyttämään palveluani. En ollut ikinä ennen tehnyt sisäänkirjautumista komentoriviohjelmassa, joten halusin oppia mahdollisimman paljon.

Työn vaiheet lyhyesti

1. Mikropalveluprojektin alustus ja GitHub repon luonti
2. Alustavat riippuvuudet ja tietokannan konfigurointi
3. Käyttäjien ja roolien luonti
4. JWT autentikointi
5. REST API ja sisäänkirjautuminen
6. gRPC palvelut
7. Docker
8. CLI:n luonti
9. Käyttäjien hakeminen
10. Palvelun julkaisu OpenShiftiin

# 2 Käytetyt tekniikat

# 3 Arkkitehtuurikaavio

# 4 Yhteenveto



## OpenShift

Kirjautuminen Rahdin konttirekisteriin. Käytin PowerShelliä.
```
get-content dockerlogin | docker login -u unused --password-stdin docker-registry.rahti.csc.fi
```
Laitoin salasanan tiedostoon "dockerlogin". Yllä oleva komento lukee salasanan tiedostosta ja siirtää sen "docker login" komennolle --password-stdin:n avulla. Tämä on Dockerin suosittelema tapa ja tietoturvallisempi, koska salasana ei jää lokeihin ja komentohistoriaan. Tämä Stackoverflow-keskustelu oli apuna: https://stackoverflow.com/questions/51489359/docker-using-password-via-the-cli-is-insecure-use-password-stdin

Tällä komennolla loin imagen mikropalvelustani.
```
docker build -t auth-microservice .
```

Tällä komennolla tein imagelle tägin ennen puskemista Rahdin konttirekisteriin
```
docker tag auth-microservice docker-registry.rahti.csc.fi/ohke-teknologiat-seminaari/auth-microservice:rahti
```

Tällä komennolla puskin imagen Rahdin konttirekisteriin omaan image streamiini.
```
docker push docker-registry.rahti.csc.fi/ohke-teknologiat-seminaari/auth-microservice:rahti
```