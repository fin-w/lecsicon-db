// @generated automatically by Diesel CLI.

diesel::table! {
    lecsicon (id, word) {
        id -> Integer,
        word -> Text,
        lemma -> Nullable<Text>,
        tag -> Nullable<Integer>,
        adposition_type -> Nullable<Bool>,
        definite -> Nullable<Bool>,
        degree -> Nullable<Integer>,
        gender -> Nullable<Integer>,
        mood -> Nullable<Integer>,
        mutation -> Nullable<Integer>,
        name_type -> Nullable<Integer>,
        number -> Nullable<Integer>,
        number_form -> Nullable<Bool>,
        number_type -> Nullable<Integer>,
        person -> Nullable<Integer>,
        polarity -> Nullable<Bool>,
        polite -> Nullable<Integer>,
        position -> Nullable<Bool>,
        possessive -> Nullable<Bool>,
        pronoun_type -> Nullable<Integer>,
        relative -> Nullable<Bool>,
        style -> Nullable<Integer>,
        tense -> Nullable<Integer>,
        verb_form -> Nullable<Bool>,
    }
}
