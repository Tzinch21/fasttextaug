import json
import timeit
import itertools
from functools import partial

import pandas as pd
import nlpaug.augmenter.char as nac

import fasttextaug as fau

data = pd.read_csv("/app/data/Reviews.csv")
REVIEWS = data.iloc[:, -1].to_list()


if __name__ == "__main__":
    bench_result = {}
    REPEATS = 3
    NUMBER_MY_LIB = 7
    NUMBER_THEIR_LIB = 1

    for flags in itertools.product([True, False], repeat=3):
        str_flags = str(flags)
        bench_result[str_flags] = {}
        special_char, numeric, uppercase = flags

        my_lib_aug = fau.character.KeyboardAug(
            include_special_char=special_char, include_numeric=numeric, include_upper_case=uppercase
        )
        exist_lib_aug = nac.KeyboardAug(
            include_special_char=special_char, include_numeric=numeric, include_upper_case=uppercase
        )

        for num_thread in (1, 2, 4, 8):
            bench_result[str_flags][num_thread] = {}
            my_bench = timeit.repeat(
                partial(my_lib_aug.augment, data=REVIEWS, num_thread=num_thread),
                repeat=REPEATS,
                number=NUMBER_MY_LIB,
            )
            bench_result[str_flags][num_thread]["fasttextaug"] = [i / NUMBER_MY_LIB for i in my_bench]
            bench_result[str_flags][num_thread]["fasttextaug_number"] = NUMBER_MY_LIB
            bench_result[str_flags][num_thread]["fasttextaug_repeats"] = REPEATS

            if num_thread == 1:
                lib_bench = timeit.repeat(
                    partial(exist_lib_aug.augment, data=REVIEWS, num_thread=num_thread),
                    repeat=REPEATS,
                    number=NUMBER_THEIR_LIB,
                )
                bench_result[str_flags][num_thread]["nlpaug"] = [
                    i / NUMBER_THEIR_LIB for i in lib_bench
                ]
                bench_result[str_flags][num_thread]["nlpaug_number"] = NUMBER_THEIR_LIB
                bench_result[str_flags][num_thread]["nlpaug_repeats"] = REPEATS

            print(f"Did keyboard_list - {num_thread}")

    with open("/reports/keyboard/keyboard_list.json", "w") as file:
        print(json.dumps(bench_result), file=file)
