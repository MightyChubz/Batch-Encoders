use std::fs::read_dir;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use terminal::{Action, Clear};

/// This is an entry with all the needed information for each and every encode.
#[derive(Clone)]
pub(crate) struct QueueEntry {
    /// The input file to read from
    pub input: String,
    /// The output file to create
    pub output: String,
    /// The CRF rating of video
    pub crf: String,
    /// The video library to use when encoding
    pub video_library: String,
    /// The video container/extension to use. This is typically best consider the file type (.mkv,
    /// .mp4, .webm, etc)
    pub video_container: String,
    /// An array with special video parameters, this can be settings specific to the video library used
    pub special_video_params: Vec<String>,
    /// The audio library to use when encoding
    pub audio_library: String,
    /// The audio bitrate to use for the encode
    pub audio_bitrate: String,
}

impl Default for QueueEntry {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            crf: String::new(),
            video_library: String::from("libaom-av1"),
            video_container: String::from("webm"),
            special_video_params: vec!["-row-mt", "1", "-tiles", "4x4", "-cpu-used", "8"]
                .iter()
                .map(|e| e.to_string())
                .collect(),
            audio_library: String::from("libopus"),
            audio_bitrate: String::from("128K"),
        }
    }
}

/// The main program class and main runtime of the program, this is where all the input and encoding
/// happens.
#[derive(Default)]
pub struct Program {
    /// This is a [Vec] with all the [QueueEntry]s added in by the user
    queue: Vec<QueueEntry>,
    /// A flag for whether or not to close the program
    exit: bool,
}

impl Program {
    /// Starts an instance of [Program] and runs it.
    ///
    /// # Example
    /// ```
    /// // Starts the program
    /// Program::run();
    /// ```
    pub fn run() {
        let mut program = Self::default();

        loop {
            if program.exit {
                break;
            }

            program.clear_console();
            program.print_queue();
            program.take_input();
        }
    }

    /// Cycles through any present entry on the queue and prints it out with all the information and
    /// its index within the queue.
    fn print_queue(&self) {
        if !self.queue.is_empty() {
            for (i, entry) in self.queue.iter().enumerate() {
                println!(
                    "{}: [INPUT: {}, OUTPUT: {}, CRF: {}, VIDEO_LIBRARY: {}, \
                VIDEO_CONTAINER: {}, SPECIAL_PARAMS: \"{}\", AUDIO_LIBRARY: {}, AUDIO_BITRATE: {}]",
                    i,
                    entry.input,
                    entry.output,
                    entry.crf,
                    entry.video_library,
                    entry.video_container,
                    entry
                        .special_video_params
                        .iter()
                        .map(|e| format!("{} ", e))
                        .collect::<String>(),
                    entry.audio_library,
                    entry.audio_bitrate
                );
            }
        }
    }

    /// Create a temporary terminal hook to clear the terminal. This is mainly done like this for
    /// cross-platform purposes.
    fn clear_console(&self) {
        let terminal = terminal::stdout();
        terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
        terminal.act(Action::MoveCursorTo(0, 0)).unwrap();
        terminal.flush_batch().unwrap();
    }

    /// Prompts the user for input.
    ///
    /// # Arguments
    ///
    /// * `message`: Message to prompt the user with.
    ///
    /// returns: What the user input within a [String].
    ///
    /// # Examples
    ///
    /// ```
    /// let video_library = self.prompt("Enter Video Library (leave blank for auto): ");
    /// ```
    fn prompt(&self, message: &str) -> String {
        println!("{}", message);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Like the `prompt` function but parses and returns a [u8] instead of a [String].
    ///
    /// # Arguments
    ///
    /// * `message`: Message to prompt the user with.
    ///
    /// returns: A user input u8.
    ///
    /// # Examples
    ///
    /// ```
    /// let crf = self.prompt_u8("Enter CRF Rating: ").to_string();
    /// ```
    fn prompt_u8(&self, message: &str) -> u8 {
        loop {
            let input = self.prompt(message).trim().parse::<u8>();
            match input {
                Ok(result) => {
                    return result;
                }
                Err(_) => {
                    println!("Invalid input...");
                    continue;
                }
            }
        }
    }

    /// This lists all of the inputs the user can make, and reads for the next input, taking its
    /// first character and checking for correct selection.
    fn take_input(&mut self) {
        let result =
            self.prompt("[A]dd file; [R]emove file; [C]hange CRF; [E]ncode queue; [Q]uit ");
        match result.trim().to_lowercase().chars().next().unwrap_or('z') {
            'a' => self.add_file(),
            'r' => self.remove_file(),
            'c' => self.change_crf(),
            'e' => self.encode_queue_and_clear(),
            'q' => self.exit = true,
            _ => {}
        }
    }

    /// List a directory and sub directories recursively, showing only files.
    ///
    /// # Arguments
    ///
    /// * `dir`: The main [Path] directory to access and read from.
    ///
    /// returns: A [Vec<String>] of all the files, each entry being the path `dir` to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// let files = self.list_dir(&Path::new("."));
    /// for file in files.iter() {
    ///     // print every file found
    ///     println!("{}", file);
    /// }
    /// ```
    fn list_dir(&self, dir: &Path) -> Vec<String> {
        let mut files = vec![];

        if dir.is_dir() {
            let dir_contents = read_dir(dir).unwrap();
            for entry in dir_contents.flatten() {
                let file_path = entry.path();
                if file_path.is_dir() {
                    files.append(&mut self.list_dir(&file_path))
                } else {
                    files.push(file_path.to_str().unwrap().to_string())
                }
            }
        }

        files
    }

    /// Adds file to the queue for encode. This takes multiple parameters, from CRF rating to special
    /// video encoding parameters specific to the video library being used.
    fn add_file(&mut self) {
        let mut files = self.list_dir(Path::new("."));
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i, file);
        }

