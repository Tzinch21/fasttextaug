# Fasttextaug

It's a lightning-fast version of text submodule in [nlpaug](https://github.com/makcedward/nlpaug/tree/master) library written in Rust.

This library helps you with augmenting nlp for your machine learning projects.

## Features:
- Generate synthetic data for improving model performance
- Simple, easy-to-use and lightweight library.
- Easy-to-switch, same API, classes & parameteres
- Up to 80 times faster than original pure python version

## Avaliable textual augmentors:
| Target | Augmenter | Action | Description |
|:---:|:---:|:---:|:---:|
| Character | KeyboardAug | substitute | Simulate keyboard distance error |
| Character | OcrAug | substitute | Simulate OCR engine error |
| Character | RandomCharAug | insert, substitute, swap, delete | Apply augmentation randomly |
| Word | RandomWordAug | swap, substitute, delete | Apply augmentation randomly |

## Installation
The library supports python 3.8+ in linux, macos and windows platform.

To install the library:
```
pip install fasttextaug
```

## References
This library, based on the idea originated from the freely distributed [nlpaug](https://github.com/makcedward/nlpaug/tree/master). Please also explore the original library, and support their work.

Also like to extend thanks to the [Maturin](https://github.com/PyO3/maturin) project, which enables easy development of Python packages using Rust.

## License
This software is available under the following licenses:
- MIT
