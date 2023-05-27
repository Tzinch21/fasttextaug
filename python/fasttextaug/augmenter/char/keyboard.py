from fasttextaug.utils import get_lib_abspath
from fasttextaug.rust_fasttextaug import RustKeyboardApiClass

from ..base import BaseAug


class KeyboardAug(BaseAug):
    """
    Augmenter that simulate typo error by random values. For example, people may type i as o incorrectly.\
        One keyboard distance is leveraged to replace character by possible keyboard error.

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
    :param set stopwords: Set of words which will be skipped from augment operation.
    :param bool include_special_char: Include special character
    :param bool include_numeric: If True, numeric character may be included in augmented data.
    :param bool include_upper_case: If True, upper case character may be included in augmented data.
    :param int min_char: If word less than this value, do not draw word for augmentation
    :param str model_path: Loading customize model from file system
    :param str lang: Indicate built-in language model. Default value is 'en'. Possible values are 'en', 'ru' (Russian).
        If custom model is used (passing model_path), this value will be ignored.

    >>> import fasttextaug.augmenter.char as fac
    >>> aug = fac.KeyboardAug()
    """

    def __init__(
        self,
        aug_char_min=1,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
        aug_word_p=0.3,
        stopwords=None,
        include_special_char=True,
        include_numeric=True,
        include_upper_case=True,
        min_char=4,
        model_path=None,
        lang=None,
    ):
        if model_path is None:
            dir_path = get_lib_abspath() + "/res/keyboard"
            lang = "en" if lang is None else lang
            model_path = f"{dir_path}/{lang}.json"

        self._rust_aug = RustKeyboardApiClass(
            aug_min_char=aug_char_min,
            aug_max_char=aug_char_max,
            aug_p_char=aug_char_p,
            aug_min_word=aug_word_min,
            aug_max_word=aug_word_max,
            aug_p_word=aug_word_p,
            stopwords=stopwords,
            include_special_char=include_special_char,
            include_numeric=include_numeric,
            include_upper_case=include_upper_case,
            min_char=min_char,
            model_path=model_path,
        )

    def get_rust_api_object(self) -> RustKeyboardApiClass:
        return self._rust_aug
