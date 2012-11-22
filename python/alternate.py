import vim
import os

def get_alternate(filename):
    """ Get the alternate file name

    """
    dirname, basename = os.path.split(filename)
    if basename.startswith("test_"):
        prod_name = basename[5:]
        prod_dir = os.path.dirname(dirname)
        return os.path.abspath(os.path.join(prod_dir, prod_name))
    else:
        test_dir = os.path.join(dirname, "test")
        if not os.path.exists(test_dir):
            os.makedirs(test_dir)
        test_name = "test_" + basename
        return os.path.abspath(os.path.join(test_dir, test_name))


def get_cur_filename():
    """ Get current file name """
    cur_buffer = vim.current.buffer
    cur_filename = cur_buffer.name
    return cur_filename

def find_or_create_buffer(alternate):
    """ Find or create an alternate buffer """
    # Run ":wincdm w" until current.buffer returns the correct name.
    # If this fails, call :vs to create a new buffer
    num_windows =  vim.eval("winnr('$')")
    num_windows = int(num_windows)
    found = False
    n = num_windows - 1
    for i in range(n):
        vim.command(":wincmd w")
        buffer_name = vim.current.buffer.name
        if buffer_name == alternate:
            found = True
            break
    if not found:
        vim.command(":vs " + alternate)


def alternate():
    """ Main entry point:
    Seach the alternate file.

    If already opened in the same tab, switch to the correct window,
    else call :vs

    """
    cur_filename = get_cur_filename()
    alternate = get_alternate(cur_filename)
    find_or_create_buffer(alternate)

if __name__ == "__main__":
    alternate()

