from fasttextaug.rust_fasttextaug import RustRandomCharApiClass

from ..base import BaseAug


class RandomCharAug(BaseAug):
    """
    Augmenter that generate character error by random values. For example, people may type i as o incorrectly.

    :param str action: Possible values are 'insert', 'substitute', 'swap' and 'delete'. If value is 'insert', a new
        character will be injected to randomly. If value is 'substitute', a random character will be replaced
        original character randomly. If value is 'swap', adjacent characters within sample word will be swapped
        randomly. If value is 'delete', character will be removed randomly.
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
    :param bool include_upper_case: If True, upper case character may be included in augmented data. If `candidates'
        value is provided, this param will be ignored.
    :param bool include_lower_case: If True, lower case character may be included in augmented data. If `candidates'
        value is provided, this param will be ignored.
    :param bool include_numeric: If True, numeric character may be included in augmented data. If `candidates'
        value is provided, this param will be ignored.
    :param bool include_special_char: If True, special character may be included in augmented data. If `candidates'
        value is provided, this param will be ignored.
    :param int min_char: If word less than this value, do not draw word for augmentation
    :param swap_mode: When action is 'swap', you may pass 'adjacent', 'middle' or 'random'. 'adjacent' means swap action
        only consider adjacent character (within same word). 'middle' means swap action consider adjacent character but
        not the first and last character of word. 'random' means swap action will be executed without constraint.
    :param str spec_char: Special character may be included in augmented data. If `candidates'
        value is provided, this param will be ignored.
    :param set stopwords: Set of words which will be skipped from augment operation.
    :param List candidates: List of string for augmentation. E.g. ['AAA', '11', '===']. If values is provided,
        `include_upper_case`, `include_lower_case`, `include_numeric` and `spec_char` will be ignored.
    :param str lang: Indicate built-in set of chars (uppercase / lowercase). Default value is 'en'. Possible values are 'en', 'ru' (Russian).
        If `candidates' value is provided, this param will be ignored.

    >>> import fasttextaug.augmenter.char as fac
    >>> aug = fac.RandomCharAug()
    """

    def __init__(
        self,
        action="substitute",
        aug_char_min=1,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
        aug_word_p=0.3,
        include_upper_case=True,
        include_lower_case=True,
        include_numeric=True,
        include_special_char=True,
        min_char=4,
        swap_mode="adjacent",
        spec_char="!@#$%^&*()_+",
        stopwords=None,
        candidates=None,
        lang=None,
    ):
        if lang is None:
            lang = "en"

        self._rust_aug = RustRandomCharApiClass(
            action=action,
            aug_min_char=aug_char_min,
            aug_max_char=aug_char_max,
            aug_p_char=aug_char_p,
            aug_min_word=aug_word_min,
            aug_max_word=aug_word_max,
            aug_p_word=aug_word_p,
            include_upper_case=include_upper_case,
            include_lower_case=include_lower_case,
            include_numeric=include_numeric,
            include_special_char=include_special_char,
            lang=lang,
            stopwords=stopwords,
            min_char=min_char,
            swap_mode=swap_mode,
            spec_char=spec_char,
            candidates=candidates,
        )

    def get_rust_api_object(self) -> RustRandomCharApiClass:
        return self._rust_aug
