from typing import Set, List, Optional

def augment_by_ocr(
    input_string: str,
    n: int,
    aug_min_char: Optional[int],
    aug_max_char: Optional[int],
    aug_p_char: Optional[float],
    aug_min_word: Optional[int],
    aug_max_word: Optional[int],
    aug_p_word: Optional[float],
    min_chars: Optional[int],
    model_path: str,
    stopwords: Optional[Set[str]],
) -> List[str]:
    """"""

def augment_by_ocr_list(
    input_string: List[str],
    aug_min_char: Optional[int],
    aug_max_char: Optional[int],
    aug_p_char: Optional[float],
    aug_min_word: Optional[int],
    aug_max_word: Optional[int],
    aug_p_word: Optional[float],
    min_chars: Optional[int],
    model_path: str,
    stopwords: Optional[Set[str]],
) -> List[str]:
    """"""
