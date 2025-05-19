use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct IndexLessThan<const LT: usize>(usize);
// where
//     DT: std::fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord;
// {
//     // less_then: DT,
//     // #[deref]
//     value: DT,
// }

impl<const LT: usize> TryFrom<usize> for IndexLessThan<LT> {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < LT {
            Ok(Self(value))
        } else {
            Err(format!(
                "Too big Error! {value} was expected to be less than {LT}"
            ))
        }
    }
}
