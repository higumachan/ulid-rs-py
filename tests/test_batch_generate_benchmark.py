import time

import pytest
import ulid_rs_py
import ulid

def batch_generate_rs():
    return ulid_rs_py.batch_new(1000)

#@pytest.mark.skip()
def test_batch_generate_ulid_rs_py_benchmark(benchmark):
    ret = benchmark(batch_generate_rs)
    assert ret


def batch_generate_py():
    return [ulid.new() for i in range(1000)]

#@pytest.mark.skip()
def test_batch_generate_ulid_py_benchmark(benchmark):
    ret = benchmark(batch_generate_py)
    assert ret
