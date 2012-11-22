import sys

import mock

vim = mock.Mock()
sys.modules["vim"] = vim

from run_pytest import run_pytest

def test_run_current_test_file():
    vim.current.buffer.name = "test/test_foo.py"
    run_pytest()
    assert vim.command.call_args_list == [
            mock.call("!py.test test/test_foo.py"),
            mock.call('let t:pytest_latest_test="test/test_foo.py"')
        ]
    vim.command.reset_mock()
    vim.bindeval.return_value = "test/test_foo.py"
    vim.current.buffer.name = "bar.py"
    run_pytest()
    assert vim.command.call_args_list == [
            mock.call("!py.test test/test_foo.py"),
            mock.call('let t:pytest_latest_test="test/test_foo.py"')
        ]
