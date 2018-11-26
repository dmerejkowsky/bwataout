if !exists('loaded_snippet') || &cp
    finish
endif

Snippet ifn if __name__ == "__main__":<CR><{}>

Snippet pdb import pdb; pdb.set_trace()
Snippet ipdb import ipdb; ipdb.set_trace()
Snippet pylint # pylint: disable=<{}>
