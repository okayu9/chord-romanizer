# StrictV1 chord formulas

StrictV1 separates the base quality, seventh quality, and explicitly added,
altered, or omitted degrees. Original quality text is preserved for rendering.
The Python019 formula path retains its compatibility behavior.

The following cases have dedicated formula and degree-identity regressions:

| Notation | StrictV1 formula | Related distinction |
| --- | --- | --- |
| Caug7, C+7 | 1 3 #5 b7 | Cmaj7(#5) retains the major seventh |
| Csus47 | 1 4 5 b7 | Suspension does not turn 7 into a natural seventh |
| C+5, C7+5 | augmented fifth in place of 5 | b13 is a separate degree and does not remove 5 |
| Cm7b5sus2 | 1 2 b5 b7 | Suspension changes the third and retains the altered fifth |
| Caug9 | 1 3 #5 b7 9 | Caug(9) adds 9 without a seventh |
| Csus49 | 1 4 5 b7 9 | Csus4(9) adds 9 without a seventh |
| C7(omit3,5) | 1 b7 | The omit verb is inherited inside this comma-separated group |

An inherited omission ends at a new explicit command or group boundary.
For example, C7(omit3,add9,5) retains 5, while C7(omit3,5)(add9)
omits both 3 and 5 and then adds 9. Unknown text still prevents a StrictV1
formula instead of silently falling back to a major triad.

The aug9/sus49 and shorthand omission-list cases adopt the ordinary extended
ninth and comma-list readings. These should be distinguished from parenthetical
added tensions, which do not require all lower extensions. A chord symbol alone
does not determine register, doubling, or every note in a performed voicing.

References:

- [MusicXML chord kinds](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/kind-value/)
- [MusicXML added degrees](https://www.w3.org/2021/06/musicxml40/tutorial/chord-symbols-and-diagrams/)
- [University of Puget Sound: Jazz Chord Basics](https://musictheory.pugetsound.edu/mt21c/JazzChordBasics.html)
- [Maricopa: Special Cases](https://open.maricopa.edu/mtc130/chapter/special-cases/)

Run the Rust regressions with:

```sh
cargo test --manifest-path chord-romanizer-rs/Cargo.toml
```
