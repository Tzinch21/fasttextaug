from typing import Dict, List, Tuple

def create_ocr_from_mapping_and_get_predict(
    mapping: Dict[str, List[str]], feature: str
) -> List[str]:
    """
    Создать OCR-модель из переданного mapping
    Затем вернуть предикт для feature
    """

def create_ocr_and_get_stats(filepath: str) -> Tuple[int, int, List[Tuple[int, int]]]:
    """Получить статы созданной OCR-модели по переданному пути"""

def create_ocr_and_get_predict(filepath: str, feature: str) -> List[str]:
    """Создать OCR модель по пути и получить предикт для feature"""

def create_keyboard_and_get_predict(
    allow_special_char: bool, allow_numeric: bool, upper_case: bool, model_path: str, feature: str
) -> List[str]:
    """Создать keyboard-модель по файлу и получить предикт по фичам
    allow_special_char - разрешить спец символы
    allow_numeric - разрешить цифры
    upper_case - использовать капс
    """

def create_keyboard_and_get_stats(
    allow_special_char: bool, allow_numeric: bool, upper_case: bool, model_path: str, feature: str
) -> Tuple[int, int, List[Tuple[int, int]]]:
    """Создать keyboard-модель по файлу и получить статы
    allow_special_char - разрешить спец символы
    allow_numeric - разрешить цифры
    upper_case - использовать капс
    """
