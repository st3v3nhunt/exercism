#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    // create an instance of the struct using an _associated_ function (via `impl HighScores`)
    // `new` is not special, but a convention
    // `Self` is the struct type
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        scores.iter().rev().take(3).copied().collect()
        // scores.reverse()
        // scores.truncate(3);
        // scores
    }
}
