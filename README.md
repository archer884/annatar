# Annatar

> A command line tool for adding text to pictures

This crate is named for the Dark Lord Sauron. The name is taken from the Quenya for 'Lord of Gifts,' which has a hilarious mispronunciation that seems to apply almost directly to this case. Like the library it's based on, this is clearly for making evil things.

## Installation

Visit [rustup.rs](https://rustup.rs/) to download the Rust tools, then run the following command via cargo:

```shell
cargo install annatar
```

## Usage

```shell
annatar foo.jpg \
    --top "This text will appear near the top of the image." \
    --middle "This text will appear near the middle of the image." \
    --bottom "This text will appear near the bottom of the image."
```

Per the usual conventions, `-t`, `-m`, and `-b` are also available as arguments. Additionally, it is possible to supply only a caption as a positional argument, in which case the caption works as a synonym for `--bottom`, e.g.:

```shell
annatar foo.jpg "This text will appear near the bottom of the image."
```

Images may be provided as either local paths or URLs; annatar is happy to fetch your picture from the internet for you.

> Note: Linux usually does not include annatar's default font, Impact, which will result in an error unless an alternate font is selected. To avoid this, set the var `ANNATAR_DEFAULT_FONT` in your shell profile. Feel free to pick something you like.

### Annotation size

By default, annatar sizes the text used for your captions on the basis of the height of the image itself. The exact algorithm used for this purpose was selected by a team of scientists working round the clock for weeks on end at the Vatican, and we didn't let them out until we saw white smoke. Rumors that the members of our text scaling enclave were able to agree only once the majority of members had starved or been bludgeoned to death by the others are, as far as you know, unfounded.

The important thing is that, normally, the text will look ok. For images with strange aspect ratios (either very wide or very narrow relative to their height), text can look either too large or too small. In that case, or in the case wherein you prefer to express greater emphasis, you may prefer to pass the `-s --scale` flag with a scaling multiplier.

```shell
annatar doge.png \
    --scale 2.0 \
    --top "SUCH BIG" \
    --bottom "SO SCALE"
```

This scale multiplier acts (surprisingly) as a *multiplier* for the scaling value selected by annatar. So, text scaled at `2.0` will be twice as tall (annatar scales text by height, proportionally) as it would have been otherwise.

> Note: You will probably find a value like `2.0` to be excessive under most circumstances; I usually scale by about 30%—`0.7` or `1.3`—at most.

#### In-band annotation scaling

The `-s --scale` mutliplier is set for *all* annotations, top, middle, and bottom. To allow annotations of different size, an in-band scaling format is provided.

```shell
annatar doge.png \
    --top "this one is normal" \
    --bottom "\1.3 this one is bigger!"
```

White space between the scaling modifier (`\1.3` above) and the annotation (`this one is bigger`) will be ignored. However, some amount of intervening white space *is required.*

> Note: for those of you who are plagued by morbid curiosity, here's the regular expression used: `\\(?P<scale>\d+(\.\d+)?)\s+(?P<caption>.+)`.

### Rightsholder protections

We realize that laws in places like Britain or the European Union may not support the concept of "fair use" we have in the United States. This program is explicitly intended to create derivative and transformative works for the purpose of fair and reasonable criticism and commentary, but that may not be a right [afforded to you](https://fee.org/articles/why-it-matters-that-some-rights-are-inalienable/) by those who rule over you in your jurisdiction. If that is the case, we encourage you to use annatar ONLY with the `--rightsholder-protections` flag set.

It may help to create a function similar to the following in your shell profile. The following function is written for [PowerShell](https://github.com/PowerShell/PowerShell).

```powershell
<#
.SYNOPSIS
Invokes annatar with rightsholder protections enabled.

.DESCRIPTION
This function performs pass-through for all arguments passed to it and adds 
the --rightsholder-protections flag in order to maintain compatibility with 
non-US copyright law when using annatar.

.NOTES
Annatar may print alarming warnings when rightsholder protections are enabled.
Please ignore these. They will be addressed in a future release.
#>
function Invoke-Annatar {
    annatar $args --rightsholder-protections
}
```

## Version history

- **0.5.7** Permit selection of default font via environment var
- **0.5.6** Fix regression: scale should be optional.
- **0.5.2** Fix macOS font problem
- **0.4.0** Add in-band annotation scaling

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: https://github.com/archer884/annatar/blob/master/LICENSE-MIT
[LICENSE-MIT]: https://github.com/archer884/annatar/blob/master/LICENSE-APACHE
