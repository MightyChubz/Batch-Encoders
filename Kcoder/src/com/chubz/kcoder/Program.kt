package com.chubz.kcoder

import java.io.File
import java.util.*
import kotlin.system.exitProcess

class Program : Runnable {
    private val queue: MutableList<QueueEntry> = mutableListOf()

    override fun run() {
        while (true) {
            clearConsole()
            printQueue()
            takeInput()
        }
    }

    /**
     * Prints the entire queue of encodes ready to be done.
     */
    private fun printQueue() {
        if (queue.isNotEmpty()) {
            for (i in 0 until queue.size) {
                val entry = queue[i]
                println(
                    "${i + 1}: [INPUT: ${entry.input}, OUTPUT: ${entry.output}, CRF: ${entry.crf}, " +
                            "VLIBRARY: ${entry.vLibrary}, ALIBRARY: ${entry.aLibrary}, FILE TYPE: ${entry.vtype}, " +
                            "AUDIO BITRATE: ${entry.aBitrate}K]"
                )
            }
        }
    }

    /**
     * Takes input from the user for different operations, whether it's adding files, removing them, changing crf,
     * encoding the queue, or quiting the program.
     */
    private fun takeInput() {
        val input = prompt("[A]dd File, [R]emove File, [C]hange CRF, [E]ncode Queue, [Q]uit ")[0]
        when (input.lowercase()) {
            "a" -> addFile()
            "r" -> removeFile()
            "c" -> changeCRF()
            "e" -> encodeQueue()
            "q" -> exitProcess(0)
        }
    }

    /**
     * Prompts the user with message and waits for input.
     *
     * @param message Message to prompt the user with
     * @return User inputted [String]
     */
    private fun prompt(message: String): String {
        print(message)

        val scanner = Scanner(System.`in`)
        return scanner.nextLine()
    }

    /**
     * Prompts the user with message and waits for a numbered input.
     *
     * @param message Message to prompt the user with
     * @return User inputted [Int]
     */
    private fun promptInt(message: String): Int {
        val scanner = Scanner(System.`in`)
        var number: Int
        while (true) {
            try {
                print(message)
                number = scanner.nextInt()
                break
            } catch (e: InputMismatchException) {
                println("Input invalid...")
                continue
            }
        }

        return number
    }

    /**
     * Lists files in specified directory.
     *
     * @param dir The directory to list.
     * @return [Set] with all file names in the directory.
     */
    private fun listFiles(dir: String = "./"): List<String> {
        val files = File(dir).listFiles()
        return files!!
            .filter { file -> !file.isDirectory && !queue.any { it.input == file.name } }
            .map { it.name }
    }

    /**
     * Adds file to queue.
     */
    private fun addFile() {
        val files = listFiles()
        for (i in files.indices) {
            val entry = files[i]
            println("${i + 1}: $entry")
        }

        val inputIndex = promptInt("Select Input File: ")
        val input = files[inputIndex - 1]
        val crf = promptInt("CRF Rating: ")
        val vLibrary = prompt("Video Library: ")
        val filetype = prompt("Video filetype: ")
        val aLibrary = prompt("Audio Library: ")
        val aBitrate = promptInt("Audio Bitrate: ")

        val entry = QueueEntry(input = input, crf = crf)
        if (vLibrary.isNotBlank()) {
            entry.vLibrary = vLibrary
        }

        if (filetype.isNotBlank()) {
            entry.vtype = filetype
        }

        if (aLibrary.isNotBlank()) {
            entry.aLibrary = aLibrary
        }

        if (aBitrate != 0) {
            entry.aBitrate = aBitrate
        }

        entry.output = "${input.split(".")[0]}.${entry.vtype}"
        queue.add(entry)
    }

    /**
     * Removes file from queue.
     */
    private fun removeFile() {
        printQueue()

        val entryNumber = promptInt("Select Entry: ")
        queue.removeAt(entryNumber - 1)
    }

    /**
     * Changes crf rating of queue entry
     */
    private fun changeCRF() {
        printQueue()

        val entryNumber = promptInt("Select Entry: ")
        val crf = promptInt("CRF Rating: ")

        queue[entryNumber - 1].crf = crf
    }

    /**
     * Encodes the entire queue to the AV1 format using the webm container.
     */
    private fun encodeQueue() {
        for (entry in queue) {
            val command = generateFfmpegCommand(entry)

            clearConsole()
            ProcessBuilder(command)
                .inheritIO()
                .start()
                .waitFor()
        }
    }

    /**
     * Generates the proper ffmpeg command depending on the os the user is on.
     *
     * @param entry The current [QueueEntry] to generate the command for.
     * @return [List] of the command and its arguments.
     */
    private fun generateFfmpegCommand(entry: QueueEntry): List<String> {
        val command = mutableListOf<String>()
        val os = System.getProperty("os.name")
        if (os.contains("Windows")) {
            command.addAll(listOf("cmd", "/c"))
        }
        command.addAll(
            listOf(
                "ffmpeg",
                "-hwaccel",
                "auto",
                "-i",
                "\"${entry.input}\"",
                "-c:v",
                entry.vLibrary,
                "-crf",
                entry.crf.toString(),
                "-b:v",
                "0",
                "-pix_fmt",
                "yuv420p",
            )
        )

        if (entry.vLibrary == "libaom-av1") {
            command.addAll(listOf(
                "-row-mt",
                "1",
                "-tiles",
                "4x4",
                "-cpu-used",
                "8",
            ))
        }

        command.addAll(
            listOf(
                "-c:a",
                entry.aLibrary,
                "-b:a",
                "${entry.aBitrate}K",
                "\"${entry.output}\""
            )
        )

        return command
    }

    /**
     * Clears the console window.
     */
    private fun clearConsole() {
        val os = System.getProperty("os.name")
        if (os.contains("Windows")) {
            ProcessBuilder("cmd", "/c", "cls").inheritIO().start().waitFor()
        } else {
            print("\\033[H\\033[2J")
            System.out.flush()
        }
    }
}

fun main() {
    val program = Program()
    program.run()
}