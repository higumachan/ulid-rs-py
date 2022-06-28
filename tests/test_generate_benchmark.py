import time

import fast_ulid
import pytest
import ulid_rs_py
import ulid


def test_generate_ulid_rs_py_benchmark(benchmark):
    ret = benchmark(ulid_rs_py.new)
    assert ret

def test_generate_ulid_py_benchmark(benchmark):
    ret = benchmark(ulid.new)
    assert ret

def test_generate_fast_ulid_py_benchmark(benchmark):
    ret = benchmark(fast_ulid.ulid)
    assert ret