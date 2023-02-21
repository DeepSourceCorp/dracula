# pylint:disable=pointless-string-statement, redundant-u-string-prefix
"""Test for backslash escapes in byte vs unicode strings"""

# Would be valid in Unicode, but probably not what you want otherwise
BAD_UNICODE = b'\u0042'  # [anomalous-unicode-escape-in-string]
BAD_LONG_UNICODE = b'\U00000042'  # [anomalous-unicode-escape-in-string]
# +1:[anomalous-unicode-escape-in-string]
BAD_NAMED_UNICODE = b'\N{GREEK SMALL LETTER ALPHA}'

GOOD_UNICODE = u'\u0042'
GOOD_LONG_UNICODE = u'\U00000042'
GOOD_NAMED_UNICODE = u'\N{GREEK SMALL LETTER ALPHA}'


# Valid raw strings
RAW_BACKSLASHES = r'raw'

# In a comment you can have whatever you want: \ \\ \n \m
# even things that look like bad strings: "C:\Program Files"


"""Test for anomalous backslash escapes in strings"""

BAD_ESCAPE = '\z'  # [anomalous-backslash-in-string]
BAD_ESCAPE_NOT_FIRST = 'abc\z'  # [anomalous-backslash-in-string]
BAD_ESCAPE_WITH_PREFIX = b'abc\z'  # [anomalous-backslash-in-string]
BAD_ESCAPE_WITH_BACKSLASH = b'a\
    \z'  # [anomalous-backslash-in-string]
# +3:[anomalous-backslash-in-string]
BAD_ESCAPE_BLOCK = b'''
    abc
    \z
'''
BAD_ESCAPE_PARENS = (b'abc'
                     b'\z')  # [anomalous-backslash-in-string]
GOOD_ESCAPE = '\b'

# Valid raw strings
BAD_ESCAPE_BUT_RAW = r'\z'

# In a comment you can have whatever you want: \z