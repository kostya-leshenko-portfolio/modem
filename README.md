##### Kostya Leshenko


### Modem
I implemented the basic decoder for the wav files provided (48kHz, 160 samples per bit, etc.). This task wasn't particularly difficult, the assinment explained everything, I just needed to write the code. I didn't implement any of the extras.


#### Building
`cargo build` or `cargo build --release`

#### Running
`cargo run`, `cargo run --release` will try to read klechtch.wav from the working directory.

Alternatively, you can pass the name of the file you want to decode to the program:
`cargo run -- my_file.wav`
