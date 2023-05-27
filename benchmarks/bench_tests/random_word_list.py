import json
import timeit
from functools import partial

import pandas as pd
import nlpaug.augmenter.word as naw

import fasttextaug.augmenter.word as faw

data = pd.read_csv("/app/data/Reviews.csv")
REVIEWS = data.iloc[:, -1].to_list()


if __name__ == "__main__":
    bench_result = {}
    REPEATS = 3
    NUMBER_MY_LIB = 7
    NUMBER_THEIR_LIB = 3

    for action in ["substitute", "swap", "delete"]:
        bench_result[action] = {}

        my_lib_aug = faw.RandomWordAug(action=action)
        exist_lib_aug = naw.RandomWordAug(action=action)

        for num_thread in (1, 2, 4, 8):
            bench_result[action][num_thread] = {}
            my_bench = timeit.repeat(
                partial(my_lib_aug.augment, data=REVIEWS, num_thread=num_thread),
                repeat=REPEATS,
                number=NUMBER_MY_LIB,
            )
            bench_result[action][num_thread]["fasttextaug"] = [i / NUMBER_MY_LIB for i in my_bench]
            bench_result[action][num_thread]["fasttextaug_number"] = NUMBER_MY_LIB
            bench_result[action][num_thread]["fasttextaug_repeats"] = REPEATS

            if num_thread == 1:
                lib_bench = timeit.repeat(
                    partial(exist_lib_aug.augment, data=REVIEWS, num_thread=num_thread),
                    repeat=REPEATS,
                    number=NUMBER_THEIR_LIB,
                )
                bench_result[action][num_thread]["nlpaug"] = [
                    i / NUMBER_THEIR_LIB for i in lib_bench
                ]
                bench_result[action][num_thread]["nlpaug_number"] = NUMBER_THEIR_LIB
                bench_result[action][num_thread]["nlpaug_repeats"] = REPEATS

    with open("/reports/random_word/random_word_list.json", "w") as file:
        print(json.dumps(bench_result), file=file)
