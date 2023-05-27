use super::super::BaseAugmentor;
use crate::model::word::WordModel;

pub trait WordAugmentor<T>: BaseAugmentor<T>
where
    T: WordModel,
{
}
