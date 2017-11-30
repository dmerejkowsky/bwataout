#!/usr/bin/env python

{% if browser %}
from selenium import webdriver
{% endif %}
import subprocess

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
            subprocess.run("{{ cmd }}", shell=True, check=True)
            {% endif %}
            {% if browser %}
            driver.refresh()
            {% endif %}
        except Exception as e:
            print(e)


if __name__ == "__main__":
    main()
