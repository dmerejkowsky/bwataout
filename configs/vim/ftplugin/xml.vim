" Special settings for XML files

" To use quickfix with XML files:
if executable("xmllint")
  setlocal makeprg=xmllint\ %\ --noout
endif
