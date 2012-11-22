import os
import vim


def run_pytest():
    """ Main entry point

    If current file is a test file, run it and store its name,
    Otherwise, run the latest stored test.

    """
    cur_filename = vim.current.buffer.name
    if not cur_filename:
        return
    basename = os.path.basename(cur_filename)
    if basename.startswith("test"):
        to_run = cur_filename
    else:
        to_run = vim.bindeval("t:pytest_latest_test")
        if not to_run:
            return
    vim.command("!py.test %s" % to_run)
    vim.command('let t:pytest_latest_test="%s"' % to_run)

if __name__ == "__main__":
    run_pytest()
