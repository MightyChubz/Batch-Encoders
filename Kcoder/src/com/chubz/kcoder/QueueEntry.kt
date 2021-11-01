package com.chubz.kcoder

/**
 * The queue entry is a structure that holds all the important information for the queue, ensuring ffmpeg has everything
 * it needs to encode correctly.
 *
 * @param input The input file to transcode
 * @param output The output file of the transcode
 * @param crf The crf rating of the encode
 * @param vLibrary The video format to use for encoding
 * @param vtype The video file type
 * @param aLibrary The audio format to use for encoding
 * @param aBitrate The bitrate for the audio streams.
 */
class QueueEntry(
    val input: String,
    var output: String = "",
    var crf: Int,
    var vLibrary: String = "libaom-av1",
    var vtype: String = "webm",
    var aLibrary: String = "libopus",
    var aBitrate: Int = 128
)
