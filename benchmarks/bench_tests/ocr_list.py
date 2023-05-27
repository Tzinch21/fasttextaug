import json
import timeit
from functools import partial

import pandas as pd
import nlpaug.augmenter.char as nac

import fasttextaug.augmenter.char as fac

data = pd.read_csv("/app/data/Reviews.csv")
REVIEWS = data.iloc[:, -1].to_list()


if __name__ == "__main__":
    bench_result = {}
    REPEATS = 5
    NUMBER_MY_LIB = 10
    NUMBER_THEIR_LIB = 1

    my_lib_ocr_aug = fac.OcrAug()
    exist_lib_ocr_aug = nac.OcrAug()

    for num_thread in (1, 2, 4, 8):
        bench_result[num_thread] = {}
        my_bench = timeit.repeat(
            partial(my_lib_ocr_aug.augment, data=REVIEWS, num_thread=num_thread),
            repeat=REPEATS,
            number=NUMBER_MY_LIB,
        )
        bench_result[num_thread]["fasttextaug"] = [i / NUMBER_MY_LIB for i in my_bench]
        bench_result[num_thread]["fasttextaug_number"] = NUMBER_MY_LIB
        bench_result[num_thread]["fasttextaug_repeats"] = REPEATS

        if num_thread == 1:
            lib_bench = timeit.repeat(
                partial(exist_lib_ocr_aug.augment, data=REVIEWS, num_thread=num_thread),
                repeat=REPEATS,
                number=NUMBER_THEIR_LIB,
            )
            bench_result[num_thread]["nlpaug"] = [i / NUMBER_THEIR_LIB for i in lib_bench]
            bench_result[num_thread]["nlpaug_number"] = NUMBER_THEIR_LIB
            bench_result[num_thread]["nlpaug_repeats"] = REPEATS

    with open("/reports/ocr/ocr_list.json", "w") as file:
        print(json.dumps(bench_result), file=file)
