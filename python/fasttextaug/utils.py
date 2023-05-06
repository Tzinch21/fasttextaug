import os


def get_lib_abspath() -> str:
    """Useful to get full path for library"""
    return os.path.dirname(os.path.abspath(__file__))
