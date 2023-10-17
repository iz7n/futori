# futori (太り)

Japanese for "fatten"

Convert half-width characters to full-width

# Usage

First build with `cargo build --release`

Then if you'd like, link the binary into the bin directory to it's available in the path: `ln target/release/futori ~/bin`

Simply pass in a list of file paths to be converted

`futori file1.txt file2.txt ...`

Now each of the files will have half-width characters like ｶﾀｶﾅ replaced with カタカナ
