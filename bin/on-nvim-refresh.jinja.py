#!/usr/bin/env python

import subprocess
import sys

{% if browser %}
from selenium import webdriver
{% endif %}
import neovim


def main():
    vim = neovim.attach("socket", path="/tmp/neovim")
    vim.subscribe("refresh")
    vim.command('nnoremap ,m :w<cr>:call rpcnotify(0, "refresh")<cr>')
    print("Ready to roll")
    {% if browser %}
    url = "{{ url }}"
    {% if browser == "chrome" %}
    driver = webdriver.Chrome()
    {% else %}
    driver = webdriver.Firefox()
    {% endif %}
    driver.get(url)
    {% endif %}
    while True:
        try:
            vim.next_message()
            {% if cmd %}
            print("{{ cmd }}")
            process = subprocess.run("{{ cmd }}", shell=True)
            if process.returncode != 0:
                print("Process failed")
            {% endif %}
            {% if browser %}
            driver.refresh()
            {% endif %}
        except neovim.api.NvimError:
            sys.exit(e)
        except OSError as e:
            if e.args[0] == "EOF":
                sys.exit("Neovim connection lost")


if __name__ == "__main__":
    main()
