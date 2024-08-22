use super::license_plate_enums::{Country, StateOrProvince};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[allow(dead_code)]
static VALID_GAME_PLATES_WITH_BORDERS: LazyLock<
    HashMap<(Country, StateOrProvince), HashSet<(Country, StateOrProvince)>>,
> = LazyLock::new(|| {
    // TODO add Canadian borders (will need milestones and score updates)
    // TODO consider adding cross-country borders (eg. coast-to-coast through Canada)
    let map = HashMap::from([
        // US
        (
            (Country::US, StateOrProvince::AL),
            HashSet::from([
                (Country::US, StateOrProvince::FL),
                (Country::US, StateOrProvince::GA),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::MS),
            ]),
        ),
        ((Country::US, StateOrProvince::AK), HashSet::from([])),
        (
            (Country::US, StateOrProvince::AZ),
            HashSet::from([
                (Country::US, StateOrProvince::NM),
                (Country::US, StateOrProvince::UT),
                (Country::US, StateOrProvince::NV),
                (Country::US, StateOrProvince::CA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::AR),
            HashSet::from([
                (Country::US, StateOrProvince::LA),
                (Country::US, StateOrProvince::MS),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::OK),
                (Country::US, StateOrProvince::TX),
            ]),
        ),
        (
            (Country::US, StateOrProvince::CA),
            HashSet::from([
                (Country::US, StateOrProvince::AZ),
                (Country::US, StateOrProvince::NV),
                (Country::US, StateOrProvince::OR),
            ]),
        ),
        (
            (Country::US, StateOrProvince::CO),
            HashSet::from([
                (Country::US, StateOrProvince::NM),
                (Country::US, StateOrProvince::OK),
                (Country::US, StateOrProvince::KS),
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::WY),
                (Country::US, StateOrProvince::UT),
            ]),
        ),
        (
            (Country::US, StateOrProvince::CT),
            HashSet::from([
                (Country::US, StateOrProvince::RI),
                (Country::US, StateOrProvince::MA),
                (Country::US, StateOrProvince::NY),
            ]),
        ),
        (
            (Country::US, StateOrProvince::DE),
            HashSet::from([
                (Country::US, StateOrProvince::NJ),
                (Country::US, StateOrProvince::PA),
                (Country::US, StateOrProvince::MD),
            ]),
        ),
        (
            (Country::US, StateOrProvince::FL),
            HashSet::from([
                (Country::US, StateOrProvince::GA),
                (Country::US, StateOrProvince::AL),
            ]),
        ),
        (
            (Country::US, StateOrProvince::GA),
            HashSet::from([
                (Country::US, StateOrProvince::SC),
                (Country::US, StateOrProvince::NC),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::AL),
                (Country::US, StateOrProvince::FL),
            ]),
        ),
        ((Country::US, StateOrProvince::HI), HashSet::from([])),
        (
            (Country::US, StateOrProvince::ID),
            HashSet::from([
                (Country::US, StateOrProvince::WA),
                (Country::US, StateOrProvince::OR),
                (Country::US, StateOrProvince::NV),
                (Country::US, StateOrProvince::UT),
                (Country::US, StateOrProvince::WY),
                (Country::US, StateOrProvince::MT),
            ]),
        ),
        (
            (Country::US, StateOrProvince::IL),
            HashSet::from([
                (Country::US, StateOrProvince::WI),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::IN),
            ]),
        ),
        (
            (Country::US, StateOrProvince::IN),
            HashSet::from([
                (Country::US, StateOrProvince::IL),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::OH),
                (Country::US, StateOrProvince::MI),
            ]),
        ),
        (
            (Country::US, StateOrProvince::IA),
            HashSet::from([
                (Country::US, StateOrProvince::MN),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::IL),
                (Country::US, StateOrProvince::WI),
            ]),
        ),
        (
            (Country::US, StateOrProvince::KS),
            HashSet::from([
                (Country::US, StateOrProvince::OK),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::CO),
            ]),
        ),
        (
            (Country::US, StateOrProvince::KY),
            HashSet::from([
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::VA),
                (Country::US, StateOrProvince::WV),
                (Country::US, StateOrProvince::OH),
                (Country::US, StateOrProvince::IN),
                (Country::US, StateOrProvince::IL),
                (Country::US, StateOrProvince::MO),
            ]),
        ),
        (
            (Country::US, StateOrProvince::LA),
            HashSet::from([
                (Country::US, StateOrProvince::MS),
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::TX),
            ]),
        ),
        (
            (Country::US, StateOrProvince::ME),
            HashSet::from([(Country::US, StateOrProvince::NH)]),
        ),
        (
            (Country::US, StateOrProvince::MD),
            HashSet::from([
                (Country::US, StateOrProvince::DE),
                (Country::US, StateOrProvince::PA),
                (Country::US, StateOrProvince::WV),
                (Country::US, StateOrProvince::VA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MA),
            HashSet::from([
                (Country::US, StateOrProvince::NH),
                (Country::US, StateOrProvince::VT),
                (Country::US, StateOrProvince::NY),
                (Country::US, StateOrProvince::CT),
                (Country::US, StateOrProvince::RI),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MI),
            HashSet::from([
                (Country::US, StateOrProvince::WI),
                (Country::US, StateOrProvince::IN),
                (Country::US, StateOrProvince::OH),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MN),
            HashSet::from([
                (Country::US, StateOrProvince::ND),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::WI),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MS),
            HashSet::from([
                (Country::US, StateOrProvince::AL),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::LA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MO),
            HashSet::from([
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::IL),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::KS),
                (Country::US, StateOrProvince::OK),
            ]),
        ),
        (
            (Country::US, StateOrProvince::MT),
            HashSet::from([
                (Country::US, StateOrProvince::ID),
                (Country::US, StateOrProvince::WY),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::ND),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NE),
            HashSet::from([
                (Country::US, StateOrProvince::KS),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::WY),
                (Country::US, StateOrProvince::CO),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NV),
            HashSet::from([
                (Country::US, StateOrProvince::AZ),
                (Country::US, StateOrProvince::UT),
                (Country::US, StateOrProvince::ID),
                (Country::US, StateOrProvince::OR),
                (Country::US, StateOrProvince::CA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NH),
            HashSet::from([
                (Country::US, StateOrProvince::VT),
                (Country::US, StateOrProvince::MA),
                (Country::US, StateOrProvince::ME),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NJ),
            HashSet::from([
                (Country::US, StateOrProvince::NY),
                (Country::US, StateOrProvince::PA),
                (Country::US, StateOrProvince::DE),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NM),
            HashSet::from([
                (Country::US, StateOrProvince::TX),
                (Country::US, StateOrProvince::OK),
                (Country::US, StateOrProvince::CO),
                (Country::US, StateOrProvince::AZ),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NY),
            HashSet::from([
                (Country::US, StateOrProvince::PA),
                (Country::US, StateOrProvince::NJ),
                (Country::US, StateOrProvince::CT),
                (Country::US, StateOrProvince::MA),
                (Country::US, StateOrProvince::VT),
            ]),
        ),
        (
            (Country::US, StateOrProvince::NC),
            HashSet::from([
                (Country::US, StateOrProvince::VA),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::GA),
                (Country::US, StateOrProvince::SC),
            ]),
        ),
        (
            (Country::US, StateOrProvince::ND),
            HashSet::from([
                (Country::US, StateOrProvince::MT),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::MN),
            ]),
        ),
        (
            (Country::US, StateOrProvince::OH),
            HashSet::from([
                (Country::US, StateOrProvince::MI),
                (Country::US, StateOrProvince::IN),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::WV),
                (Country::US, StateOrProvince::PA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::OK),
            HashSet::from([
                (Country::US, StateOrProvince::TX),
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::KS),
                (Country::US, StateOrProvince::CO),
                (Country::US, StateOrProvince::NM),
            ]),
        ),
        (
            (Country::US, StateOrProvince::OR),
            HashSet::from([
                (Country::US, StateOrProvince::CA),
                (Country::US, StateOrProvince::NV),
                (Country::US, StateOrProvince::ID),
                (Country::US, StateOrProvince::WA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::PA),
            HashSet::from([
                (Country::US, StateOrProvince::OH),
                (Country::US, StateOrProvince::WV),
                (Country::US, StateOrProvince::MD),
                (Country::US, StateOrProvince::DE),
                (Country::US, StateOrProvince::NJ),
                (Country::US, StateOrProvince::NY),
            ]),
        ),
        (
            (Country::US, StateOrProvince::RI),
            HashSet::from([
                (Country::US, StateOrProvince::MA),
                (Country::US, StateOrProvince::CT),
            ]),
        ),
        (
            (Country::US, StateOrProvince::SC),
            HashSet::from([
                (Country::US, StateOrProvince::NC),
                (Country::US, StateOrProvince::GA),
            ]),
        ),
        (
            (Country::US, StateOrProvince::SD),
            HashSet::from([
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::MN),
                (Country::US, StateOrProvince::ND),
                (Country::US, StateOrProvince::MT),
                (Country::US, StateOrProvince::WY),
            ]),
        ),
        (
            (Country::US, StateOrProvince::TN),
            HashSet::from([
                (Country::US, StateOrProvince::AL),
                (Country::US, StateOrProvince::GA),
                (Country::US, StateOrProvince::NC),
                (Country::US, StateOrProvince::VA),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::MO),
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::MS),
            ]),
        ),
        (
            (Country::US, StateOrProvince::TX),
            HashSet::from([
                (Country::US, StateOrProvince::LA),
                (Country::US, StateOrProvince::AR),
                (Country::US, StateOrProvince::OK),
                (Country::US, StateOrProvince::NM),
            ]),
        ),
        (
            (Country::US, StateOrProvince::UT),
            HashSet::from([
                (Country::US, StateOrProvince::AZ),
                (Country::US, StateOrProvince::CO),
                (Country::US, StateOrProvince::WY),
                (Country::US, StateOrProvince::ID),
                (Country::US, StateOrProvince::NV),
            ]),
        ),
        (
            (Country::US, StateOrProvince::VT),
            HashSet::from([
                (Country::US, StateOrProvince::NY),
                (Country::US, StateOrProvince::MA),
                (Country::US, StateOrProvince::NH),
            ]),
        ),
        (
            (Country::US, StateOrProvince::VA),
            HashSet::from([
                (Country::US, StateOrProvince::MD),
                (Country::US, StateOrProvince::WV),
                (Country::US, StateOrProvince::KY),
                (Country::US, StateOrProvince::TN),
                (Country::US, StateOrProvince::NC),
            ]),
        ),
        (
            (Country::US, StateOrProvince::WA),
            HashSet::from([
                (Country::US, StateOrProvince::OR),
                (Country::US, StateOrProvince::ID),
            ]),
        ),
        (
            (Country::US, StateOrProvince::WV),
            HashSet::from([
                (Country::US, StateOrProvince::VA),
                (Country::US, StateOrProvince::MD),
                (Country::US, StateOrProvince::PA),
                (Country::US, StateOrProvince::OH),
                (Country::US, StateOrProvince::KY),
            ]),
        ),
        (
            (Country::US, StateOrProvince::WI),
            HashSet::from([
                (Country::US, StateOrProvince::MN),
                (Country::US, StateOrProvince::IA),
                (Country::US, StateOrProvince::IL),
                (Country::US, StateOrProvince::MI),
            ]),
        ),
        (
            (Country::US, StateOrProvince::WY),
            HashSet::from([
                (Country::US, StateOrProvince::CO),
                (Country::US, StateOrProvince::NE),
                (Country::US, StateOrProvince::SD),
                (Country::US, StateOrProvince::MT),
                (Country::US, StateOrProvince::ID),
                (Country::US, StateOrProvince::UT),
            ]),
        ),
        // CA
        ((Country::CA, StateOrProvince::AB), HashSet::from([])),
        ((Country::CA, StateOrProvince::BC), HashSet::from([])),
        ((Country::CA, StateOrProvince::MB), HashSet::from([])),
        ((Country::CA, StateOrProvince::NB), HashSet::from([])),
        ((Country::CA, StateOrProvince::NL), HashSet::from([])),
        ((Country::CA, StateOrProvince::NT), HashSet::from([])),
        ((Country::CA, StateOrProvince::NS), HashSet::from([])),
        ((Country::CA, StateOrProvince::NU), HashSet::from([])),
        ((Country::CA, StateOrProvince::ON), HashSet::from([])),
        ((Country::CA, StateOrProvince::PE), HashSet::from([])),
        ((Country::CA, StateOrProvince::QC), HashSet::from([])),
        ((Country::CA, StateOrProvince::SK), HashSet::from([])),
        ((Country::CA, StateOrProvince::YT), HashSet::from([])),
    ]);

    map
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_contain_us_wa_plate() {
        let key = (Country::US, StateOrProvince::WA);

        assert!(
            VALID_GAME_PLATES_WITH_BORDERS.contains_key(&key),
            "Expected US-WA to be found"
        );
    }

    #[test]
    fn will_not_contain_ca_wa_plate() {
        let key = (Country::CA, StateOrProvince::WA);

        assert!(
            !VALID_GAME_PLATES_WITH_BORDERS.contains_key(&key),
            "Expected CA-WA to not be found"
        );
    }

    #[test]
    fn will_retrieve_correct_us_ca_borders() {
        let expected_borders = HashSet::from([
            (Country::US, StateOrProvince::AZ),
            (Country::US, StateOrProvince::NV),
            (Country::US, StateOrProvince::OR),
        ]);

        let key = (Country::US, StateOrProvince::CA);

        let actual_borders = VALID_GAME_PLATES_WITH_BORDERS.get_key_value(&key);

        assert!(actual_borders.is_some());
        let (_, actual_borders) = actual_borders.unwrap();

        assert_eq!(expected_borders, *actual_borders);
    }
}
