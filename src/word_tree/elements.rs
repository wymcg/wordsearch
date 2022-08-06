#[derive(Clone, PartialEq)]
pub(crate) enum WordElement {
    BeginWord,
    Letter(char),
    EndWord
}