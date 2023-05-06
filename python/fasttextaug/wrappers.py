import os
from typing import Optional

from fasttextaug import rust_fasttextaug as rust_ft

from .utils import get_lib_abspath


def get_predict_from_ocr_model(feature: str, filepath: Optional[str] = None) -> str:
    if filepath:
        return rust_ft.get_predict_from_ocr_model(feature, filepath)
    default_path = os.path.join(get_lib_abspath(), "res/ru.json")
    return rust_ft.get_predict_from_ocr_model(feature, default_path)
