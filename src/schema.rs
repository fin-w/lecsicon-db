// @generated automatically by Diesel CLI.

diesel::table! {
    lecsicon (id, word) {
        id -> Integer,
        word -> Text,
        lemma -> Text,
        tag -> Text,
        adp_type -> Nullable<Text>,
        definite -> Nullable<Text>,
        degree -> Nullable<Text>,
        gender -> Nullable<Text>,
        mood -> Nullable<Text>,
        mutation -> Nullable<Text>,
        name_type -> Nullable<Text>,
        number -> Nullable<Text>,
        num_form -> Nullable<Text>,
        num_type -> Nullable<Text>,
        person -> Nullable<Integer>,
        polarity -> Nullable<Text>,
        polite -> Nullable<Text>,
        position -> Nullable<Text>,
        poss -> Nullable<Text>,
        pron_type -> Nullable<Text>,
        relative -> Nullable<Text>,
        style -> Nullable<Text>,
        tense -> Nullable<Text>,
        verb_form -> Nullable<Text>,
        word_use_frequency -> Nullable<Integer>,
    }
}
