use serde::Serialize;

/// A struct for keeping track of the upgrades currently
/// active. Points to an index in constant arrays.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct PHeadsUpgradeState {
    p_heads_idx: usize,
}

impl PHeadsUpgradeState {
    /// Starting state
    pub fn new() -> Self {
        PHeadsUpgradeState { p_heads_idx: 0 }
    }

    /// Can upgrade p heads
    pub fn can_upgrade(&self) -> bool {
        self.p_heads_idx < (PHEADS_UPGRADES.len() - 1)
    }
}

/// A struct for managing each possible upgrade to the probability of heads.
struct PHeadsUpgrade {
    prob: f64,
    cost: f64,
}

/// The array of available upgrades
static PHEADS_UPGRADES: [PHeadsUpgrade; 9] = [
    PHeadsUpgrade {
        prob: 0.20,
        cost: 0.0,
    },
    PHeadsUpgrade {
        prob: 0.25,
        cost: 0.01,
    },
    PHeadsUpgrade {
        prob: 0.30,
        cost: 0.10,
    },
    PHeadsUpgrade {
        prob: 0.35,
        cost: 1.00,
    },
    PHeadsUpgrade {
        prob: 0.40,
        cost: 10.00,
    },
    PHeadsUpgrade {
        prob: 0.45,
        cost: 100.00,
    },
    PHeadsUpgrade {
        prob: 0.50,
        cost: 1_000.00,
    },
    PHeadsUpgrade {
        prob: 0.55,
        cost: 10_000.00,
    },
    PHeadsUpgrade {
        prob: 0.60,
        cost: 100_000.00,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, true)]
    #[case(1, true)]
    #[case(2, true)]
    #[case(3, true)]
    #[case(4, true)]
    #[case(5, true)]
    #[case(6, true)]
    #[case(7, true)]
    #[case(8, false)]
    fn test_upgradea_p_heads(#[case] idx: usize, #[case] expected: bool) {
        let up = PHeadsUpgradeState { p_heads_idx: idx };
        assert_eq!(expected, up.can_upgrade());
    }
}
