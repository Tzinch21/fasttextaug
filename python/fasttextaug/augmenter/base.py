from typing import List, Union


class BaseAug:
    def get_rust_api_object(self):
        raise NotImplemented

    def augment(self, data: Union[List[str], str], n=1, num_thread=1) -> List[str]:
        if isinstance(data, list):
            if num_thread == 1:
                aug_result = self.get_rust_api_object().augment_list_single_thread(data)
            else:
                aug_result = self.get_rust_api_object().augment_list_multi_thread(data, num_thread)
        else:
            if num_thread == 1:
                aug_result = self.get_rust_api_object().augment_string_single_thread(data, n)
            else:
                aug_result = self.get_rust_api_object().augment_string_multi_thread(
                    data, n, num_thread
                )
        return aug_result
