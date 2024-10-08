use std::collections::HashSet;
use std::sync::LazyLock;

use serde::Serialize;

use super::license_plate_enums::{Country, StateOrProvince};
use super::license_plates::SpottedPlate;

#[derive(Serialize)]
pub struct GameScoreResult {
    num_of_spotted_plates: u32,
    achievements: Vec<String>,
    total_score: u32,
}

impl GameScoreResult {
    /// Calculate total game score from spotted plates.
    /// Score is calculated based on the number of spotted plates and
    /// any special achievement bonuses such as `West Coast`.
    pub fn new(plates: &[SpottedPlate]) -> GameScoreResult {
        let plates_hash: HashSet<_> = plates.iter().collect();

        let num_of_spotted_plates = plates_hash.len() as u32;

        let (achievements, total_score) = [("West Coast", calc_west_coast_bonus(plates_hash))]
            .iter()
            .filter(|(_, (is_achieved, _))| *is_achieved)
            .fold(
                (Vec::new(), num_of_spotted_plates),
                |(mut achievements, mut total_score), (this_achievement, (_, this_score))| {
                    total_score += this_score;
                    achievements.push(String::from(*this_achievement));
                    (achievements, total_score)
                },
            );

        GameScoreResult {
            num_of_spotted_plates,
            achievements,
            total_score,
        }
    }
}

static WEST_COAST_STATES: LazyLock<HashSet<&SpottedPlate>> = LazyLock::new(|| {
    HashSet::from([
        &SpottedPlate {
            country: Country::US,
            state_or_province: StateOrProvince::CA,
        },
        &SpottedPlate {
            country: Country::US,
            state_or_province: StateOrProvince::OR,
        },
        &SpottedPlate {
            country: Country::US,
            state_or_province: StateOrProvince::WA,
        },
    ])
});

fn calc_west_coast_bonus(plates: HashSet<&SpottedPlate>) -> (bool, u32) {
    if WEST_COAST_STATES.is_subset(&plates) {
        return (true, 30);
    }

    (false, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_return_zero_total_score_on_no_spots() {
        let spotted_plates = [];

        let actual_score_result = GameScoreResult::new(&spotted_plates);

        assert_eq!(0, actual_score_result.num_of_spotted_plates);
        assert_eq!(0, actual_score_result.total_score);
        assert_eq!(0, actual_score_result.achievements.len());
    }

    #[test]
    fn will_return_valid_total_score_on_spots_without_achievements() {
        let spotted_plates = [
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::AB,
            },
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::AK,
            },
        ];

        let actual_score_result = GameScoreResult::new(&spotted_plates);

        assert_eq!(2, actual_score_result.num_of_spotted_plates);
        assert_eq!(2, actual_score_result.total_score);
        assert_eq!(0, actual_score_result.achievements.len());
    }

    #[test]
    fn will_return_valid_total_score_on_duplicate_spots_without_achievements() {
        let spotted_plates = [
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::AB,
            },
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::AB,
            },
        ];

        let actual_score_result = GameScoreResult::new(&spotted_plates);

        assert_eq!(1, actual_score_result.num_of_spotted_plates);
        assert_eq!(1, actual_score_result.total_score);
        assert_eq!(0, actual_score_result.achievements.len());
    }

    #[test]
    fn will_return_valid_total_score_on_west_coast_achievement() {
        let spotted_plates = [
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::CA,
            },
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::OR,
            },
            SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::WA,
            },
        ];

        let actual_score_result = GameScoreResult::new(&spotted_plates);

        assert_eq!(3, actual_score_result.num_of_spotted_plates);
        assert_eq!(33, actual_score_result.total_score);
        assert_eq!(1, actual_score_result.achievements.len());
        assert_eq!("West Coast", actual_score_result.achievements[0]);
    }

    #[test]
    fn will_return_west_coast_bonus_when_achieved_exactly() {
        let spotted_plates = HashSet::from([
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::CA,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::OR,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::WA,
            },
        ]);

        let (actual_is_achieved, actual_score) = calc_west_coast_bonus(spotted_plates);

        assert!(actual_is_achieved);
        assert_eq!(30, actual_score);
    }

    #[test]
    fn will_return_west_coast_bonus_when_achieved_with_other_states() {
        let spotted_plates = HashSet::from([
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::CA,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::OR,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::WA,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::NV,
            },
        ]);

        let (actual_is_achieved, actual_score) = calc_west_coast_bonus(spotted_plates);

        assert!(actual_is_achieved);
        assert_eq!(30, actual_score);
    }

    #[test]
    fn will_not_return_west_coast_bonus_when_not_achieved() {
        let spotted_plates = HashSet::from([
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::CA,
            },
            &SpottedPlate {
                country: Country::US,
                state_or_province: StateOrProvince::OR,
            },
        ]);

        let (actual_is_achieved, actual_score) = calc_west_coast_bonus(spotted_plates);

        assert!(!actual_is_achieved);
        assert_eq!(0, actual_score);
    }

    #[test]
    fn will_not_return_west_coast_bonus_when_not_spots() {
        let spotted_plates = HashSet::from([]);

        let (actual_is_achieved, actual_score) = calc_west_coast_bonus(spotted_plates);

        assert!(!actual_is_achieved);
        assert_eq!(0, actual_score);
    }
}
