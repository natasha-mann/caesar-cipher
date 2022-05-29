# Caesar Cipher

It was said that Julius Caesar protected his private documents by encrypting them using a cipher. The cypher works by taking each letter of the input text and shifting it by a number of letters. If the shift takes the letter past the end of the alphabet then it moves back to the beginning of the alphabet. In the case of a shift by 2
, w, x , y and z would map to y, z, a, b. Case ought to be preserved, and any non-(A-Z/a-z) characters are left unmodified.

The challenge this week is to encrypt the entirety of Jane Austin's "Pride and Prejudice" as fast as possible (without using any compile-time pre-computation). You can disregard any time taken to read/write the problem and its output to disk.

Rewards:

- 5 Points are awarded for encoding the book in under 500ms
- 2 Further points are awarded for encoding the book in under 100ms
- 2 Further points are awarded for encoding the book in under 50ms

The remaining 1 point is awarded to the fastest submission.

#### Getting Started

```
git clone git@github.com:natasha-mann/caesar-cipher.git
cargo run
```

The shift value is currently set at 2. The value of this can be changed on line 17 of `main.rs`

#### Tests

To test the application run:

```
cargo test
```
