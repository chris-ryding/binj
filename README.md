# binj

Binary INJection - a command-line utility to modify data in binary files

`binj` (pronounced "binge") is a utility to add, delete, and modify data in a binary file or data stream. It reads from either a given file or standard in and writes the modified data to a designated output file or standard out.

## Command syntax

The basic command format is `command@offset,data`. Commands are a single character code, offset is the number of bytes between the start of the file and where the command takes effect, and the data is the byte stream to be inserted or which will replace existing content, or the number of bytes to delete.

Valid commands are `insert` 'I', `delete` 'D', and `replace` 'R'.

Both offsets and data bytes formats follow typical programming syntax:

If the number starts with a 0 and only contains digits 0-7, it is interpreted as an octal value. If the number starts with a "0x" sequence, it is interpreted as a hex value. All others are assumed to be decimal values.

The default behavior is to read a binary stream from standard input `stdin` and write the modified stream to standard output `stdout`. Alternatively, input and output files may be designated with the `-i` and `-o` options.

Ex:

    binj -i in.dat -o out.dat I@123,0x1a2b

### Insert

Adds data to the stream starting at the given offset. The data originally at that location is apended after the added bytes

Syntax:

    I@offset,data

Ex:

    Starting file data: 00 01 02 03 04 05 06 07

    binj I@2,0x1234 < input_data

    Result: 00 01 12 34 02 03 04 05 06 07

### Delete

Deletes the indicated number of bytes from the stream starting at the indicated offset.

Syntax:

    D@offset,count

Ex:

    Starting file data: 00 10 20 30 40 50 60 70 80 A0 B0

    binj D@4,2 < input_data

    Result: 00 10 20 30 60 70 80 A0 B0

### Replace

Replaces a series of bytes with the given data starting at the indicated offset.

Syntax:

    R@offset,data

Ex:

    Starting file data: 00 10 20 30 40 50 60 70 80 A0 B0

    binj -i in.dat R@0,0xfedc12340011

    Result: FE DC 12 34 00 11 60 70 80 A0 B0
