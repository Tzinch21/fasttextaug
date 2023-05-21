import perftester
import nlpaug.augmenter.char as nac

import fasttextaug as fau

INPUT_TEXT = (
    "This is the best tasting stevia powder I've tried. "
    "Most other powders have a weird after taste but this one doesn't. "
    "I recently bought a 5lb bag of it online and I'm making my way through it."
)
perftester.config.set_defaults("time", Number=100000, Repeat=1)

if __name__ == "__main__":
    my_lib_ocr_aug = fau.character.OcrAug()
    exist_lib_ocr_aug = nac.OcrAug()

    my_t = perftester.time_benchmark(my_lib_ocr_aug.augment, data=INPUT_TEXT, n=1)

    lib_t = perftester.time_benchmark(exist_lib_ocr_aug.augment, data=INPUT_TEXT, n=1)
    perftester.pp(my_t)
    perftester.pp(lib_t)
