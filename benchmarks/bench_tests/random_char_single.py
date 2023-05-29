import json
import timeit
from functools import partial

import nlpaug.augmenter.char as nac

import fasttextaug.augmenter.char as fac

INPUT_TEXT_SINGLE_STR = (
    "This is the best tasting stevia powder I've tried. "
    "Most other powders have a weird after taste but this one doesn't. "
    "I recently bought a 5lb bag of it online and I'm making my way through it."
)

if __name__ == "__main__":
    bench_result = {}
    REPEATS = 3
    number_my_lib = (10_000, 10_000, 10_000, 10_000, 100_00, 1000, 100, 100)
    number_exist_lib = (1000, 1000, 1000, 1000, 100, 100, 10, 10)

    for action in ["insert", "substitute", "swap", "delete"]:
        bench_result[action] = {}

        for idx, n_size in enumerate((1, 2, 3, 5, 10, 100, 1_000, 10_000)):
            bench_result[action][n_size] = {}
            bench_result[action][n_size]["n_size"] = n_size

            my_lib_aug = fac.RandomCharAug(action=action)
            exist_lib_aug = nac.RandomCharAug(action=action)

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

            bench_result[action][n_size]["fasttextaug"] = [i / number_my_lib[idx] for i in my_bench]
            bench_result[action][n_size]["fasttextaug_repeats"] = REPEATS
            bench_result[action][n_size]["fasttextaug_number"] = number_my_lib[idx]

            bench_result[action][n_size]["nlpaug"] = [i / number_exist_lib[idx] for i in lib_bench]
            bench_result[action][n_size]["nlpaug_repeats"] = REPEATS
            bench_result[action][n_size]["nlpaug_number"] = number_exist_lib[idx]

    with open("/reports/random_char/random_char_single.json", "w") as file:
        print(json.dumps(bench_result), file=file)
