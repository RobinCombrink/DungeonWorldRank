use super::*;

fn profile_response() -> CharacterResponse {
    serde_json::from_str(include_str!("../fixtures/character_profile.json"))
        .expect("the committed profile body deserialises")
}

#[test]
fn profile_body_converts_into_the_rank_the_printing_reads() {
    let character_rank = CharacterRank::from(&profile_response());

    assert_eq!(character_rank.name, "Meleekick");
    assert_eq!(character_rank.realm, "Ravencrest");
    assert_eq!(character_rank.region, "eu");
    assert_eq!(character_rank.world_rank, 188579);
    assert_eq!(character_rank.class_rank, 19633);
}

#[test]
fn profile_body_carries_the_class_the_per_class_total_is_queried_for() {
    assert!(matches!(profile_response().class, Class::Mage));
}
