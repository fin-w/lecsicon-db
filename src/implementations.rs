use crate::definitions::*;

use diesel::prelude::*;
use hunspell_rs::Hunspell;
use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::CustomUserError;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;

impl Default for LecsiconEntry {
    fn default() -> LecsiconEntry {
        LecsiconEntry {
            id: -99,
            word: "ERROR".to_string(),
            lemma: None,
            tag: None,
            adposition_type: None,
            definite: None,
            degree: None,
            gender: None,
            mood: None,
            mutation: None,
            name_type: None,
            number: None,
            number_form: None,
            number_type: None,
            person: None,
            polarity: None,
            polite: None,
            position: None,
            possessive: None,
            pronoun_type: None,
            relative: None,
            style: None,
            tense: None,
            verb_form: None,
        }
    }
}

pub fn load_text_lecsicon(filename: &Path) -> Option<Vec<LecsiconEntry>> {
    let lecsicon_file = File::open(filename);
    match lecsicon_file {
        Ok(file) => {
            let mut csv_reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b'\t')
                .flexible(true)
                .from_reader(file);
            let mut all_entries: Vec<LecsiconEntry> = vec![];

            let mut previous_entries: VecDeque<(String, i32)> = VecDeque::new();
            for result in csv_reader.records() {
                let mut new_lecsicon_entry = LecsiconEntry {
                    ..Default::default()
                };
                match result {
                    Ok(record) => {
                        new_lecsicon_entry.id = 101;
                        new_lecsicon_entry.word = record.get(0)?.to_string();
                        new_lecsicon_entry.lemma = Some(record.get(1)?.to_string());
                        new_lecsicon_entry.tag = match record.get(2)? {
                            "ADJ" => Some(Tag::Adjective),
                            "ADP" => Some(Tag::Adposition),
                            "ADV" => Some(Tag::Adverb),
                            "CONJ" => Some(Tag::Conjugation),
                            "DET" => Some(Tag::Determiner),
                            "INTJ" => Some(Tag::Interjection),
                            "NOUN" => Some(Tag::Noun),
                            "NUM" => Some(Tag::Number),
                            "PART" => Some(Tag::Particle),
                            "PRON" => Some(Tag::Pronoun),
                            "PROPN" => Some(Tag::ProperNoun),
                            "VERB" => Some(Tag::Verb),
                            _ => {
                                println!("{}", record.get(2)?);
                                panic!("Tag not recognised");
                            }
                        };

                        for (word, id) in previous_entries.iter() {
                            if *word == new_lecsicon_entry.word && *id >= new_lecsicon_entry.id {
                                new_lecsicon_entry.id = id + 1;
                            }
                        }
                        previous_entries
                            .push_front((new_lecsicon_entry.word.clone(), new_lecsicon_entry.id));
                        previous_entries.truncate(20);

                        if record.len() > 3 {
                            let raw_details = record.get(3)?;
                            let details = raw_details.split('|');
                            for tag_pair in details {
                                match tag_pair {
                                    "AdpType=Prep" => {
                                        new_lecsicon_entry.adposition_type =
                                            Some(AdpositionType::Preposition)
                                    }
                                    "Definite=Def" => {
                                        new_lecsicon_entry.definite = Some(Definite::Definite)
                                    }
                                    "Degree=Cmp" => {
                                        new_lecsicon_entry.degree = Some(Degree::Comparative)
                                    }
                                    "Degree=Equ" => {
                                        new_lecsicon_entry.degree = Some(Degree::Equative)
                                    }
                                    "Degree=Pos" => {
                                        new_lecsicon_entry.degree = Some(Degree::Positive)
                                    }
                                    "Degree=Sup" => {
                                        new_lecsicon_entry.degree = Some(Degree::Superlative)
                                    }
                                    "Gender=Fem" => {
                                        new_lecsicon_entry.gender = Some(Gender::Feminine)
                                    }
                                    "Gender=Fem,Masc" => {
                                        new_lecsicon_entry.gender = Some(Gender::FeminineMasculine)
                                    }
                                    "Gender=Masc" => {
                                        new_lecsicon_entry.gender = Some(Gender::Masculine)
                                    }
                                    "Mood=Imp" => new_lecsicon_entry.mood = Some(Mood::Imperative),
                                    "Mood=Ind" => new_lecsicon_entry.mood = Some(Mood::Indicative),
                                    "Mood=Sub" => new_lecsicon_entry.mood = Some(Mood::Subjunctive),
                                    "Mutation=AM" => {
                                        new_lecsicon_entry.mutation = Some(Mutation::Aspirate);
                                    }
                                    "Mutation=HM" => {
                                        new_lecsicon_entry.mutation = Some(Mutation::HProthesis);
                                    }
                                    "Mutation=NM" => {
                                        new_lecsicon_entry.mutation = Some(Mutation::Nasal);
                                    }
                                    "Mutation=SM" => {
                                        new_lecsicon_entry.mutation = Some(Mutation::Soft);
                                    }
                                    "NameType=Geo" => {
                                        new_lecsicon_entry.name_type = Some(NameType::Geographical);
                                    }
                                    "NameType=Prs" => {
                                        new_lecsicon_entry.name_type = Some(NameType::Person);
                                    }
                                    "Number=Coll" => {
                                        new_lecsicon_entry.number = Some(Number::Collective);
                                    }
                                    "Number=Plur" => {
                                        new_lecsicon_entry.number = Some(Number::Plural);
                                    }
                                    "Number=Sing" => {
                                        new_lecsicon_entry.number = Some(Number::Singular);
                                    }
                                    "Numform=Word" => {
                                        new_lecsicon_entry.number_form = Some(NumberForm::Word);
                                    }
                                    "Numtype=Card" => {
                                        new_lecsicon_entry.number_type = Some(NumberType::Cardinal);
                                    }
                                    "Numtype=Ord" => {
                                        new_lecsicon_entry.number_type = Some(NumberType::Ordinal);
                                    }
                                    "Person=0" => new_lecsicon_entry.person = Some(Person::Zero),
                                    "Person=1" => new_lecsicon_entry.person = Some(Person::One),
                                    "Person=2" => new_lecsicon_entry.person = Some(Person::Two),
                                    "Person=3" => new_lecsicon_entry.person = Some(Person::Three),
                                    "Polarity=Neg" => {
                                        new_lecsicon_entry.polarity = Some(Polarity::Negative)
                                    }
                                    "Polite=Form" => {
                                        new_lecsicon_entry.polite = Some(Polite::Formal)
                                    }
                                    "Polite=Inf" => {
                                        new_lecsicon_entry.polite = Some(Polite::Informal)
                                    }
                                    "Position=Prenom" => {
                                        new_lecsicon_entry.position = Some(Position::Prenom)
                                    }
                                    "Poss=Yes" => {
                                        new_lecsicon_entry.possessive = Some(Possessive::Yes)
                                    }
                                    "PronType=Art" => {
                                        new_lecsicon_entry.pronoun_type = Some(PronounType::Article)
                                    }
                                    "PronType=Dem" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Demonstrative)
                                    }
                                    "PronType=Int" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Interrogative)
                                    }
                                    "PronType=Neg" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Negative)
                                    }
                                    "PronType=Prs" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Personal)
                                    }
                                    "PronType=Rel" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Relative)
                                    }
                                    "PronType=Tot" => {
                                        new_lecsicon_entry.pronoun_type =
                                            Some(PronounType::Reflexive)
                                    }
                                    "Relative=Rel" => {
                                        new_lecsicon_entry.relative = Some(Relative::Rel)
                                    }
                                    "Style=Arch" => new_lecsicon_entry.style = Some(Style::Archaic),
                                    "Style=Coll" => {
                                        new_lecsicon_entry.style = Some(Style::Colloquial)
                                    }
                                    "Style=Form" => new_lecsicon_entry.style = Some(Style::Formal),
                                    "Tense=Fut" => new_lecsicon_entry.tense = Some(Tense::Future),
                                    "Tense=Imp" => {
                                        new_lecsicon_entry.tense = Some(Tense::Imperfect)
                                    }
                                    "Tense=Past" => new_lecsicon_entry.tense = Some(Tense::Past),
                                    "Tense=Pqp" => {
                                        new_lecsicon_entry.tense = Some(Tense::Plusquamperfekt)
                                    }
                                    "Tense=Pres" => new_lecsicon_entry.tense = Some(Tense::Present),
                                    "VerbForm=Vnoun" => {
                                        new_lecsicon_entry.verb_form = Some(VerbForm::Verbnoun)
                                    }
                                    "" => {}
                                    " " => {}
                                    _ => panic!("Unrecognised tag pair: <{tag_pair}>"),
                                }
                            }
                        }
                        all_entries.push(new_lecsicon_entry);
                    }
                    Err(res) => {
                        println!("problem with the following entry:");
                        dbg!(new_lecsicon_entry);
                        dbg!(res);
                    }
                }
            }
            Some(all_entries)
        }
        Err(_) => {
            println!("loadtextlecsicon failed");
            None
        }
    }
}
pub fn convert_sqlite_to_text(sqlite_entry: SqliteLecsiconEntry) -> LecsiconEntry {
    LecsiconEntry {
        id: sqlite_entry.id,
        word: sqlite_entry.word,
        lemma: sqlite_entry.lemma,
        tag: match sqlite_entry.tag {
            Some(tag) => match tag {
                0 => Some(Tag::Adjective),
                1 => Some(Tag::Adposition),
                2 => Some(Tag::Adverb),
                3 => Some(Tag::Conjugation),
                4 => Some(Tag::Determiner),
                5 => Some(Tag::Interjection),
                6 => Some(Tag::Noun),
                7 => Some(Tag::Number),
                8 => Some(Tag::Particle),
                9 => Some(Tag::Pronoun),
                10 => Some(Tag::ProperNoun),
                11 => Some(Tag::Verb),
                _ => {
                    dbg!(tag);
                    panic!("tag not recognised");
                }
            },
            None => None,
        },
        adposition_type: sqlite_entry
            .adposition_type
            .map(|_true_var| AdpositionType::Preposition),
        definite: sqlite_entry.definite.map(|_true_var| Definite::Definite),
        degree: match sqlite_entry.degree {
            Some(degree) => match degree {
                0 => Some(Degree::Comparative),
                1 => Some(Degree::Equative),
                2 => Some(Degree::Positive),
                3 => Some(Degree::Superlative),
                _ => {
                    dbg!(degree);
                    panic!("degree not recognised");
                }
            },
            None => None,
        },
        gender: match sqlite_entry.gender {
            Some(gender) => match gender {
                0 => Some(Gender::Feminine),
                1 => Some(Gender::FeminineMasculine),
                2 => Some(Gender::Masculine),
                _ => {
                    dbg!(gender);
                    panic!("gender not recognised");
                }
            },
            None => None,
        },
        mood: match sqlite_entry.mood {
            Some(mood) => match mood {
                0 => Some(Mood::Imperative),
                1 => Some(Mood::Indicative),
                2 => Some(Mood::Subjunctive),
                _ => {
                    dbg!(mood);
                    panic!("mood not recognised");
                }
            },
            None => None,
        },
        mutation: match sqlite_entry.mutation {
            Some(mutation) => match mutation {
                0 => Some(Mutation::Aspirate),
                1 => Some(Mutation::HProthesis),
                2 => Some(Mutation::Nasal),
                3 => Some(Mutation::Soft),
                _ => {
                    dbg!(mutation);
                    panic!("mutation not recognised");
                }
            },
            None => None,
        },
        name_type: match sqlite_entry.name_type {
            Some(name_type) => match name_type {
                0 => Some(NameType::Geographical),
                1 => Some(NameType::Person),
                _ => {
                    dbg!(name_type);
                    panic!("name_type not recognised");
                }
            },
            None => None,
        },
        number: match sqlite_entry.number {
            Some(number) => match number {
                0 => Some(Number::Collective),
                1 => Some(Number::Plural),
                2 => Some(Number::Singular),
                _ => {
                    dbg!(number);
                    panic!("number not recognised");
                }
            },
            None => None,
        },
        number_form: sqlite_entry.number_form.map(|_true_var| NumberForm::Word),
        number_type: match sqlite_entry.number_type {
            Some(n_t) => match n_t {
                0 => Some(NumberType::Cardinal),
                1 => Some(NumberType::Ordinal),
                _ => {
                    dbg!(n_t);
                    panic!("number_type not recognised");
                }
            },
            None => None,
        },
        person: match sqlite_entry.person {
            Some(person) => match person {
                0 => Some(Person::One),
                1 => Some(Person::Three),
                2 => Some(Person::Two),
                3 => Some(Person::Zero),
                _ => {
                    dbg!(person);
                    panic!("person not recognised");
                }
            },
            None => None,
        },
        polarity: sqlite_entry.polarity.map(|_true_var| Polarity::Negative),
        polite: match sqlite_entry.polite {
            Some(p) => match p {
                0 => Some(Polite::Formal),
                1 => Some(Polite::Informal),
                _ => {
                    dbg!(p);
                    panic!("polite not recognised");
                }
            },
            None => None,
        },
        position: sqlite_entry.position.map(|_true_var| Position::Prenom),
        possessive: sqlite_entry.possessive.map(|_true_var| Possessive::Yes),
        pronoun_type: match sqlite_entry.pronoun_type {
            Some(p_t) => match p_t {
                0 => Some(PronounType::Article),
                1 => Some(PronounType::Demonstrative),
                2 => Some(PronounType::Interrogative),
                3 => Some(PronounType::Negative),
                4 => Some(PronounType::Personal),
                5 => Some(PronounType::Reflexive),
                6 => Some(PronounType::Relative),
                _ => {
                    dbg!(p_t);
                    panic!("pron_type not recognised");
                }
            },
            None => None,
        },
        relative: sqlite_entry.relative.map(|_true_var| Relative::Rel),
        style: match sqlite_entry.style {
            Some(s) => match s {
                0 => Some(Style::Archaic),
                1 => Some(Style::Colloquial),
                2 => Some(Style::Formal),
                _ => {
                    dbg!(s);
                    panic!("style not recognised");
                }
            },
            None => None,
        },
        tense: match sqlite_entry.tense {
            Some(t) => match t {
                0 => Some(Tense::Future),
                1 => Some(Tense::Imperfect),
                2 => Some(Tense::Past),
                3 => Some(Tense::Plusquamperfekt),
                4 => Some(Tense::Present),
                _ => {
                    dbg!(t);
                    panic!("tense not recognised");
                }
            },
            None => None,
        },
        verb_form: sqlite_entry.verb_form.map(|_true_var| VerbForm::Verbnoun),
    }
}

