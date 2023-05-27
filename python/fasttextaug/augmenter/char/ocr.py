from fasttextaug.utils import get_lib_abspath
from fasttextaug.rust_fasttextaug import RustOCRApiClass

from ..base import BaseAug


class OcrAug(BaseAug):
    """
    Augmenter that simulate ocr error by random values. For example, OCR may recognize I as 1 incorrectly.\
        Pre-defined OCR mapping is leveraged to replace character by possible OCR error.

    :param int aug_char_min: Minimum number of character will be augmented.
    :param int aug_char_max: Maximum number of character will be augmented. If None is passed, number of augmentation is
        calculated via aup_char_p. If calculated result from aug_char_p is smaller than aug_char_max, will use calculated result
        from aup_char_p. Otherwise, using aug_max.
    :param float aug_char_p: Percentage of character (per token) will be augmented.
    :param int aug_word_min: Minimum number of word will be augmented.
    :param int aug_word_max: Maximum number of word will be augmented. If None is passed, number of augmentation is
        calculated via aup_word_p. If calculated result from aug_word_p is smaller than aug_word_max, will use calculated result
        from aug_word_p. Otherwise, using aug_max.
    :param float aug_word_p: Percentage of word will be augmented.
    :param int min_char: If word less than this value, do not draw word for augmentation
    :param set stopwords: Set of words which will be skipped from augment operation.
    :param obj dict_of_path: Use pre-defined dictionary by default. Pass either file path of dict to use custom mapping.
    :param str lang: Indicate built-in language model. Default value is 'en'. Possible values are 'en', 'ru' (Russian).
        If custom model is used (passing model_path), this value will be ignored.

    >>> import fasttextaug.augmenter.char as fac
    >>> aug = fac.OcrAug()
    """

    def __init__(
        self,
        aug_char_min=2,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
        aug_word_p=0.3,
        min_char=1,
        stopwords=None,
        dict_of_path=None,
        lang=None,
    ):
        if dict_of_path is None:
            dir_path = get_lib_abspath() + "/res/ocr"
            lang = "en" if lang is None else lang
            dict_of_path = f"{dir_path}/{lang}.json"

        self._rust_aug = RustOCRApiClass(
            aug_min_char=aug_char_min,
            aug_max_char=aug_char_max,
            aug_p_char=aug_char_p,
            aug_min_word=aug_word_min,
            aug_max_word=aug_word_max,
            aug_p_word=aug_word_p,
            stopwords=stopwords,
            min_char=min_char,
            dict_of_path=dict_of_path,
        )

    def get_rust_api_object(self) -> RustOCRApiClass:
        return self._rust_aug
