# Book letter counter

Downloads a book from the web in a simple text form and counts each letters occurance in the text.

## Dependencies

Since the programme is written in rust, it's necessary to set up the rust toolchain. It can easily be done with a single command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To create the histogram plot for the numbers it's necesarry to set up `conda` and install the `matplotlib` library.

## Programme execution

To build the programme without running it use:

`cargo build`

To execute the programme use:

`cargo run`

To run the python programme use:

`python src/hist.py`
