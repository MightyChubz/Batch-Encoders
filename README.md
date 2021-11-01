# Batch Encoders
A collection of the same program written in different languages.

## About
This program is a simple batch encoder, it allows you to add multiple files, set crf rating, and exclusive to `Kcoder`, you get to modify the video and audio codecs, and bitrate for audio. This is a program that is easy to make, as shown with the `encode.bat` implementation, but with the limitations of bat script, I had to shift to another language, like `Python` or `Kotlin`. This is a project that I plan to update, to show my knowledge with each language I add, to show that I'm competent enough in them to at least get a program like this done. Hopefully you, the person reading this, learn something from any of the implementations shown here.

## Requirements
The requirements for all of these implementations are as such:
- Latest `Java Runtime` (and `Java JDK 17` if you plan to add to the code)
- `ffmpeg`, the latest version if possible

## Warnings
This may be fixed by the time I post this, but on `Arch Linux`, there is a present problem with `ffmpeg` when encoding with the `libaom-av1` codec due to an `ABI version mismatch`. Until the program is updated to fix this, you may or may not experience major problem using any of these programs (besides `Kcoder` currently since it allows unique video and audio codecs).

## Features
For each of these programs, they should implement these set features unless limited by the language, or shown to be far too much work to implements (`bat script` being a perfect example).

The set features are these:
- A queue for multiple encodes
- An input file selector that lists all files in the base directory, allowing for numbered selection
- To automatically set the output file with proper extension
- The ability to edit the CRF rating
- The ability to remove a file from the queue
- The ability to set video and audio codecs
- The ability to modify the bitrate of the audio codec
- And finally, the ability to add custom special params for the video codec; this is important since to enable multithreading for av1, you have to write special parameters such as `-row-mt 1 -tiles 4x4`