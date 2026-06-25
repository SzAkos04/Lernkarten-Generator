use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::OnceLock;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

const TRENNBAR_PREFIXES: &[&str] = &[
    "hinterher",
    "vielleicht",
    "herunter",
    "vorüber",
    "zusammen",
    "heraus",
    "herein",
    "hinaus",
    "hinein",
    "gegen",
    "durch",
    "unter",
    "wider",
    "vorbei",
    "weiter",
    "zurück",
    "hinter",
    "auf",
    "aus",
    "bei",
    "dar",
    "ein",
    "er",
    "fehl",
    "fest",
    "form",
    "fort",
    "frei",
    "ge",
    "heim",
    "her",
    "hin",
    "hoch",
    "los",
    "mit",
    "nach",
    "per",
    "um",
    "von",
    "vor",
    "weg",
    "über",
    "ab",
    "an",
    "be",
    "zu",
];

fn clean_filename(name: &str) -> String {
    if name.is_empty() {
        return "unknown".to_string();
    }
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r#"[\\/*?:"<>|]"#).unwrap());
    re.replace_all(name, "").into_owned()
}

fn apply_umlaut(s: &str, rule: Option<&String>) -> String {
    match rule {
        Some(r) => match r.split_once('>') {
            Some((old, new)) => s.replacen(old, new, 1),
            None => s.to_string(),
        },
        None => s.to_string(),
    }
}

fn split_verb_components(inf: &str) -> (&str, &str, String) {
    let parts: Vec<&str> = inf.trim().split_whitespace().collect();
    let sich = if parts.contains(&"sich") { "sich" } else { "" };

    let ignore = ["sich", "auf", "für", "mit", "nach"];
    let clean_parts: Vec<&str> = parts
        .into_iter()
        .filter(|&p| {
            p != "sich"
                && !p.ends_with('+')
                && !p.ends_with('A')
                && !p.ends_with('D')
                && !ignore.contains(&p)
        })
        .collect();

    let mut verb = clean_parts.last().unwrap_or(&"").to_string();
    let mut prefix = "";

    for pfx in TRENNBAR_PREFIXES {
        if verb.starts_with(pfx) && verb.len() > pfx.len() + 2 {
            prefix = pfx;
            verb = verb[pfx.len()..].to_string();
            break;
        }
    }

    (sich, prefix, verb)
}

fn regular_prasens(inf: &str, umlaut: Option<&String>) -> HashMap<String, String> {
    let (sich, prefix, pure_verb) = split_verb_components(inf);
    let s = if pure_verb.len() > 2 {
        &pure_verb[..pure_verb.len() - 2]
    } else {
        &pure_verb
    };
    let s_du = apply_umlaut(s, umlaut);
    let ins_e = if s.ends_with('t') || s.ends_with('d') {
        "e"
    } else {
        ""
    };

    let drop_s = s_du.ends_with('s') || s_du.ends_with('z') || s_du.ends_with('ß');
    let du_suffix = if drop_s { "t" } else { "st" };

    let p_ich = format!("{}e", s);
    let p_du = format!("{}{}{}", s_du, ins_e, du_suffix);
    let p_er = format!("{}{}t", s_du, ins_e);
    let p_wir = pure_verb.clone();
    let p_ihr = format!("{}{}t", s, ins_e);
    let p_sie = pure_verb;

    let pfx_space = if !prefix.is_empty() {
        format!(" {}", prefix)
    } else {
        String::new()
    };
    let reflex_prefix = |role_sich: &str| {
        if !sich.is_empty() {
            format!("{} ", role_sich)
        } else {
            String::new()
        }
    };

    let persons = [
        ("ich", p_ich, reflex_prefix("mich")),
        ("du", p_du, reflex_prefix("dich")),
        ("er", p_er, reflex_prefix("sich")),
        ("wir", p_wir, reflex_prefix("uns")),
        ("ihr", p_ihr, reflex_prefix("euch")),
        ("sie", p_sie, reflex_prefix("sich")),
    ];

    persons
        .into_iter()
        .map(|(role, base_form, reflex)| {
            (
                role.to_string(),
                format!("{}{}{}", reflex, base_form, pfx_space)
                    .trim()
                    .to_string(),
            )
        })
        .collect()
}

fn regular_prateritum(inf: &str) -> String {
    let (_, prefix, pure_verb) = split_verb_components(inf);
    let s = if pure_verb.len() > 2 {
        &pure_verb[..pure_verb.len() - 2]
    } else {
        &pure_verb
    };
    let ins_e = if s.ends_with('t') || s.ends_with('d') {
        "e"
    } else {
        ""
    };

    let form = format!("{}{}te", s, ins_e);
    if !prefix.is_empty() {
        format!("{} {}", form, prefix)
    } else {
        form
    }
}

