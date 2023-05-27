mod random;

use super::BaseModel;

pub trait WordModel: BaseModel {}

pub use random::RandomWordModel;
