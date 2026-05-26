#[derive(Debug, Clone, PartialEq)]
pub enum Side { Long, Short, Flat }

/// Tally agent votes; a quorum decides, otherwise stay flat.
pub fn decide(votes: &[Side], quorum: usize) -> Side {
    let longs = votes.iter().filter(|v| **v == Side::Long).count();
    let shorts = votes.iter().filter(|v| **v == Side::Short).count();
    if longs >= quorum && longs >= shorts { Side::Long }
    else if shorts >= quorum { Side::Short }
    else { Side::Flat }
}
