pub mod note;

use crate::backend::sql::DBEncoder;

use enum_dispatch::enum_dispatch;
use note::Note;

#[enum_dispatch(SQLEntity)]
pub(crate) enum EntityType {
    Note(Note),
}

#[enum_dispatch]
pub(crate) trait SQLEntity {
    fn name(&self) -> &'static str;
    fn with_encoder<'q>(&'q self, encoder: &mut impl DBEncoder<'q>);
}

// #[enum_dispatch]
// pub(crate) trait SQLEntity {
//     fn with_encoder<'q>(&'q self, encoder: &mut impl DBEncoder<'q>);
// }
