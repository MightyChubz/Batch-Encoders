use std::fs::read_dir;
use std::io::{stdin, stdout, Stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use terminal::{Action, Clear, Terminal};

#[derive(Clone)]
pub(crate) struct QueueEntry<'a> {
    pub input: String,
    pub output: String,
    pub crf: u8,
    pub video_library: String,
    pub video_container: String,
    pub special_video_params: Vec<&'a str>,
    pub audio_library: String,
    pub audio_bitrate: u8,
}

impl<'a> Default for QueueEntry<'a> {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            crf: 0,
            video_library: String::from("libaom-av1"),
            video_container: String::from("webm"),
            special_video_params: vec!["-row-mt", "1", "-tiles", "4x4", "-cpu-used", "8"],
            audio_library: String::from("libopus"),
            audio_bitrate: 128,
        }
    }
}

#[derive(Default)]
pub struct Program<'a> {
    queue: Vec<QueueEntry<'a>>,
    exit: bool,
}

impl<'a> Program<'a> {
    pub fn run() {
        let mut program = Self::default();

        loop {
            if program.exit {
                break;
            }

            // program.clear_console();
            program.print_queue();
            program.take_input();
        }
    }

    fn print_queue(&self) {
        if !self.queue.is_empty() {
            for (i, entry) in self.queue.iter().enumerate() {
                println!(
                    "{}: [INPUT: {}, OUTPUT: {}, CRF: {}, VIDEO_LIBRARY: {}, \
                VIDEO_CONTAINER: {}, SPECIAL_PARAMS: \"{}\", AUDIO_LIBRARY: {}, AUDIO_BITRATE: {}K]",
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

    fn clear_console(&self) {
        let terminal = terminal::stdout();
        terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
        terminal.act(Action::MoveCursorTo(0, 0)).unwrap();
        terminal.flush_batch().unwrap();
    }

    fn prompt(&self, message: &str) -> String {
        println!("{} ", message);
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

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

    fn take_input(&mut self) {
        let result =
            self.prompt("[A]dd file; [R]emove file; [C]hange CRF; [E]ncode queue; [Q]uit ");
        match result.trim().to_lowercase().chars().next().unwrap_or('z') {
            'a' => self.add_file(),
            'r' => self.remove_file(),
            'c' => self.change_crf(),
            'e' => self.encode_queue(),
            'q' => self.exit = true,
            _ => {}
        }
    }

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

    fn add_file(&mut self) {
        let files = self.list_dir(Path::new("."));
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i, file);
        }

        let input_index = self.prompt_u8("Select input file: ");
        let input = files[input_index as usize].as_str();
        let crf = self.prompt_u8("CRF Rating: ");
        let video_library = self.prompt("Video Library: ");
        let video_container = self.prompt("Video Filetype: ");
        // let special_video_params = self.prompt("Enter Special Video Params: ");
        let audio_library = self.prompt("Audio Library: ");
        let audio_bitrate = self.prompt_u8("Audio Bitrate: ");

        let mut entry = QueueEntry {
            input: input.to_string(),
            crf,
            ..Default::default()
        };

        if !video_library.trim().is_empty() {
            entry.video_library = video_library;
        }

        if !video_container.trim().is_empty() {
            entry.video_container = video_container;
        }

        // if !special_video_params.trim().is_empty() {
        //     let split: Vec<String> = special_video_params.split(' ').map(|e| e.to_string()).collect();
        //     entry.special_video_params = ;
        // }

        if !audio_library.trim().is_empty() {
            entry.audio_library = audio_library;
        }

        if audio_bitrate != 0 {
            entry.audio_bitrate = audio_bitrate;
        }

        let mut output = PathBuf::new();
        output.push(Path::new(input).file_name().unwrap());
        output.set_extension(&entry.video_container);
        entry.output = output.to_str().unwrap().to_string();

        self.queue.push(entry);
    }

    fn select(&self) -> usize {
        self.print_queue();

        self.prompt_u8("Select Entry from Queue: ") as usize
    }

    fn remove_file(&mut self) {
        let selection = self.select();
        self.queue.remove(selection as usize);
    }

    fn change_crf(&mut self) {
        let selection = self.select();
        let crf = self.prompt_u8("New CRF Rating: ");

        self.queue[selection].crf = crf;
    }

    fn encode_queue(&mut self) {
        let queue = self.queue.to_vec();
        for entry in queue.iter() {
            let commands = self.generate_ffmpeg_command(entry);
            // self.clear_console();
            Command::new("ffmpeg")
                .args(commands)
                .stdout(Stdio::inherit())
                .stdin(Stdio::inherit())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    }

    fn generate_ffmpeg_command(&self, entry: &QueueEntry) -> Vec<String> {
        let input = entry.input.to_string();
        let video_library = entry.video_library.to_string();
        let crf = entry.crf.to_string();
        let special_video_params: Vec<String> = entry
            .special_video_params
            .iter()
            .map(|e| e.to_string())
            .collect();
        let audio_library = entry.audio_library.to_string();
        let audio_bitrate = format!("{}K", entry.audio_bitrate);
        let output = entry.output.to_string();

        let mut commands = vec![];
        commands.append(
            &mut [
                "-hwaccel",
                "auto",
                "-i",
                &input,
                "-c:v",
                &video_library,
                "-crf",
                &crf,
                "-b:v",
                "0",
                "-pix_fmt",
                "yuv420p",
            ]
            .to_vec(),
        );

        // commands.append(&mut special_video_params.iter().map(|e| e.as_str()).collect());
        commands.append(&mut ["-c:a", &audio_library, "-b:a", &audio_bitrate, &output].to_vec());

        commands.iter().map(|e| e.to_string()).collect()
    }
}
