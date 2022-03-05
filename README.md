# Richelieu

Rust-based implementation of the drunken bishop algorithm for fingerprint visualization

## Getting started

The only required option is `--data` (or `-d`), containing data to encode. Run with the `--help` option to see other flags. An example run might look like this:

    $ richelieu --data 'Hello, world!'
    +-----------------+
    |       oE ..     |
    |    . oo*.o      |
    |     o.= B       |
    |     . .o X      |
    |      . SB       |
    |                 |
    |                 |
    |                 |
    |                 |
    +-----------------+

Note that the program does not currently include any hashing. Consider hashing data before feeding it to the drunken bishop.

## Links

- <https://www.jfurness.uk/the-drunken-bishop-algorithm/>
- <https://pthree.org/2013/05/30/openssh-keys-and-the-drunken-bishop/>

## TODO

- Ensure this works larger or more complex messages. Compare against <https://www.jfurness.uk/the-drunken-bishop-algorithm/>
- If we include hashing, we can print the hash algo at the top of the output frame.
- A fun variant might be a "wrap-around" board that avoids clipping to the edges.