fn regular_partizip(inf: &str) -> String {
    let (_, prefix, pure_verb) = split_verb_components(inf);
    let s = if pure_verb.len() > 2 {
        &pure_verb[..pure_verb.len() - 2]
    } else {
        &pure_verb
    };
    let ins_e = if s.ends_with('t') || s.ends_with('d') {
        "e"
    } else {
        ""
    };

    if !prefix.is_empty() {
        format!("{}ge{}{}t", prefix, s, ins_e)
    } else {
        format!("ge{}{}t", s, ins_e)
    }
}

fn parse_kv_item(item: &[Value]) -> HashMap<String, String> {
    let mut data = HashMap::new();
    for p in item.iter().skip(1).filter_map(|val| val.as_str()) {
        if let Some((k, v)) = p.split_once('=') {
            data.insert(k.to_string(), v.to_string());
        }
    }
    data
}

fn format_siehe_auch(siehe_str: Option<&String>) -> String {
    match siehe_str {
        Some(s) if !s.trim().is_empty() && s != "-" => s
            .split(',')
            .map(|i| i.trim())
            .filter(|i| !i.is_empty())
            .map(|item| format!("- [[{}]]", item))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => "-".to_string(),
    }
}

fn format_info_box(info_str: Option<&String>) -> String {
    match info_str {
        Some(s) if !s.trim().is_empty() && s != "-" && s != "None" => {
            format!("\n> {}", s.trim())
        }
        _ => String::new(),
    }
}

fn make_verb(v: &[Value]) -> String {
    let d = parse_kv_item(v);
    let inf = d
        .get("inf")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let hu = d.get("hu").map(String::as_str).unwrap_or("");
    let niveau = d.get("niv").map(String::as_str).unwrap_or("A1");

    let (_, prefix, _) = split_verb_components(&inf);
    let default_sep = if !prefix.is_empty() {
        "trennbar"
    } else {
        "untrennbar"
    };
    let verb_sep = d
        .get("sep")
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| default_sep.to_string());

    let has_strong_hints = d.contains_key("uml") || d.contains_key("du") || d.contains_key("er");
    let default_typ = if has_strong_hints { "stark" } else { "schwach" };
    let verb_typ = d
        .get("typ")
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| default_typ.to_string());

    let pr = d
        .get("pr")
        .cloned()
        .unwrap_or_else(|| regular_prateritum(&inf));
    let pp = d
        .get("pp")
        .cloned()
        .unwrap_or_else(|| regular_partizip(&inf));

    let mut forms = regular_prasens(&inf, d.get("uml"));

    // Explicitly overrule any grammatical person forms if passed via the JSON fields
    for person in &["ich", "du", "er", "wir", "ihr", "sie"] {
        if let Some(custom_form) = d.get(*person) {
            forms.insert(person.to_string(), custom_form.clone());
        }
    }

    let aux_raw = d
        .get("aux")
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "haben".to_string());
    let aux_verb = if aux_raw == "sein" || aux_raw == "ist" {
        "ist"
    } else {
        "hat"
    };

    let siehe_auch_block = format_siehe_auch(d.get("siehe"));
    let info_block = format_info_box(d.get("infobox"));

    format!(
        r#"---
Wort: {inf}
Wortart: Verb
Verbtyp:
  - {verb_typ}
  - {verb_sep}
Niveau: {niveau}
Bedeutung: {hu}
Präteritum: {pr}
Partizip_II: {pp}
Hilfsverb:
  - {aux_verb}
Quelle:
aliases:
---

# `=this.Wort`

## Ungarisch Bedeutung: "`=this.Bedeutung`"

| Infinitiv        | Präteritum             | Partizip II                               |
| ---------------- | ---------------------- | ----------------------------------------- |
| **`=this.Wort`** | **`=this.Präteritum`** | `=this.Hilfsverb` **`=this.Partizip_II`** |

| Personalpronomen | Verb                                         |
| ---------------- | -------------------------------------------- |
| ich              | **{ich}** |
| du               | **{du}** |
| er / sie / es    | **{er}** |
| wir              | **{wir}** |
| ihr              | **{ihr}** |
| sie / Sie        | **{sie}** |

> [!INFO]
> **Verbtyp:** `=this.Verbtyp`{info_block}

---

## Siehe auch

{siehe_auch_block}

---

`=this.Bedeutung` :: `=this.Wort`, `=this.Präteritum`, \
`=this.Hilfsverb` `=this.Partizip_II`
#Lernkarten"#,
        inf = inf,
        verb_typ = verb_typ,
        verb_sep = verb_sep,
        niveau = niveau,
        hu = hu,
        pr = pr,
        pp = pp,
        aux_verb = aux_verb,
        ich = forms.get("ich").unwrap_or(&String::new()),
        du = forms.get("du").unwrap_or(&String::new()),
        er = forms.get("er").unwrap_or(&String::new()),
        wir = forms.get("wir").unwrap_or(&String::new()),
        ihr = forms.get("ihr").unwrap_or(&String::new()),
        sie = forms.get("sie").unwrap_or(&String::new()),
        info_block = info_block,
        siehe_auch_block = siehe_auch_block
    )
}

