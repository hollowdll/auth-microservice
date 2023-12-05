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

## 2.1 Mikropalvelun tekniikat

Rakensin mikropalveluni C#-ohjelmointikielellä ja .NET frameworkin ASP.NET Corella. ASP.NET Core on avoimen lähdekoodin framework, millä voi rakentaa moderneja webbisovelluksia. Se soveltuu etenkin pilviympäristöissä suoritettaviin ohjelmistoihin. (https://github.com/dotnet/aspnetcore).

Käytin .NET frameworkin versiota 6. Lisäksi hyödynsin .NET frameworkin CLI työkaluja, jotka helpottivat omaa työstämistäni.

Projektin loin valmiilla pohjalla, jota lähdin muokkaamaan. Sen sai komennolla

```bash
dotnet new webapi
```

### 2.1.1 Tietokanta

Tietokantana käytin PostgreSQL:ää. Mikropalvelussa käytin .NET frameworkin Entity Frameworkia relaatiotietokannan hallinnoimiseen (https://learn.microsoft.com/en-us/aspnet/entity-framework). Se on ORM eli object-relational mapper, millä voi luoda tietokantatauluja suoraan C#-koodista. Tämä työkalu oli minulle tuttu jo entuudestaan, ja itselleni se antoi hyvän kehittäjäkokemuksen.

Tietokannan scheman sai generoitua komennolla

```bash
dotnet ef migrations add UserDatabase
```

Se generoi C#-kooditiedoston, jolla pystyy luomaan tietokannan. Mikropalveluni konfiguroin, että se luo tietokannan käynnistäessä, jos sitä ei ole.

### 2.1.2 Käyttäjät

Käyttäjien luontiin ja hallinnoimiseen käytin .NET frameworkin Identity järjestelmää (https://learn.microsoft.com/en-us/aspnet/core/security/authentication/identity?view=aspnetcore-8.0&tabs=visual-studio). Sillä saa helposti tuotantovalmiin pohjan käyttäjätietojen hallinoimiseen, mitä voi tarvittaessa laajentaa ja muokata. Tietokantatauluja ei tällä tarvitse luoda käyttäjille, koska ne saa generoitua valmiiksi. Muutin kuitenkin valmiit tietokantataulujen nimet.

### 2.1.3 Autentikaatio

Autentikaatioon käytin JSON Web Tokenia. Kun käyttäjä pyytää jotain resurssia palvelusta, pyynnön mukana pitää tulla JWT access token. Palvelu tarkistaa, että token on kelvollinen ja voimassa. Token pitää sisällään käyttäjän tietoja kuten käyttäjänimen ja roolit. (https://en.wikipedia.org/wiki/JSON_Web_Token).

Sisäänkirjautumisen yhteydessä palvelu palauttaa aina uuden JWT access tokenin, jolla käyttäjä voi autentikoitua palveluun. Tokenit ovat voimassa vain lyhyen ajan tietoturvan vahvistamiseksi. 

### 2.1.4 REST API

Palvelussa on REST API, missä on kaksi endpointia. Toinen on sisäänkirjautumiseen ja toinen käyttäjien hakemiseen. Käyttäjien hakeminen vaatii autentikaation ja admin-roolin. Katso [REST API endpointit](../README.md#rest-api)

REST API pyörii palvelun portissa 5105, missä on käytössä HTTP/1.1.

### 2.1.5 gRPC

REST API:n lisäksi palvelussani on myös toisenlainen API. Käytin tähän gRPC Remote Procedure Call frameworkia (https://en.wikipedia.org/wiki/GRPC). gRPC käyttää HTTP protokollan versiota HTTP/2. Tiedonsiirrossa on käytössä Protocol Buffers -dataformaattia (https://en.wikipedia.org/wiki/Protocol_Buffers).

gRPC on REST API:a nopeampi tiedonsiirrossa yleensä suuremmilla tietomäärillä. Minun projektissa nopeuksilla ei ollut melkein mitään eroa, koska tietomäärät olivat hyvin pienet. gRPC on kuitenkin tänä päivänä suosittu ja käytetty etenkin mikropalveluiden maailmassa, joten halusin kokeilla sitä.

Ideana on määritellä ensin .proto tiedostoon tietotyypit ja RPC:t. Tämän jälkeen .proto tiedostosta voi generoida jollekkin ohjelmointikielelle koodia, mitä voi kutsua. Käytin seuraavaa dokumentaatiota kun aloitin: https://learn.microsoft.com/en-us/aspnet/core/grpc/basics?view=aspnetcore-8.0.

gRPC palvelut pyörivät portissa 5106, ja pitävät sisällään myös sisäänkirjautumisen ja käyttäjien hakemisen.

### 2.1.6 Docker

Dockeria hyödynsin sekä kehityksessä, että julkaisussa. Tein Dockerfilen, millä sain tehtyä mikropalvelustani Docker imagen. Tästä imagesta sai ajettua kontteja. Palveluni käytti kuitenkin tietokantaa, joten hyödynsin Docker Composea paikallisesti, jolla sain ajettua palveluni ja tietokannan yhdessä. Määrittelin tarvittavat asiat docker-compose.yml tiedostoon, missä määrittelin tietokannan omaan konttiin. Näin sain tehokkaasti testattua kokonaisuutta omalla koneellani.

Imagen luonti
```bash
docker build -t auth-microservice .
```

Palvelun ja tietokannan ajo Docker Composella
```bash
docker compose up -d
```

# 3 Arkkitehtuurikaavio

Alla oleva kaavio havainnolistaa mikropalvelun rakennetta ja JWT autentikaation kulkua. Palvelussani on ainoastaan JWT access tokenit, eikä refresh tokeneja ole.

Sisäänkirjautuminen on sekä gRPC palveluissa, että REST API:ssa.

![Arkkitehtuurikaavio](schema.JPG)

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