import json
import timeit
import itertools
from functools import partial

import nlpaug.augmenter.char as nac

import fasttextaug as fau

INPUT_TEXT_SINGLE_STR = (
    "This is the best tasting stevia powder I've tried. "
    "Most other powders have a weird after taste but this one doesn't. "
    "I recently bought a 5lb bag of it online and I'm making my way through it."
)


if __name__ == "__main__":
    bench_result = {}
    REPEATS = 3
    number_my_lib = (100_000, 100_000, 10_000, 1000, 100)
    number_exist_lib = (10_000, 1000, 100, 100, 100)

    for flags in itertools.product([True, False], repeat=3):
        bench_result[flags] = {}
        special_char, numeric, uppercase = flags

        for idx, n_size in enumerate((1, 10, 100, 1_000, 10_000)):
            bench_result[flags][n_size] = {}
            bench_result[flags][n_size]["n_size"] = n_size

            my_lib_aug = fau.character.KeyboardAug(
                include_special_char=special_char,
                include_numeric=numeric,
                include_upper_case=uppercase,
            )
            exist_lib_aug = nac.KeyboardAug(
                include_special_char=special_char,
                include_numeric=numeric,
                include_upper_case=uppercase,
            )

            my_bench = timeit.repeat(
                partial(my_lib_aug.augment, data=INPUT_TEXT_SINGLE_STR, n=n_size),
                repeat=REPEATS,
                number=number_my_lib[idx],
            )
            lib_bench = timeit.repeat(
                partial(exist_lib_aug.augment, data=INPUT_TEXT_SINGLE_STR, n=n_size),
                repeat=REPEATS,
                number=number_exist_lib[idx],
            )

            bench_result[flags][n_size]["fasttextaug"] = [i / number_my_lib[idx] for i in my_bench]
            bench_result[flags][n_size]["fasttextaug_repeats"] = REPEATS
            bench_result[flags][n_size]["fasttextaug_number"] = number_my_lib[idx]

            bench_result[flags][n_size]["nlpaug"] = [i / number_exist_lib[idx] for i in lib_bench]
            bench_result[flags][n_size]["nlpaug_repeats"] = REPEATS
            bench_result[flags][n_size]["nlpaug_number"] = number_exist_lib[idx]

    with open("/reports/keyboard/keyboard_single.json", "w") as file:
        print(json.dumps(bench_result), file=file)
