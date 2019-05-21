This is a command line tool for converting the output of zero-epwing to the format that Nazeka uses to load non-JMDict dictionaries.

The zero-epwing dictionary must already be manually edited to fix missing definitions (zero-epwing is buggy) and missing characters (because of how epwing works). Failing to do the former will cause the converter to panic, failing to do the latter will cause weird symbolic text to show up when you use the dictionary. The latter issue will move to this converter in the future.

Pre-alpha, not currently for general use. If you want to use this you have to compile it for yourself.

Currently only supports:

- shinmeikai, 5th edition (requires manual editing)
- kenkyuusha's wadai, 5th edition

# Usage

First thing you need to do is convert an epwing dictionary to json with https://github.com/FooSoft/zero-epwing - this looks something like

    ./zero-epwing.exe --entries --pretty shinmeikai/ > my_zeroepwing_shinmeikai_rip.json

Then you use nazeka_epwing_converter on that zeroepwing rip like so:

    ./nazeka_epwing_converter.exe my_zeroepwing_shinmeikai_rip.json > my_nazeka_json_dictionary_shinmeikai.json

Note: This is running from a unix-like shell (git bash) in a weird locale, so it might not work for you. I think that cmd.exe might not support pipes the same way unix shells do? Maybe powershell would work? I don't know.

[nazeka_epwing_converter.zip](https://github.com/wareya/nazeka_epwing_converter/files/3203997/nazeka_epwing_converter.zip)
