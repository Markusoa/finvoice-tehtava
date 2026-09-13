# Finvoice PDF -generaattori

Ohjelma lukee Finvoice 2.01 -muotoisen XML-laskun ja muodostaa sen perusteella
PDF muodon tästä kyseisestä laskusta.

## Ohjelman toiminta lyhyesti

Ohjelman suoritus etenee seuraavasti:

1. XML-tiedosto luetaan merkkijonoksi.
2. XML muunnetaan Rustin tietorakenteiksi.
3. Laskun tiedoista rakennetaan PDF:n sisältö ja ulkoasu.
4. PDF renderöidään ja tallennetaan `lasku.pdf`-tiedostoon.

## `src/main.rs`

Main fileä käytetään pdf tiedoston generoimiseen ja täällä
määrittellään tietenkin myös kaikki se mitä pdf tiedostossa sitten esitetään.

### XML-tiedoston lukeminen

Ohjelma lukee projektin juuressa olevan tiedoston:

```rust
let xml = std::fs::read_to_string("finvoice_testi_2_01.xml")?;
```

Tämän jälkeen `quick_xml` muuntaa XML-sisällön `Finvoice`-rakenteeksi:

```rust
let finvoice: Finvoice = quick_xml::de::from_str(&xml)?;
```

Muuttuja `finvoice` sisältää kaikki tarpeelliset tiedot esim. myyjän, ostajan, maksutietojen,
laskurivien ja loppusummien tiedot.

### Laskurivien muodostaminen

`row1` ja `row2` sisältävät laskurivit, niissä ovat tiedot:

- tuotteen tai palvelun nimi
- määrä ja yksikkö
- yksikköhinta
- ALV-prosentti
- ALV:n määrä
- rivin loppusumma

Muotoilussa käytetään tekstin leveyksiä, jotta sarakkeet pysyvät suunnilleen
samassa kohdassa PDF-tiedostossa.

Tässä olisi ehkä myös ollut toimivaa, jos olisi tehnyt jonkulaisen grid systeemin, 
jotta rivien tiedot pysyvät vertikaalisesti kohdillaan

### PDF:n layout ja `view`-rakenteet, flexpdf kirjasto

PDF rakennetaan `flexpdf`-kirjaston avulla. `view()` toimii säiliönä, jonka
sisälle lisätään tekstiä tai muita näkymiä `children([...])`-kutsulla.

Ohjelman pääasiallinen rakenne on:

```text
Pääview
|-- Laskun otsikko
|-- Yläosa
|   |-- Myyjä ja ostaja vasemmalla
|   `-- Maksutiedot oikealla
|-- Laskurivien otsikot ja rivit
`-- Loppusummat
```

Yläosan `FlexDirection::Row` asettaa myyjä- ja ostajatiedot sekä maksutiedot
vaakasuunnassa vierekkäin. Muissa näkymissä sisältö asetellaan oletuksena
ylhäältä alaspäin.

Lyhyen tutustumisen jälkeen flexpdf kirjasto näytti kivalta ja helpolta joten päädyin siihen PDF-tiedoston generoimista varten.

### Tyylit ja välistykset

`Style` määrittää näkymän ulkoasun ja sijainnin. Ohjelmassa käytettyjä
ominaisuuksia ovat esimerkiksi:

- `padding`: sisämarginaali näkymän ympärillä
- `padding_left`: siirtää sisältöä oikealle
- `padding_top` ja `padding_bottom`: lisäävät tilaa ylä- tai alapuolelle
- `gap`: määrittää välin näkymän lapsielementtien välillä
- `font_size`: muuttaa tekstin kokoa
- `flex_grow`: antaa näkymälle lisää käytettävissä olevaa leveyttä

Näillä arvoilla muokkailin pdf tiedoston ulkonäköä.

### PDF:n renderöinti ja tallennus

Kun dokumentin rakenne on valmis, se renderöidään PDF-tiedostoksi ja
tallennetaan:

```rust
let pdf = render_document(&doc)?;
std::fs::write("lasku.pdf", pdf)?;
```

Ohjelma palauttaa virheen `Result`-tyypin avulla, jos XML-tiedoston lukeminen,
muuntaminen, PDF:n muodostaminen tai tiedoston tallentaminen epäonnistuu.

## `src/convert.rs`

`convert.rs` sisältää tietorakenteet, joihin XML muunnetaan. Rakenteet on
merkitty `serde::Deserialize`-johdannalla, joten `quick_xml` osaa täyttää ne
XML-elementtien perusteella.

Esimerkiksi `Finvoice` sisältää seuraavat kokonaisuudet:

- `seller`: myyjän tiedot
- `buyer`: ostajan tiedot
- `sellerinfo`: myyjän pankkitiedot
- `invoice`: laskun numerot ja summat
- `rows`: laskurivit
- `payment`: maksutiedot

Kun XML-tiedoston tiedot on tallennettu tämän convert filen avulla rustin tietorakenteisiin niin sitä voidaan helposti käyttää main filessä näyttämään nämä tiedot PDF-tiedostossa


# Virheiden käsittely

Ohjelma käyttää Rustin Result tyyppiä ja ? operaattoria virheiden välittämiseen. Jos XML-tiedostoa ei voida lukea niin ohjelma lopetetaan ja virhe ilmoitetaan käyttäjälle. Ohjelmassa on kuitenkin riskejä, esim. jos XML-tiedostossa on vähemmän kuin kaksi laskuriviä niin ohjelma kaatuu, koska ohjelma on hard koodattu siihen tilanteeseen että laskurivejä on 2.

Tämä laskurivien hard koodaus on kylläkin vain sen takia että sain PDF tiedostossa laskurivien tiedot vertikaalisesti kohdilleen ja tämä oli helpoin tehdä kyseisellä tavalla mutta laskurivit voisi myös tehdä ilman tätä hard koodausta.

## Ohjelman suorittaminen

Ohjelma suoritetaan cargo run komennolla.

```powershell
cargo run
```

Tuloksena syntyy tiedosto `lasku.pdf`.


