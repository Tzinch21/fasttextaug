from typing import List, Union

from fasttextaug.rust_fasttextaug import RustRandomCharAugmentor


class RandomCharAug:
    def __init__(
        self,
        action="substitute",
        aug_char_min=1,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
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

        self._rust_aug = RustRandomCharAugmentor(
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

    def augment(self, data: Union[List[str], str], n=1, num_thread=1) -> List[str]:
        if isinstance(data, list):
            if num_thread == 1:
                aug_result = self._rust_aug.augment_list_single_thread(data)
            else:
                aug_result = self._rust_aug.augment_list_multi_thread(data, num_thread)
        else:
            if num_thread == 1:
                aug_result = self._rust_aug.augment_string_single_thread(data, n)
            else:
                aug_result = self._rust_aug.augment_string_multi_thread(data, n, num_thread)
        return aug_result
