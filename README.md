# Pychmrs: PyO3 Wrapper for the libchm (rust) Library

## Why?

All the churn in the Python build ecosystem broke most of the existing chm reading libraries, and it was easy enough to
make this wrapper around this rust library.

## Overview
```python
# Example usage of the PyChmRS library
import pychmrs
chm_file  = pychmrs.ChmFileWrapper('cool.chm')
print(chm_file.list_paths())
# [... , '/020161622x_cnode1.html',... ]
byte_string = chm_file.read('/chapter01.html')

```

## Installation
```shell
uv venv
uv pip install .
```
