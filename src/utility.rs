use std::io::{self, Write};
use std::path::PathBuf;
use std::fs;
use browser_locations::{ Browser, locate_any_stable };
use crate::repl::*;

pub fn get_input(placeholder: &str) -> io::Result<String>
{
    loop {

        print!("{}", placeholder);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty()
        {
            continue;
        } 
        return Ok(input.to_string());
    }
}

pub fn get_output_filename() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();

    print!("Enter filename (leave blank for default): ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut input)?;

    let input = input.trim();

    if input.is_empty() {
        // yt-dlp's own default template
        Ok("%(title)s [%(id)s].%(ext)s".to_string())
    } else {
        // user-provided name, yt-dlp still needs to control the extension
        Ok(format!("{}.%(ext)s", input))
    }
}

pub fn resolution_to_format_string(res: VideoResolution) -> String {
    let height = match res {
        VideoResolution::R2160 => 2160,
        VideoResolution::R1440 => 1440,
        VideoResolution::R1080 => 1080,
        VideoResolution::R720 => 720,
        VideoResolution::R480 => 480,
        VideoResolution::R360 => 360,
        VideoResolution::R240 => 240,
        VideoResolution::R144 => 144,
    };

    format!("bestvideo[height<={}]+bestaudio/best[height<={}]", height, height)
}

pub fn fetch_found_broswers() -> Vec<String> {
    let browsers_to_check = vec![
        Browser::Chrome,
        Browser::Firefox,
        Browser::Edge,
        Browser::Brave,
        Browser::Opera
    ];

    let mut found_browsers: Vec<String> = Vec::new();

    for browser in browsers_to_check {
        if let Ok(_) = locate_any_stable(browser.clone()) {
            found_browsers.push(browser.to_string());
        }
    }

    found_browsers
}

pub fn chmod_executable(_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(_path, perms)?;
    }
    Ok(())
}