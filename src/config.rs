use std::env;
use std::path::{ Path, PathBuf };
use std::sync::OnceLock;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ArchiveType {
    #[default]
    None,
    Zip,
    TarXz,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Dependency {
    pub name: String,
    pub download_url: String,
    pub path: PathBuf,
    pub archive_type: ArchiveType,
    pub zip_path: PathBuf,
}

impl Dependency {
    fn new(
        name: String,
        download_url: String,
        path: &PathBuf,
        archive_type: ArchiveType,
        zip_path: &PathBuf
    ) -> Dependency {
        Dependency {
            name,
            download_url,
            path: path.to_path_buf(),
            archive_type,
            zip_path: zip_path.to_path_buf(),
        }
    }
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Config {
    home_path: PathBuf,
    yt_dlp_path: PathBuf,
    ffmpeg_path: PathBuf,
    ffmpeg_zip_path: PathBuf,
    deno_path: PathBuf,
    deno_zip_path: PathBuf,
    download_dir: PathBuf,
    dependencies: Vec<Dependency>,
}

#[allow(dead_code)]
impl Config {
    // Private singleton instance accessor
    fn instance() -> &'static Config {
        static CONFIG_INSTANCE: OnceLock<Config> = OnceLock::new();
        CONFIG_INSTANCE.get_or_init(|| {
            let home_str = if cfg!(windows) {
                env::var("USERPROFILE").expect("USERPROFILE environment variable not set")
            } else {
                env::var("HOME").expect("HOME environment variable not set")
            };

            let home_path = PathBuf::from(home_str);

            let yt_dlp_path = home_path
                .join("tools")
                .join("yt_dlp")
                .join(if cfg!(target_os = "windows") { "yt_dlp.exe" } else { "yt_dlp" });

            let ffmpeg_path = home_path
                .join("tools")
                .join("ffmpeg")
                .join(if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "ffmpeg" });

            let ffmpeg_zip_path = home_path
                .join("tools")
                .join("ffmpeg")
                .join(if cfg!(target_os = "windows") { "ffmpeg.zip" } else { "ffmpeg.tar.xz" });

            let deno_path = home_path
                .join("tools")
                .join("deno")
                .join(if cfg!(target_os = "windows") { "deno.exe" } else { "deno" });

            let deno_zip_path = home_path.join("tools").join("deno").join("deno.zip");

            let download_dir = match yt_dlp_path.parent() {
                Some(path) => path.join("downloads"),
                None => panic!("Error: yt_dlp_path has no parent directory: {:?}", yt_dlp_path),
            };

            let mut config = Config {
                home_path: home_path,
                yt_dlp_path: yt_dlp_path,
                ffmpeg_path: ffmpeg_path,
                ffmpeg_zip_path: ffmpeg_zip_path,
                deno_path: deno_path,
                deno_zip_path: deno_zip_path,
                download_dir: download_dir,
                ..Default::default()
                // dependencies: dependencies,
            };

            config.init_dependencies();

            config
        })
    }

    pub fn get_home_path() -> &'static Path {
        &Self::instance().home_path
    }

    pub fn get_download_dir() -> &'static Path {
        &Self::instance().download_dir
    }

    pub fn get_dependencies() -> &'static Vec<Dependency> {
        &Self::instance().dependencies
    }

    pub fn get_yt_dlp_path() -> &'static Path {
        &Self::instance().yt_dlp_path
    }

    pub fn get_ffmpeg_path() -> &'static Path {
        &Self::instance().ffmpeg_path
    }

    pub fn get_deno_path() -> &'static Path {
        &Self::instance().deno_path
    }

    fn init_dependencies(&mut self) {
        self.dependencies.push(
            Dependency::new(
                "deno".to_string(),
                if cfg!(target_os = "windows") {
                    "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-pc-windows-msvc.zip".to_string()
                } else {
                    "https://github.com/denoland/deno/releases/latest/download/deno-x86_64-unknown-linux-gnu.zip".to_string()
                },
                &self.deno_path,
                ArchiveType::Zip,
                &self.deno_zip_path
            )
        );

        self.dependencies.push(
            Dependency::new(
                "ffmpeg".to_string(),
                if cfg!(target_os = "windows") {
                    "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip".to_string()
                } else {
                    "https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz".to_string()
                },
                &self.ffmpeg_path,
                if cfg!(target_os = "windows") {
                    ArchiveType::Zip
                } else {
                    ArchiveType::TarXz
                },
                &self.ffmpeg_zip_path
            )
        );

        self.dependencies.push(
            Dependency::new(
                "yt_dlp".to_string(),
                if cfg!(target_os = "windows") {
                    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe".to_string()
                } else {
                    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp".to_string()
                },
                &self.yt_dlp_path,
                ArchiveType::None,
                &PathBuf::from("")
            )
        );
    }
}
