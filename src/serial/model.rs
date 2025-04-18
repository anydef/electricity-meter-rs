use sml_rs::parser::OctetStr;

#[derive(PartialEq, Debug)]
pub struct OctSlice<'a>(pub(crate) OctetStr<'a>);

pub enum Unit {
    Wh,
}

impl TryFrom<u8> for Unit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x30 => Ok(Unit::Wh),
            _ => Err(()),
        }
    }
}