pub fn convert_text_to_sqlite(text_entry: LecsiconEntry) -> SqliteLecsiconEntry {
    SqliteLecsiconEntry {
        id: text_entry.id,
        word: text_entry.word,
        lemma: text_entry.lemma,
        tag: match text_entry.tag {
            Some(tag) => match tag {
                Tag::Adjective => Some(0),
                Tag::Adposition => Some(1),
                Tag::Adverb => Some(2),
                Tag::Conjugation => Some(3),
                Tag::Determiner => Some(4),
                Tag::Interjection => Some(5),
                Tag::Noun => Some(6),
                Tag::Number => Some(7),
                Tag::Particle => Some(8),
                Tag::Pronoun => Some(9),
                Tag::ProperNoun => Some(10),
                Tag::Verb => Some(11),
            },
            None => None,
        },
        adposition_type: text_entry
            .adposition_type
            .map(|AdpositionType::Preposition| true),
        definite: text_entry.definite.map(|Definite::Definite| true),
        degree: match text_entry.degree {
            Some(degree) => match degree {
                Degree::Comparative => Some(0),
                Degree::Equative => Some(1),
                Degree::Positive => Some(2),
                Degree::Superlative => Some(3),
            },
            None => None,
        },
        gender: match text_entry.gender {
            Some(gender) => match gender {
                Gender::Feminine => Some(0),
                Gender::FeminineMasculine => Some(1),
                Gender::Masculine => Some(2),
            },
            None => None,
        },
        mood: match text_entry.mood {
            Some(mood) => match mood {
                Mood::Imperative => Some(0),
                Mood::Indicative => Some(1),
                Mood::Subjunctive => Some(2),
            },
            None => None,
        },
        mutation: match text_entry.mutation {
            Some(mutation) => match mutation {
                Mutation::Aspirate => Some(0),
                Mutation::HProthesis => Some(1),
                Mutation::Nasal => Some(2),
                Mutation::Soft => Some(3),
            },
            None => None,
        },
        name_type: match text_entry.name_type {
            Some(name_type) => match name_type {
                NameType::Geographical => Some(0),
                NameType::Person => Some(1),
            },
            None => None,
        },
        number: match text_entry.number {
            Some(number) => match number {
                Number::Collective => Some(0),
                Number::Plural => Some(1),
                Number::Singular => Some(2),
            },
            None => None,
        },
        number_form: text_entry.number_form.map(|NumberForm::Word| true),
        number_type: match text_entry.number_type {
            Some(n_t) => match n_t {
                NumberType::Cardinal => Some(0),
                NumberType::Ordinal => Some(1),
            },
            None => None,
        },
        person: match text_entry.person {
            Some(person) => match person {
                Person::One => Some(0),
                Person::Three => Some(1),
                Person::Two => Some(2),
                Person::Zero => Some(3),
            },
            None => None,
        },
        polarity: text_entry.polarity.map(|Polarity::Negative| true),
        polite: match text_entry.polite {
            Some(p) => match p {
                Polite::Formal => Some(0),
                Polite::Informal => Some(1),
            },
            None => None,
        },
        position: text_entry.position.map(|Position::Prenom| true),
        possessive: text_entry.possessive.map(|Possessive::Yes| true),
        pronoun_type: match text_entry.pronoun_type {
            Some(p_t) => match p_t {
                PronounType::Article => Some(0),
                PronounType::Demonstrative => Some(1),
                PronounType::Interrogative => Some(2),
                PronounType::Negative => Some(3),
                PronounType::Personal => Some(4),
                PronounType::Reflexive => Some(5),
                PronounType::Relative => Some(6),
            },
            None => None,
        },
        relative: text_entry.relative.map(|Relative::Rel| true),
        style: match text_entry.style {
            Some(s) => match s {
                Style::Archaic => Some(0),
                Style::Colloquial => Some(1),
                Style::Formal => Some(2),
            },
            None => None,
        },
        tense: match text_entry.tense {
            Some(t) => match t {
                Tense::Future => Some(0),
                Tense::Imperfect => Some(1),
                Tense::Past => Some(2),
                Tense::Plusquamperfekt => Some(3),
                Tense::Present => Some(4),
            },
            None => None,
        },
        verb_form: text_entry.verb_form.map(|VerbForm::Verbnoun| true),
    }
}

