# Instructions

Compile and run the prolog server:

```bash
swipl -g true -O -o solution.srv -c prolog/server.pl
./solution.srv
# kill when finished
```

Run the rust program to query the solutions:

```bash
cargo run
```
