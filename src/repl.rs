use std::io::{ self, Write };
use std::str::FromStr;
use crate::config::Config;
use crate::utility::*;
use std::process::Command;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum VideoResolution {
    R2160,
    R1440,
    R1080,
    R720,
    R480,
    R360,
    R240,
    R144,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVideoResolutionError;

impl FromStr for VideoResolution {
    type Err = ParseVideoResolutionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "144" => Ok(VideoResolution::R144),
            "240" => Ok(VideoResolution::R240),
            "360" => Ok(VideoResolution::R360),
            "480" => Ok(VideoResolution::R480),
            "720" => Ok(VideoResolution::R720),
            "1080" => Ok(VideoResolution::R1080),
            "1440" => Ok(VideoResolution::R1440),
            "2160" => Ok(VideoResolution::R2160),
            _ => Err(ParseVideoResolutionError),
        }
    }
}


fn download_video(resolution: Option<VideoResolution>) -> Result<(), Box<dyn std::error::Error>> {
    let url = get_input("Enter the url: ")?;

    let res: VideoResolution = match resolution {
        Some(res) => res,
        None => {
            let mut _res: VideoResolution;

            loop {
                let input = get_input("Enter a resolution(e.g 1080): ")?;

                _res = match input.parse::<VideoResolution>() {
                    Ok(res) => res,
                    Err(_) => {
                        println!("Invalid resolution: {}", input);
                        continue;
                    }
                };

                break;
            }

            _res
        }
    };

    let browsers = fetch_found_broswers();

    if browsers.is_empty() {
        return Err("no browsers found, need a browser to get cookies from".to_string().into());
    }

    println!("Found browsers:\n");

    for (i, browser) in browsers.iter().enumerate() {
        println!("[{}]: {}", i + 1, browser);
        io::stdout().flush()?;
    }

    let mut _chosen_browser: &str;

    loop {
        let input = get_input("Choose the browswer in which you are logged in to youtube: ")?;

        let index: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input: {}", input);
                continue;
            }
        };

        _chosen_browser = match browsers.get(index - 1) {
            Some(browser) => browser,
            None => {
                println!("Invalid option: {}", index);
                continue;
            }
        };

        break;
    }

    let filename_template = get_output_filename()?;

    let status = Command::new(Config::get_yt_dlp_path())
        .arg("--ffmpeg-location")
        .arg(Config::get_ffmpeg_path())
        .arg("--js-runtimes")
        .arg(format!("deno:{}", Config::get_deno_path().display()))
        .arg("--cookies-from-browser")
        .arg(_chosen_browser)
        .arg("-f")
        .arg(resolution_to_format_string(res))
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("-P")
        .arg(Config::get_download_dir())
        .arg("-o")
        .arg(&filename_template)
        .arg(url)
        .status()?;

    let exit_code = match status.code() {
        Some(code) => code,
        None => 1,
    };

    if !status.success() {
        return Err(format!("yt-dlp exited with ({})", exit_code).into());
    } else {
        println!("Video downloaded to: {}", Config::get_download_dir().display());
    }

    Ok(())
}

pub fn run_menu() -> Result<(), Box<dyn std::error::Error>> {
    let menu = format!(
        r"
*********************************************************
*               Youtube Video Downloader!               *
*                  author: 0xmadumere                   *
*********************************************************

[1] Download video (best quality)
[2] Download video (choose resolution)
[3] Download audio only (mp3) COMING SOON
[4] Set output folder COMING SOON
[5] Exit

output folder: {}
---------------------------------------------------------
", Config::get_download_dir().display()
);

    'outer_loop: loop {
        print!("{}", menu);
        io::stdout().flush()?;

        loop {
            let input = get_input("Select an option: ")?;

            let option: u8 = match input.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid input: {}", input);
                    continue;
                }
            };

            match option {
                1 => {
                    if let Err(e) = download_video(Some(VideoResolution::R2160)) {
                        println!("Error: {}", e);
                    }
                }
                2 => {
                    if let Err(e) = download_video(None) {
                        println!("Error: {}", e);
                    }
                }
                3 => {
                    println!("Coming soon.");
                    continue;
                },
                4 => {
                    println!("Coming soon.");
                    continue;
                },
                5 => {
                    println!("Goodbye!");
                    break 'outer_loop;
                }
                op => {
                    println!("invalid option: {}", op);
                    continue;
                }
            }

            break;
        }
    }

    Ok(())
}
