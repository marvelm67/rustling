// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}


fn build_scores_table(results: &str) -> HashMap<String, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<String, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap().to_string();
        let team_2_name = split_iterator.next().unwrap().to_string();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Update the scores for team 1
        scores
            .entry(team_1_name.clone())
            .and_modify(|team| {
                team.goals_scored += team_1_score;
                team.goals_conceded += team_2_score;
            })
            .or_insert(TeamScores {
                name: team_1_name.clone(),
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            });

        // Update the scores for team 2
        scores
            .entry(team_2_name.clone())
            .and_modify(|team| {
                team.goals_scored += team_2_score;
                team.goals_conceded += team_1_score;
            })
            .or_insert(TeamScores {
                name: team_2_name.clone(), 
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            });
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
