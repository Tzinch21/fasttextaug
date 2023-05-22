from typing import List, Union

from fasttextaug.utils import get_lib_abspath
from fasttextaug.rust_fasttextaug import RustKeyboardAugmentor


class KeyboardAug:
    def __init__(
        self,
        aug_char_min=1,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
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

        self._rust_ocr_aug = RustKeyboardAugmentor(
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

    def augment(self, data: Union[List[str], str], n=1, num_thread=1) -> List[str]:
        if isinstance(data, list):
            if num_thread == 1:
                aug_result = self._rust_ocr_aug.substitute_list_single_thread(data)
            else:
                aug_result = self._rust_ocr_aug.substitute_list_multi_thread(data, num_thread)
        else:
            if num_thread == 1:
                aug_result = self._rust_ocr_aug.substitute_string_single_thread(data, n)
            else:
                aug_result = self._rust_ocr_aug.substitute_string_multi_thread(data, n, num_thread)
        return aug_result
