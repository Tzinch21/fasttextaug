mod keyboard;
mod ocr;
mod random;

use super::BaseModel;

pub trait CharacterModel: BaseModel {}

pub use keyboard::KeyboardModel;
pub use ocr::OcrModel;
pub use random::RandomCharModel;