pub fn create_entry(conn: &mut SqliteConnection, new_entry: &SqliteLecsiconEntry) -> bool {
    match diesel::insert_into(crate::schema::lecsicon::table)
        .values(new_entry)
        .execute(conn)
    {
        Ok(_t) => true,
        Err(_e) => false,
    }
}

pub fn establish_connection(db_filepath: &Path) -> Result<Rc<SqliteConnection>, Box<dyn Error>> {
    // you can set up a database in RAM for faster access with the commands:
    // sudo mount -t tmpfs -o size=500M tmpfs path/to/tmp_fs
    // cp database.db path/to/tmp_fs/database.db
    if db_filepath.exists() {
        if let Some(db_file_path_str) = db_filepath.to_str() {
            return Ok(Rc::new(SqliteConnection::establish(db_file_path_str)?));
        }
    }
    Err("unable to establish connection with database".into())
}

pub fn get_lecsicon_entries_by_word(
    word: &str,
    conn: &mut SqliteConnection,
) -> Vec<SqliteLecsiconEntry> {
    crate::schema::lecsicon::table
        .filter(crate::schema::lecsicon::word.eq(word))
        .load(conn)
        .expect("get_lecsicon_entries_by_word(): Error searching for entries by word")
}

pub fn get_related_entries(
    entry: LecsiconEntry,
    db_connection: &mut SqliteConnection,
) -> Option<Vec<LecsiconEntry>> {
    let sqlite_compatible_entry = convert_text_to_sqlite(entry.clone());
    let mut entries: Vec<LecsiconEntry> = match entry.tag {
        Some(Tag::Verb) => {
            if entry.verb_form.is_some() {
                // Berfenw
                crate::schema::lecsicon::table
                    .filter(crate::schema::lecsicon::lemma.eq(&sqlite_compatible_entry.word))
                    .filter(crate::schema::lecsicon::tag.eq(&sqlite_compatible_entry.tag))
                    .load(db_connection)
                    .expect("get_related_entries(): Error searching for entries")
                    .into_iter()
                    .map(convert_sqlite_to_text)
                    .collect()
            } else {
                // Berf yn unig
                crate::schema::lecsicon::table
                    .filter(crate::schema::lecsicon::lemma.eq(&sqlite_compatible_entry.word))
                    .filter(crate::schema::lecsicon::tag.eq(&sqlite_compatible_entry.tag))
                    .filter(crate::schema::lecsicon::mood.eq(&sqlite_compatible_entry.mood))
                    .filter(crate::schema::lecsicon::tense.eq(&sqlite_compatible_entry.tense))
                    .filter(crate::schema::lecsicon::number.eq(&sqlite_compatible_entry.number))
                    .filter(crate::schema::lecsicon::person.eq(&sqlite_compatible_entry.person))
                    .load(db_connection)
                    .expect("get_related_entries(): Error searching for entries")
                    .into_iter()
                    .map(convert_sqlite_to_text)
                    .collect()
            }
        }
        _ => crate::schema::lecsicon::table
            .filter(crate::schema::lecsicon::lemma.eq(&sqlite_compatible_entry.word))
            .filter(crate::schema::lecsicon::tag.eq(&sqlite_compatible_entry.tag))
            .load(db_connection)
            .expect("get_related_entries(): Error searching for entries")
            .into_iter()
            .map(convert_sqlite_to_text)
            .collect(),
    };

    if entries.is_empty() {
        None
    } else {
        // filter out the entry that these related entries
        //  are linked to (don't return itself)
        entries.retain(|e| *e != entry);
        Some(entries)
    }
}

