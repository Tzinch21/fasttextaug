from fasttextaug.rust_fasttextaug import RustRandomWordApiClass

from ..base import BaseAug


class RandomWordAug(BaseAug):
    """
    Augmenter that apply randomly behavior for augmentation.

    :param str action: 'substitute', 'swap' or 'delete'. If value is 'swap', adjacent words will be swapped randomly.
        If value is 'delete', word will be removed randomly.
    :param int aug_min: Minimum number of word will be augmented.
    :param int aug_max: Maximum number of word will be augmented. If None is passed, number of augmentation is
        calculated via aup_p. If calculated result from aug_p is smaller than aug_max, will use calculated result from
        aug_p. Otherwise, using aug_max.
    :param float aug_p: Percentage of word will be augmented.
    :param Set stopwords: Set of words which will be skipped from augment operation
    :param list target_words: Union[List[str], Dict[str, List[str]]]
        - List of word for replacement (used for substitute operation only). Default value is _.
        Each word for augmentation will be substituted by a random one from target_words
        - Dict[str, List[str]]. Each word (key) for augmentation will be substituted
        by a random one from target_words[key]

    >>> import fasttextaug.augmenter.word as faw
    >>> aug = faw.RandomWordAug()
    """

    def __init__(
        self,
        action="delete",
        aug_min=1,
        aug_max=10,
        aug_p=0.3,
        stopwords=None,
        target_words=None,
    ):
        target_vec_words = None
        target_map_words = None
        if isinstance(target_words, list):
            target_vec_words = target_words
        elif isinstance(target_words, dict):
            target_map_words = target_words
        else:
            target_vec_words = ["_"]

        self._rust_aug = RustRandomWordApiClass(
            action=action,
            aug_min_word=aug_min,
            aug_max_word=aug_max,
            aug_p_word=aug_p,
            stopwords=stopwords,
            target_vec_words=target_vec_words,
            target_map_words=target_map_words,
        )

    def get_rust_api_object(self) -> RustRandomWordApiClass:
        return self._rust_aug
