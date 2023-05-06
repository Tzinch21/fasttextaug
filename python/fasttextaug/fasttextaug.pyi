from typing import Optional

from . import rust_fasttextaug as rust_fasttextaug

def get_predict_from_ocr_model(feature: str, filepath: Optional[str] = None) -> str:
    """Дока для обертки над rust функцией"""
