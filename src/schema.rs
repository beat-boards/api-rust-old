#[derive(Debug, DbEnum, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}

table! {
    use super::DifficultyMapping;
    use diesel::sql_types::{
        Uuid,
        Text,
        Double,
        Integer,
    };
    maps (id) {
        id -> Uuid,
        hash -> Text,
        difficulty -> DifficultyMapping,
        song_name -> Text,
        song_sub_name -> Text,
        song_author_name -> Text,
        level_author_name -> Text,
        difficulty_rating -> Double,
        length -> Double,
        bpm -> Double,
        note_jump_speed -> Double,
        note_count -> Integer,
        complexity -> Double,
        saber_distance -> Double,
        max_rp -> Double,
        upvotes -> Integer,
        downvotes -> Integer,
    }
}

#[derive(Debug, DbEnum, Serialize, Deserialize)]
pub enum Modifier {
    DisappearingArrows,
    FasterSong,
    GhostNotes,
    NoArrows,
    NoBombs,
    NoFail,
    NoObstacles,
    SlowerSong,
}

table! {
    use super::ModifierMapping;
    use diesel::sql_types::{
        Uuid,
        Timestamp,
        Integer,
        Double,
        Array,
    };
    scores (id) {
        id -> Uuid,
        user -> Uuid,
        map -> Uuid,
        date -> Timestamp,
        raw_score -> Integer,
        raw_percentage -> Double,
        modifiers -> Array<ModifierMapping>,
        adjusted_score -> Integer,
        raw_rp -> Double,
        adjusted_rp -> Double,
    }
}

#[derive(Debug, DbEnum, Serialize, Deserialize)]
pub enum Role {
    Owner,
    Contributor,
    Supporter,
    Ranker,
    Curator,
    ScoreSaber,
    Player,
    Toxic,
}

table! {
    use super::RoleMapping;
    use diesel::sql_types::{
        Uuid,
        Nullable,
        BigInt,
        Text,
        Bool,
        Double,
        Integer,
        Array,
    };
    users (id) {
        id -> Uuid,
        steam_id -> Nullable<BigInt>,
        oculus_id -> Nullable<Text>,
        banned -> Bool,
        username -> Text,
        role -> RoleMapping,
        country -> Text,
        rp -> Double,
        fails -> Integer,
        following -> Array<Uuid>,
        image -> Nullable<Text>,
    }
}

joinable!(scores -> maps (map));
joinable!(scores -> users (user));

allow_tables_to_appear_in_same_query!(maps, scores, users,);
