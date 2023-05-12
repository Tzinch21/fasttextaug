mod keyboard;
mod ocr;

use super::BaseModel;

pub trait CharacterModel: BaseModel {}

pub use keyboard::KeyboardModel;
pub use ocr::OcrModel;