fn make_noun(n: &[Value]) -> String {
    let mut pads = vec![None; 8];
    for (i, val) in n.iter().enumerate().take(8) {
        pads[i] = val.as_str().map(|s| s.to_string());
    }

    let wort = pads[1].clone().unwrap_or_default();
    let niveau = pads[2].as_ref().map(String::as_str).unwrap_or("A1");
    let genus = pads[3].clone().unwrap_or_default();
    let plural = pads[4].clone().unwrap_or_default();
    let bedeutung = pads[5].clone().unwrap_or_default();
    let info = pads[6].as_ref();
    let siehe = pads[7].as_ref();

    let plural_str = if !plural.is_empty() && plural != "-" {
        plural
    } else {
        String::new()
    };

    let clean_genus = genus.trim().to_lowercase();
    let is_plural_only = clean_genus == "die (pl.)";

    let title_display = if is_plural_only {
        format!("{} {}", genus, wort)
    } else {
        format!("{} {}, {}", genus, wort, plural_str)
    };

    let siehe_auch_block = format_siehe_auch(siehe);
    let info_block = format_info_box(info);

    let m_der = r#"<mark style="background: #ADCCFFA6;">der</mark>"#;
    let m_die = r#"<mark style="background: #FF5582A6;">die</mark>"#;
    let m_das = r#"<mark style="background: #BBFABBA6;">das</mark>"#;
    let m_pl = r#"<mark style="background: #FF5582A6;">die (Pl.)</mark>"#;

    let (artikel_cell, wort_cell, plural_cell) = if is_plural_only {
        (m_pl, wort.as_str(), "")
    } else {
        let art = match clean_genus.as_str() {
            "der" => m_der,
            "die" => m_die,
            "das" => m_das,
            _ => m_der,
        };
        (art, "`=this.Wort`", "`=this.Plural`")
    };

    format!(
        r#"---
Wort: {wort}
Wortart: Substantiv
Niveau: {niveau}
Genus: {genus}
Plural: {plural_str}
Bedeutung: {bedeutung}
Quelle:
aliases:
---

# {title_display}

## Ungarisch Bedeutung: "`=this.Bedeutung`"

| Artikel | Substantiv | Plural |
| ------- | ---------- | ------ |
| {artikel_cell} | {wort_cell} | {plural_cell} |

> [!INFO]{info_block}

---

## Siehe auch

{siehe_auch_block}

---

`=this.Bedeutung` :: `=this.Genus` `=this.Wort`, `=this.Plural`
#Lernkarten"#,
        wort = wort,
        niveau = niveau,
        genus = genus,
        plural_str = plural_str,
        bedeutung = bedeutung,
        title_display = title_display,
        artikel_cell = artikel_cell,
        wort_cell = wort_cell,
        plural_cell = plural_cell,
        info_block = info_block,
        siehe_auch_block = siehe_auch_block
    )
}

fn make_adj(a: &[Value]) -> String {
    let mut pads = vec![None; 8];
    for (i, val) in a.iter().enumerate().take(8) {
        pads[i] = val.as_str().map(|s| s.to_string());
    }

    let wort = pads[1].clone().unwrap_or_default();
    let niveau = pads[2].as_ref().map(String::as_str).unwrap_or("A1");
    let bedeutung = pads[3].clone().unwrap_or_default();
    let komp = pads[4].clone().unwrap_or_default();
    let sup = pads[5].clone().unwrap_or_default();
    let info = pads[6].as_ref();
    let siehe = pads[7].as_ref();

    let siehe_auch_block = format_siehe_auch(siehe);
    let info_block = format_info_box(info);

    format!(
        r#"---
Wort: {wort}
Wortart: Adjektiv
Niveau: {niveau}
Bedeutung: {bedeutung}
Komparativ: {komp}
Superlativ: {sup}
Quelle:
aliases:
---

# `=this.Wort`

## Ungarisch Bedeutung: "`=this.Bedeutung`"

| Positiv          | Komparativ             | Superlativ                |
| ---------------- | ---------------------- | ------------------------- |
| **`=this.Wort`** | **`=this.Komparativ`** | am **`=this.Superlativ`** |

> [!INFO]{info_block}

---

## Siehe auch

{siehe_auch_block}

---

`=this.Bedeutung` :: `=this.Wort`, `=this.Komparativ`, am `=this.Superlativ`
#Lernkarten"#,
        wort = wort,
        niveau = niveau,
        bedeutung = bedeutung,
        komp = komp,
        sup = sup,
        info_block = info_block,
        siehe_auch_block = siehe_auch_block
    )
}