pub fn convert_csv_to_sqlite_format(data: Vec<LecsiconEntry>) -> Vec<SqliteLecsiconEntry> {
    data.into_iter()
        .map(convert_text_to_sqlite)
        .collect::<Vec<SqliteLecsiconEntry>>()
}

pub fn save_data_to_sqlite_db(data: Vec<SqliteLecsiconEntry>, db_file_path: &Path) -> bool {
    if let Ok(mut conn) = establish_connection(db_file_path) {
        for mut entry in data {
            if !create_entry(Rc::get_mut(&mut conn).unwrap(), &entry) {
                let same_words =
                    get_lecsicon_entries_by_word(&entry.word, Rc::get_mut(&mut conn).unwrap());
                for e in same_words {
                    if e.id >= entry.id {
                        entry.id = e.id + 1;
                    }
                }
                if !create_entry(Rc::get_mut(&mut conn).unwrap(), &entry) {
                    dbg!(entry);
                    panic!("re-attempt failed");
                }
            }
        }
        return true;
    }
    false
}

impl SqliteSearcher {
    pub fn new(connection: Rc<SqliteConnection>, recent_searches: Vec<String>) -> SqliteSearcher {
        SqliteSearcher {
            input: "".to_string(),
            recent_searches,
            connection,
            commands: vec![],
        }
    }
}

