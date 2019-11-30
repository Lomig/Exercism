#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {
            return vec![];
        }

        let mut sorted_scores = self.scores.to_vec().clone();
        sorted_scores.sort();
        sorted_scores.reverse();

        sorted_scores
            .chunks(3)
            .next()
            .unwrap()
            .to_vec()
    }
}
