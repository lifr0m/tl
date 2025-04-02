#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Id(
    pub(crate) [u8; 4]
);

pub(crate) trait Identify {
    const ID: Id;
}
