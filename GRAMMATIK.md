# Input Style Guide - TELC B2 Obsidian Nyelvtani Jegyzetek (LLM-eknek)

Egy precíz, magyar anyanyelvű B2-szintű némettanuló számára készítesz Obsidian-kompatibilis nyelvtani jegyzeteket. Minden kimenet **kizárólag nyers Markdown** legyen, a lenti sablon szerint kitöltve. Semmi bevezető szöveg, semmi ```markdown kódblokk, semmi lezárás — csak a kész jegyzet.

---

## 1. Globális szabályok

- **Egy jegyzet = egy nyelvtani jelenség** (pl. "Konjunktiv II", "Relativsätze", "Passiv Präsens"). Ha a felhasználó egy nagyobb témát ad meg (pl. "Zeiten"), bontsd fel aleseményekre, és minden alesethez külön jegyzetet készíts.
- **Nyelv**: a magyarázatok magyarul íródnak, a német példamondatok és szakkifejezések (pl. *Nebensatz*, *Kasus*) németül maradnak dőltként vagy kódformázva.
- **Tömörség**: ne magyarázz túl semmit — egy B2-es tanuló már ismeri az alapokat, csak a lényeges szabályra, a tipikus hibákra és a kivételekre fókuszálj.
- **Duplikáció elkerülése**: ha kapsz egy meglévő jegyzet-listát, csak az onnan hiányzó témákat generáld.
- **Kimenet**: kizárólag a kitöltött Markdown sablon, semmi más.

---

## 2. A sablon (kötelező struktúra)

```markdown
---
Titel: {Nyelvtani jelenség neve, pl. "Konjunktiv II"}
Thema:
  - {kategória, pl. Verb / Satzbau / Kasus / Zeiten / Pronomen / Präposition}
Niveau: {A1/A2/B1/B2/C1/C2}
Quelle:
aliases:
tags:
  - Grammatik
---

# `=this.Titel`

## Mikor használjuk?
{1-3 mondatos magyar összefoglaló arról, milyen kommunikációs helyzetben kell ezt a szerkezetet használni}

## Képzés

| Szerkezet / Alak | Példa |
| ----------------- | ------- |
| {szabály 1. része} | `{német példa}` |
| {szabály 2. része} | `{német példa}` |

## Példák

> [!EXAMPLE]
> **DE:** {német mondat}
> **HU:** {magyar fordítás}

> [!EXAMPLE]
> **DE:** {német mondat}
> **HU:** {magyar fordítás}

> [!WARNING] Kivétel / Figyelem
> {csak akkor töltsd ki, ha van releváns kivétel, ellenkező esetben hagyd ki ezt a blokkot teljesen}

## Gyakori hibák magyar anyanyelvűeknek
- {tipikus interferencia-hiba, pl. szórend, esetleges elöljárós vonzat különbség}
- {második tipikus hiba, ha van}

---

## Siehe auch

- [[{kapcsolódó nyelvtani téma}]]
- [[{kapcsolódó szó vagy másik téma}]]

---

`=this.Titel` :: {egy rövid, kártyaszerű szabály-összefoglalás, ami spaced repetition plugin számára is működik}

#Grammatik
```

---

## 3. Mezőnkénti szabályok

- **Titel**: rövid, pontos elnevezés (németül vagy a bevett magyar-német hibrid néven, pl. "Konjunktiv II", nem "Feltételes mód németül").
- **Thema**: 1 vagy több kategória-cimke, hogy a jegyzetek szűrhetők legyenek Obsidian-ban (Dataview-kompatibilis).
- **Niveau**: fókusz B1-B2, de ha egy alapozó szerkezet (pl. Perfekt) szükséges előfeltétel, azt is generáld A1-A2 szinttel jelölve.
- **Képzés táblázat**: mindig konkrét, kitöltött példákkal — sose hagyj sablon-placeholdert a végleges kimenetben.
- **Példák**: minimum 2, maximum 4 pár (DE+HU), lehetőleg különböző kontextusból (nem csak egy sablonmondat variánsai).
- **Kivétel blokk**: csak akkor jelenjen meg, ha van tényleges, tanulási szempontból releváns kivétel - üres vagy triviális blokkot ne generálj.
- **Gyakori hibák**: kifejezetten magyar anyanyelvűek tipikus hibáira fókuszálj (pl. szórend, elváló igekötők elfelejtése, esetleges Kasus-tévesztés magyar nyelvtani logika miatt).
- **Siehe auch**: kötelezően legalább 2 kapcsolódó `[[wikilink]]`, ami vagy másik nyelvtani jegyzet, vagy egy releváns szókártya (pl. egy igekártya, amin bemutatható a szerkezet).
- **Kártya-sor a végén**: legyen önmagában is érthető, mint egy Anki/SR-kártya előlap-hátlap párja (`kérdés :: válasz` logika).

---

## 4. Mintapélda (Strict Token-Dense)

```markdown
---
Titel: Konjunktiv II
Thema:
  - Verb
  - Modalität
Niveau: B2
Quelle:
aliases:
tags:
  - Grammatik
---

# `=this.Titel`

## Mikor használjuk?
Irreális feltételek, udvarias kérések és kívánságok kifejezésére használjuk, amikor valami nem valós vagy nem biztos.

## Képzés

| Szerkezet / Alak | Példa |
| ----------------- | ------- |
| würde + Infinitiv (leggyakoribb) | `Ich würde gehen.` |
| Präteritum-tő + Umlaut (erős igéknél) | `Ich ginge.` / `Ich hätte.` |
| Modalige Konjunktiv II | `Ich könnte, ich müsste, ich sollte.` |

## Példák

> [!EXAMPLE]
> **DE:** Wenn ich Zeit hätte, würde ich dich besuchen.
> **HU:** Ha lenne időm, meglátogatnálak.

> [!EXAMPLE]
> **DE:** Könnten Sie mir bitte helfen?
> **HU:** Tudna nekem kérem segíteni?

> [!WARNING] Kivétel / Figyelem
> Néhány gyakori igénél (sein, haben, a módbeli segédigék, werden) az egyszerű Konjunktiv II alakot használjuk *würde* helyett is a beszélt nyelvben, mert természetesebben hangzik.

## Gyakori hibák magyar anyanyelvűeknek
- Magyarul a feltételes mód egyetlen alakkal működik ("mennék"), ezért a tanulók hajlamosak mindig *würde*-t használni sein/haben/módbeli segédigéknél is, ahol a rövidebb alak a természetes.
- A *wenn*-mellékmondat után a szórend gyakran hibás, mert a magyar mondatszerkezet nem kényszeríti ki a ige a mondat végére kerülését.

---

## Siehe auch

- [[Konjunktiv I]]
- [[Passiv Präsens]]

---

`=this.Titel` :: irreális feltétel / udvarias kérés kifejezésére: würde + Infinitiv, vagy erős igéknél Präteritum-tő + Umlaut

#Grammatik
```

---

## 5. Alkalmazandó prompt

> "Generálj egy Obsidian nyelvtani jegyzetet a Style Guide szabályai alapján a(z) [TÉMA] jelenségről, [SZINT] szinten. Meglévő jegyzetek, amiket hagyj ki: [LISTA]. Ne használj markdown kódblokkokat a válasz köré, se bevezető/lezáró szöveget — csak a kitöltött sablont add vissza nyersen."

### Több téma egyszerre
Ha egyszerre több nyelvtani jegyzetet kérsz, az LLM egymás után, `---` elválasztóval adja vissza az egyes jegyzeteket, hogy egyszerűen szét lehessen vágni külön `.md` fájlokra.
