# Dangling comment placement in empty lists
# Regression test for https://github.com/python/cpython/blob/03160630319ca26dcbbad65225da4248e54c45ec/Tools/c-analyzer/c_analyzer/datafiles.py#L14-L16
a1 = [  # a
]
a2 = [  # a
    # b
]
a3 = [
    # b
]