fn generate(
    thema: &str,
    verbs: &[Vec<Value>],
    nouns: &[Vec<Value>],
    adjs: &[Vec<Value>],
) -> io::Result<()> {
    let base = clean_filename(&thema.replace(' ', "_").replace('&', "und"));

    fs::create_dir_all(format!("{}/Verben", base))?;
    fs::create_dir_all(format!("{}/Substantive", base))?;
    fs::create_dir_all(format!("{}/Adjektive", base))?;

    for v in verbs {
        let d = parse_kv_item(v);
        let inf = d
            .get("inf")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string());
        let fname = format!("{}/Verben/{}.md", base, clean_filename(&inf));
        fs::write(fname, make_verb(v))?;
    }

    for n in nouns {
        let name = n[1].as_str().unwrap_or("unknown");
        let fname = format!("{}/Substantive/{}.md", base, clean_filename(name));
        fs::write(fname, make_noun(n))?;
    }

    for a in adjs {
        let name = a[1].as_str().unwrap_or("unknown");
        let fname = format!("{}/Adjektive/{}.md", base, clean_filename(name));
        fs::write(fname, make_adj(a))?;
    }

    let zip_name = format!("{}.zip", base);
    let file = File::create(&zip_name)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let folders = ["Verben", "Substantive", "Adjektive"];
    for folder in &folders {
        let dir_path = format!("{}/{}", base, folder);
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    let internal_path = format!("{}/{}/{}", base, folder, file_name);
                    zip.start_file(internal_path, options)?;
                    let buffer = fs::read(path)?;
                    zip.write_all(&buffer)?;
                }
            }
        }
    }
    zip.finish()?;

    println!(
        "{}: {} Verben, {} Substantive, {} Adjektive",
        thema,
        verbs.len(),
        nouns.len(),
        adjs.len()
    );
    println!("Zip: {}", zip_name);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let raw_json: Value = if args.len() > 1 && args[1] != "-" {
        let file = File::open(&args[1]).expect("Failed to open input JSON file.");
        serde_json::from_reader(file).expect("Invalid JSON template.")
    } else {
        serde_json::from_reader(io::stdin()).expect("Failed to read JSON from Stdin.")
    };

    let thema = if args.len() > 2 { &args[2] } else { "Szavak" };

    let word_list = if let Some(arr) = raw_json.as_array() {
        arr
    } else if let Some(words) = raw_json.get("words").and_then(|w| w.as_array()) {
        words
    } else {
        eprintln!("Error: Unexpected JSON root formatting.");
        std::process::exit(1);
    };

    let mut verbs = Vec::new();
    let mut nouns = Vec::new();
    let mut adjs = Vec::new();

    for item in word_list {
        if let Some(entry_arr) = item.as_array() {
            if entry_arr.is_empty() {
                continue;
            }
            if let Some(t) = entry_arr[0].as_str() {
                match t.to_lowercase().as_str() {
                    "v" => verbs.push(entry_arr.clone()),
                    "n" => nouns.push(entry_arr.clone()),
                    "a" => adjs.push(entry_arr.clone()),
                    _ => {}
                }
            }
        }
    }

    if let Err(e) = generate(thema, &verbs, &nouns, &adjs) {
        eprintln!("Execution failed with error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    const MOCK_LLM_INPUT: &str = r#"[["v","inf=abschreiben","hu=lemásolni; puskázni","niv=B1","sep=trennbar","typ=stark","pr=schrieb ab","pp=abgeschrieben","siehe=schreiben,täuschen"],["n","Abfall","A2","der","-̈e","hulladék, szemét","-","Müll,Recycling"],["a","abhängig","B2","függő, függőséges","abhängiger","abhängigsten","-","selbstständig,unabhängig"]]"#;

    #[test]
    fn test_input_parsable_by_serde() {
        let res: Result<Value, _> = serde_json::from_str(MOCK_LLM_INPUT);
        assert!(res.is_ok());
    }

    #[test]
    fn test_strict_positional_array_lengths() {
        let raw_json: Value = serde_json::from_str(MOCK_LLM_INPUT).unwrap();
        let word_list = raw_json.as_array().unwrap();

        for item in word_list {
            let entry = item.as_array().unwrap();
            let type_tag = entry[0].as_str().unwrap();
            match type_tag {
                "n" | "a" => assert_eq!(entry.len(), 8),
                "v" => assert!(entry.len() >= 3),
                _ => panic!(),
            }
        }
    }
}
