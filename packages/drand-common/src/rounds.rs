use cosmwasm_std::Timestamp;

use crate::{DRAND_GENESIS, DRAND_ROUND_LENGTH};

// See TimeOfRound implementation: https://github.com/drand/drand/blob/eb36ba81e3f28c966f95bcd602f60e7ff8ef4c35/chain/time.go#L30-L33
pub fn time_of_round(round: u64) -> Timestamp {
    DRAND_GENESIS.plus_nanos((round - 1) * DRAND_ROUND_LENGTH)
}

pub fn round_after(base: Timestamp) -> u64 {
    // Losely ported from https://github.com/drand/drand/blob/eb36ba81e3f28c966f95bcd602f60e7ff8ef4c35/chain/time.go#L49-L63
    if base < DRAND_GENESIS {
        1
    } else {
        let from_genesis = base.nanos() - DRAND_GENESIS.nanos();
        let periods_since_genesis = from_genesis / DRAND_ROUND_LENGTH;
        let next_period_index = periods_since_genesis + 1;
        next_period_index + 1 // Convert 0-based counting to 1-based counting
    }
}

/// Returns true if and only if the round number is incentivised for Nois.
/// For mainnet launch, every 10th round is considered valid.
/// For fast randomness, all rounds are valid but only every 10th round is incentivised.
///
/// If round is 0, this returns false because there is no 0 round in drand.
#[inline]
pub fn is_incentivised(round: u64) -> bool {
    round != 0 && round % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_of_round_works() {
        assert_eq!(time_of_round(1), DRAND_GENESIS);
        assert_eq!(time_of_round(2), DRAND_GENESIS.plus_seconds(3));
        assert_eq!(time_of_round(111765), Timestamp::from_seconds(1678020492));
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn time_of_round_panics_for_round_0() {
        time_of_round(0);
    }

    #[test]
    fn round_after_works() {
        // UNIX epoch
        let round = round_after(Timestamp::from_seconds(0));
        assert_eq!(round, 1);

        // Before Drand genesis (https://api3.drand.sh/dbd506d6ef76e5f386f41c651dcb808c5bcbd75471cc4eafa3f4df7ad4e4c493/info)
        let round = round_after(Timestamp::from_seconds(1677685200).minus_nanos(1));
        assert_eq!(round, 1);

        // At Drand genesis
        let round = round_after(Timestamp::from_seconds(1677685200));
        assert_eq!(round, 2);

        // After Drand genesis
        let round = round_after(Timestamp::from_seconds(1677685200).plus_nanos(1));
        assert_eq!(round, 2);

        // Drand genesis +2s/3s/4s
        let round = round_after(Timestamp::from_seconds(1677685200).plus_seconds(2));
        assert_eq!(round, 2);
        let round = round_after(Timestamp::from_seconds(1677685200).plus_seconds(3));
        assert_eq!(round, 3);
        let round = round_after(Timestamp::from_seconds(1677685200).plus_seconds(4));
        assert_eq!(round, 3);
    }

    #[test]
    fn is_incentivised_works() {
        assert!(!is_incentivised(0)); // no 0 round exists in drand
        assert!(!is_incentivised(1));
        assert!(!is_incentivised(2));
        assert!(!is_incentivised(3));
        assert!(!is_incentivised(4));
        assert!(!is_incentivised(5));
        assert!(!is_incentivised(6));
        assert!(!is_incentivised(7));
        assert!(!is_incentivised(8));
        assert!(!is_incentivised(9));
        assert!(is_incentivised(10));
        assert!(!is_incentivised(11));
        assert!(!is_incentivised(12));
        assert!(!is_incentivised(13));
        assert!(!is_incentivised(14));
        assert!(!is_incentivised(15));
        assert!(!is_incentivised(16));
        assert!(!is_incentivised(17));
        assert!(!is_incentivised(18));
        assert!(!is_incentivised(19));
        assert!(is_incentivised(20));
        assert!(!is_incentivised(21));
        assert!(!is_incentivised(22));
        assert!(!is_incentivised(23));
        assert!(!is_incentivised(24));
        assert!(!is_incentivised(25));
        assert!(!is_incentivised(26));
        assert!(!is_incentivised(27));
        assert!(!is_incentivised(28));
        assert!(!is_incentivised(29));
        assert!(is_incentivised(30));
        assert!(!is_incentivised(31));
    }
}