        let input_index = self.prompt_u8("Select input file: ");
        let input = files.remove(input_index as usize);
        let crf = self.prompt_u8("CRF Rating: ").to_string();
        let video_library = self.prompt("Video Library: ");
        let video_container = self.prompt("Video Filetype: ");
        let special_video_params = self.prompt("Enter Special Video Params: ");
        let audio_library = self.prompt("Audio Library: ");
        let audio_bitrate = self.prompt_u8("Audio Bitrate: ");

        let mut entry = QueueEntry {
            input,
            crf,
            ..Default::default()
        };

        if !video_library.trim().is_empty() {
            entry.video_library = video_library;
        }

        if !video_container.trim().is_empty() {
            entry.video_container = video_container;
        }

        if !special_video_params.trim().is_empty() {
            entry.special_video_params = special_video_params
                .split(' ')
                .map(|e| e.to_string())
                .collect();
        } else if entry.video_library != "libaom-av1" {
            entry.special_video_params = vec![];
        }

        if !audio_library.trim().is_empty() {
            entry.audio_library = audio_library;
        }

        if audio_bitrate != 0 {
            entry.audio_bitrate = format!("{}K", audio_bitrate);
        }

        let mut output = PathBuf::new();
        output.push(Path::new(&entry.input).file_name().unwrap());
        output.set_extension(&entry.video_container);
        entry.output = output.to_str().unwrap().to_string();

        self.queue.push(entry);
    }

    /// Prints the queue and prompts the user for input, this is used normally for selection in the
    /// queue when the user wants to modify it in some way. As a result of that, the function will
    /// loop and ensure that the selection made is a valid by checking if the input is a valid index
    /// in the queue.
    fn select(&self) -> usize {
        loop {
            self.clear_console();
            self.print_queue();

            let index = self.prompt_u8("Select Entry from Queue: ") as usize;
            if self.queue.get(index).is_some() {
                return index;
            }
        }
    }

    /// Removes user selected file from the queue.
    fn remove_file(&mut self) {
        let selection = self.select();
        self.queue.remove(selection as usize);
    }

    /// Lets the user modify the crf in an entry within the queue.
    fn change_crf(&mut self) {
        let selection = self.select();
        let crf = self.prompt_u8("New CRF Rating: ").to_string();

        self.queue[selection].crf = crf;
    }

    /// Encodes the entire queue, then clears it after batch encoding is complete.
    fn encode_queue_and_clear(&mut self) {
        let queue = self.queue.to_vec();
        for entry in queue.iter() {
            let commands = self.generate_ffmpeg_arguments(entry);
            self.clear_console();
            Command::new("ffmpeg")
                .args(commands)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        self.queue = vec![];
    }

    /// This takes a [QueueEntry] and forms the proper ffmpeg arguments.  
    ///
    /// # Arguments
    ///
    /// * `entry`: The [QueueEntry] to read from.
    ///
    /// returns: A [Vec] with all the arguments, meant to be fed in conjunction with [Command].
    ///
    /// # Examples
    ///
    /// ```
    /// // loop through all entries in the queue and encode all of them.
    /// let queue = self.queue.to_vec();
    /// for entry in queue.iter() {
    ///     let commands = self.generate_ffmpeg_arguments(entry);
    ///     self.clear_console();
    ///     Command::new("ffmpeg")
    ///         .args(commands)
    ///         .stdout(Stdio::inherit())
    ///         .stdin(Stdio::inherit())
    ///         .spawn()
    ///         .unwrap()
    ///         .wait()
    ///         .unwrap();
    /// }
    /// ```
    fn generate_ffmpeg_arguments(&self, entry: &QueueEntry) -> Vec<String> {
        let mut commands = vec![];
        commands.append(
            &mut [
                "-hwaccel",
                "auto",
                "-i",
                &entry.input,
                "-c:v",
                &entry.video_library,
                "-crf",
                &entry.crf,
                "-b:v",
                "0",
                "-pix_fmt",
                "yuv420p",
            ]
            .to_vec(),
        );

        commands.append(
            &mut entry
                .special_video_params
                .iter()
                .map(|e| e.as_str())
                .collect(),
        );
        commands.append(
            &mut [
                "-c:a",
                &entry.audio_library,
                "-b:a",
                &entry.audio_bitrate,
                &entry.output,
            ]
            .to_vec(),
        );

        commands.iter().map(|e| e.to_string()).collect()
    }
}
