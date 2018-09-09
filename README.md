# larry
treat a file as a l(ine) arr(a)y

Larry is facilitates handling extremely long text files by allowing you to
treat them as an immutable list of lines. Only those lines you access are read.
For unread lines only their initial byte offset is saved, so memory demands are
light regardless of the length of the file. Because only those bytes are decoded
whose lines are accessed, processing demands are light.

Larry scans the file initially for line-terminal byte sequences, considering,
for the time being, only the various combinations of carriage return and newline
that various conventions consider to be line-terminal: `0x0A`, `0x0D`, `0x0A0D`,
and `0x0D0A`. Larry doesn't "watch" its file, so if lines are added, they will
not become accessible.

Larry was inspired by the Perl module [IO::All](https://metacpan.org/source/IO::All).
