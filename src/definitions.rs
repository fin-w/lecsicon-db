use crate::schema::lecsicon;

use diesel::prelude::*;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Adjective,
    Adposition,
    Adverb,
    Conjugation,
    Determiner,
    Interjection,
    Noun,
    Number,
    Particle,
    Pronoun,
    ProperNoun,
    Verb,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AdpositionType {
    Preposition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Definite {
    Definite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Degree {
    Comparative,
    Equative,
    Positive,
    Superlative,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gender {
    Feminine,
    FeminineMasculine,
    Masculine,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mood {
    Imperative,
    Indicative,
    Subjunctive,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mutation {
    Aspirate,
    HProthesis,
    Nasal,
    Soft,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NameType {
    Geographical,
    Person,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Collective,
    Plural,
    Singular,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberForm {
    Word,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NumberType {
    Cardinal,
    Ordinal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Person {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Polarity {
    Negative,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Polite {
    Formal,
    Informal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Position {
    Prenom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Possessive {
    Yes,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PronounType {
    Article,
    Demonstrative,
    Interrogative,
    Negative,
    Personal,
    Relative,
    Reflexive,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Relative {
    Rel,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Style {
    Archaic,
    Colloquial,
    Formal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tense {
    Future,
    Imperfect,
    Past,
    Plusquamperfekt,
    Present,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VerbForm {
    Verbnoun,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LecsiconEntry {
    pub id: i32,
    pub word: String,
    pub lemma: Option<String>,
    pub tag: Option<Tag>,
    pub adposition_type: Option<AdpositionType>,
    pub definite: Option<Definite>,
    pub degree: Option<Degree>,
    pub gender: Option<Gender>,
    pub mood: Option<Mood>,
    pub mutation: Option<Mutation>,
    pub name_type: Option<NameType>,
    pub number: Option<Number>,
    pub number_form: Option<NumberForm>,
    pub number_type: Option<NumberType>,
    pub person: Option<Person>,
    pub polarity: Option<Polarity>,
    pub polite: Option<Polite>,
    pub position: Option<Position>,
    pub possessive: Option<Possessive>,
    pub pronoun_type: Option<PronounType>,
    pub relative: Option<Relative>,
    pub style: Option<Style>,
    pub tense: Option<Tense>,
    pub verb_form: Option<VerbForm>,
}

#[derive(Queryable, Selectable, Insertable, Debug, Clone, PartialEq)]
#[diesel(table_name = lecsicon)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SqliteLecsiconEntry {
    pub id: i32,
    pub word: String,
    pub lemma: Option<String>,
    pub tag: Option<i32>,
    pub adposition_type: Option<bool>,
    pub definite: Option<bool>,
    pub degree: Option<i32>,
    pub gender: Option<i32>,
    pub mood: Option<i32>,
    pub mutation: Option<i32>,
    pub name_type: Option<i32>,
    pub number: Option<i32>,
    pub number_form: Option<bool>,
    pub number_type: Option<i32>,
    pub person: Option<i32>,
    pub polarity: Option<bool>,
    pub polite: Option<i32>,
    pub position: Option<bool>,
    pub possessive: Option<bool>,
    pub pronoun_type: Option<i32>,
    pub relative: Option<bool>,
    pub style: Option<i32>,
    pub tense: Option<i32>,
    pub verb_form: Option<bool>,
}

#[derive(Clone)]
pub struct SqliteSearcher {
    pub input: String,
    pub recent_searches: Vec<String>,
    pub connection: Rc<SqliteConnection>,
    pub commands: Vec<String>,
}
