<p align = "center">
  <img 
    height = "200"
    src = "https://raw.githubusercontent.com/shmishtopher/VAU/master/media/logo.png"
    alt = "VAU - The Vocaloid Archive Utility"
  />
</p>

`vau` is a tool for unpacking vocaloid voicebanks (.ddb files) into wave files.
`vau` enables you to extract voicebanks quicky and with a variety of export
options. Support for repacking voicebanks is planned for the future.

## Usage
Getting help
```bash
> vau help
```

Extract an archive into a single wave file called `samples.wav`
```bash
> vau /PATH/TO/ARCHIVE.ddb -f samples.wav
```

Extract all of the samples in an archive to a foler called `samples/`
```bash
> vau /PATH/TO/ARCHIVE.ddb -d samples.wav
```

Extract an archive with a different sample rate
```bash
> vau /PATH/TO/ARCHIVE.ddb -f samples.wav -r 44100
```

Extracting an archive with a different bit depth and sample rate
```bash
> vau /PATH/TO/ARCHIVE.ddb -f samples.wav -r 44100 -b 16
```

All flags
```
-h, --help       Prints help information
-V, --version    Prints version information
```

All Options
```
-b, --bits <bit_depth>      Configures the bit depth for of the samples, defaults to 32
-d, --outDir <out_dir>      Place each sample in its own file in this directory
-f, --outFile <out_file>    Concatenate and emit samples to a single file
-r, --rate <sample_rate>    Configures the sample rate of the samples, defaults to 22050
```

Usage
```bash
> vau [OPTIONS] <archive>
```

## Installation
To install `vau`, head over to the releases page or click [here](https://github.com/shmishtopher/VAU/releases)
and select the latest version of `vau` compatible with your operating system.  Place `vau` in your working
directory or add it to your path to invoke it anywhere.

## Contributing
Want to see a feature added? Found a bug? Just want to join the disscussion?
Open an issue or submit a pull request!  When submitting a bug report, please
make sure you include a description of the expected behavior as well as the
steps required to reproduce the error.
