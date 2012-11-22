import sys

import mock

# Monkey-patch vim and some os.path functions
vim_module = mock.Mock()
sys.modules["vim"] = vim_module
import os
os.path.exists = mock.Mock()
os.makedirs = mock.Mock()

from alternate import *

def test_prod_to_test_exists():
    os.path.exists.return_value = True
    assert get_alternate("/path/to/foo.py") == "/path/to/test/test_foo.py"
    assert os.makedirs.called is False

def test_prod_to_test_does_no_exist():
    os.path.exists.return_value = False
    assert get_alternate("/path/to/foo.py") == "/path/to/test/test_foo.py"
    assert os.makedirs.call_args == mock.call("/path/to/test")

def test_test_to_prod():
    assert get_alternate("/path/to/test/test_foo.py") == "/path/to/foo.py"

def test_get_cur_filename():
    vim.current.buffer.name = "foo.py"
    assert get_cur_filename() == "foo.py"

def test_create_new_buffer_if_not_present():
    set_current_buffer_list(["foo.py"])
    find_or_create_buffer("test/test_foo.py")
    assert vim.cmds == [":vs test/test_foo.py"]

def test_swith_to_other_window():
    set_current_buffer_list(["test/test_foo.py", "foo.py"])
    find_or_create_buffer("test/test_foo.py")
    # Shoud have called :wincmd w exactly once:
    assert vim.cmds == [":wincmd w"]

def test_search_in_every_window():
    set_current_buffer_list(["bar.py", "test/test_foo.py", "foo.py"])
    find_or_create_buffer("test/test_foo.py")
    # Shoud have called :wincmd w exactly twice
    assert vim.cmds == [":wincmd w", ":wincmd w"]

def set_current_buffer_list(buff_names):
    """ Set the list of what the successive calls to
    vim.current.buffer will return

    """
    buffers = list()
    for buff_name in buff_names:
        buff = mock.Mock()
        buff.name = buff_name
        buffers.append(buff)
    vim.current = mock.MagicMock()
    buffer_mock = mock.PropertyMock(side_effect=buffers)
    type(vim.current).buffer = buffer_mock
    vim.evals = list()
    vim.cmds = list()

    def fake_eval(e):
        vim.evals.append(e)
        if e == "winnr('$')":
            return str(len(buff_names))
        return ""

    def fake_command(cmd):
        vim.cmds.append(cmd)

    vim.eval.side_effect = fake_eval
    vim.command.side_effect = fake_command

def test_set_current_buffer_list():
    set_current_buffer_list(["a", "b"])
    assert vim.current.buffer.name == "a"
    assert vim.current.buffer.name == "b"
    assert vim.eval("winnr('$')") == "2"
    vim.eval("somme_eval()")
    vim.command(":a command")
    assert vim.cmds == [":a command"]
    assert vim.evals == ["winnr('$')", "somme_eval()"]
