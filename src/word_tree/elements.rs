#[derive(Clone, PartialEq)]
pub enum WordElement {
    BeginWord,
    Letter(char),
    EndWord
}