impl Autocomplete for SqliteSearcher {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        if input.starts_with('/') {
            return Ok(self
                .commands
                .clone()
                .iter()
                .filter(|s| s.starts_with(input))
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        } else if input.len() > 2 {
            let mut results: Vec<String> = crate::schema::lecsicon::table
                .filter(crate::schema::lecsicon::word.like(format!("{}%", input)))
                .load(Rc::get_mut(&mut self.connection).unwrap())
                .expect("get_suggestions(): Error searching for entries by word")
                .iter()
                .map(|entry: &SqliteLecsiconEntry| String::from(&entry.word))
                .collect();
            if results.is_empty() {
                let c_dic = Hunspell::new(
                    "/usr/share/hunspell/cy_GB.aff",
                    "/usr/share/hunspell/cy_GB.dic",
                );
                results = c_dic.suggest(input);
            } else {
                results.sort_unstable();
                results.dedup();
            }
            Ok(results)
        } else if input.is_empty() {
            return Ok(self.recent_searches.to_owned());
        } else {
            return Ok(self
                .recent_searches
                .iter()
                .filter(|w| w.starts_with(input))
                .cloned()
                .collect::<Vec<String>>());
        }
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(match highlighted_suggestion {
            Some(suggestion) => {
                self.recent_searches.push(input.to_string());
                Replacement::Some(suggestion)
            }
            None => None,
        })
    }
}
