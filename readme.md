This is a command line tool for converting the output of zero-epwing to the format that Nazeka uses to load non-JMDict dictionaries.

The zero-epwing dictionary must already be manually edited to fix missing definitions (zero-epwing is buggy) and missing characters (because of how epwing works). Failing to do the former will cause the converter to panic, failing to do the latter will cause weird symbolic text to show up when you use the dictionary.

Pre-alpha, not currently for general use. If you want to use this you have to compile it for yourself.

Currently only supports shinmeikai, 5th edition.

Invoke as:

	<executable> <filename> > <desired json dict filename>.json

from a unix-like shell.
