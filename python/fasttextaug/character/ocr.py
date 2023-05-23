from typing import List, Union

from fasttextaug.utils import get_lib_abspath
from fasttextaug.rust_fasttextaug import RustOCRAugmentor


class OcrAug:
    def __init__(
        self,
        aug_char_min=2,
        aug_char_max=10,
        aug_char_p=0.3,
        aug_word_p=0.3,
        aug_word_min=1,
        aug_word_max=10,
        stopwords=None,
        min_char=1,
        dict_of_path=None,
        lang=None,
    ):
        if dict_of_path is None:
            dir_path = get_lib_abspath() + "/res/ocr"
            lang = "en" if lang is None else lang
            dict_of_path = f"{dir_path}/{lang}.json"

        self._rust_aug = RustOCRAugmentor(
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
