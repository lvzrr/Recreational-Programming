const PYTHON_ICO: &str = "";
const RUST_ICO: &str = "";
const TOML_ICO: &str = "";
const JAVA_ICO: &str = "";
const HTML_ICO: &str = "";
const MARKDOWN_ICO: &str = "󰍔";
const JS_ICO: &str = "";
const C_ICO: &str = "";
const CPP_ICO: &str = "";
const GO_ICO: &str = "󰟓";
const PDF_ICO: &str = "";
const PPTTX_ICO: &str = "󱎐";
const JAVA_CLASS_ICO: &str = "";
const CSS_ICO: &str = "";
const XML_ICO: &str = "󰗀";
const JSON_ICO: &str = "";
const YAML_ICO: &str = "";
const DOCKER_ICO: &str = "";
const SQL_ICO: &str = "";
const VIM_ICO: &str = "";
const BASH_ICO: &str = "";
const GIT_ICO: &str = "";
const LUA_ICO: &str = "";
const SWIFT_ICO: &str = "";
const RUBY_ICO: &str = "";
const PHP_ICO: &str = "";
const PERL_ICO: &str = "";
const SHELL_ICO: &str = "";
const TEX_ICO: &str = "";
const R_ICO: &str = "";
const KOTLIN_ICO: &str = "";
const SCALA_ICO: &str = "";
const TS_ICO: &str = "";
const CSHARP_ICO: &str = "";
const POWERSHELL_ICO: &str = "󰨊";
const PSD_ICO: &str = "";
const JPG_ICO: &str = "󰈥";
const PNG_ICO: &str = "󰸭";
const GIF_ICO: &str = "󰵸";
const BMP_ICO: &str = "";
const TIFF_ICO: &str = "";
const SVG_ICO: &str = "󰜡";
const MP3_ICO: &str = "󰈣";
const WAV_ICO: &str = "󰈣";
const FLAC_ICO: &str = "󰈣";
const MP4_ICO: &str = "";
const MKV_ICO: &str = "";
const AVI_ICO: &str = "";
const MOV_ICO: &str = "";
const ZIP_ICO: &str = "";
const RAR_ICO: &str = "";
const TAR_ICO: &str = "";
const GZ_ICO: &str = "";
const EXE_ICO: &str = "";
const DMG_ICO: &str = "";
const ISO_ICO: &str = "󰗮";
const TXT_ICO: &str = "";
const DOC_ICO: &str = "";
const DOCX_ICO: &str = "";
const XLS_ICO: &str = "";
const XLSX_ICO: &str = "";
const PPTX_ICO: &str = "󱎐";
const EPUB_ICO: &str = "";
const ODT_ICO: &str = "";
const ODS_ICO: &str = "";
const ODP_ICO: &str = "";
const CSV_ICO: &str = "";
const ICO_ICO: &str = "";
const HEADERFILE_ICO: &str = "";
const CONF_ICO: &str = "";
const FOLDER_ICO: &str = "";
const REG_ICO: &str = "";
const BIN_ICO: &str = "";
pub const CONF_FOLDER_ICO: &str = "";
const SYMLINK_ICO: &str = "";

// Bright ANSI color codes
pub const WHITE: &str = "\x1B[37m";
pub const BRIGHT_BLACK: &str = "\x1b[1;30m";
pub const BRIGHT_RED: &str = "\x1b[1;31m";
pub const BRIGHT_GREEN: &str = "\x1b[1;32m";
pub const BRIGHT_YELLOW: &str = "\x1b[1;33m";
pub const BRIGHT_BLUE: &str = "\x1b[1;34m";
pub const BRIGHT_MAGENTA: &str = "\x1b[1;35m";
pub const BRIGHT_CYAN: &str = "\x1b[1;36m";
pub const BRIGHT_WHITE: &str = "\x1b[1;37m";
pub const BLACK_OVER_WHITE: &str = "\x1b[30m\x1b[1;47m";
// Reset code to reset styling
pub const RESET: &str = "\x1b[0m";

pub fn geticon(filename: &String, isdir: bool, is_sym: bool) -> String {
    match filename {
        _ if isdir && filename.starts_with('.') => CONF_FOLDER_ICO.to_string(),
        _ if isdir => FOLDER_ICO.to_string(),
        _ if is_sym || filename.ends_with(".lnk") => SYMLINK_ICO.to_string(),
        _ if filename.ends_with(".exe") => EXE_ICO.to_string(),
        _ if filename.starts_with('.') => CONF_ICO.to_string(),
        _ if filename.ends_with(".py") => PYTHON_ICO.to_string(),
        _ if filename.ends_with(".rs") => RUST_ICO.to_string(),
        _ if filename.ends_with(".toml") => TOML_ICO.to_string(),
        _ if filename.ends_with(".java") => JAVA_ICO.to_string(),
        _ if filename.ends_with(".html") => HTML_ICO.to_string(),
        _ if filename.ends_with(".md") => MARKDOWN_ICO.to_string(),
        _ if filename.ends_with(".js") => JS_ICO.to_string(),
        _ if filename.ends_with(".c") => C_ICO.to_string(),
        _ if filename.ends_with(".cpp") => CPP_ICO.to_string(),
        _ if filename.ends_with(".go") => GO_ICO.to_string(),
        _ if filename.ends_with(".pdf") => PDF_ICO.to_string(),
        _ if filename.ends_with(".pptx") => PPTTX_ICO.to_string(),
        _ if filename.ends_with(".class") => JAVA_CLASS_ICO.to_string(),
        _ if filename.ends_with(".css") => CSS_ICO.to_string(),
        _ if filename.ends_with(".xml") => XML_ICO.to_string(),
        _ if filename.ends_with(".json") => JSON_ICO.to_string(),
        _ if filename.ends_with(".yaml") || filename.ends_with(".yml") => YAML_ICO.to_string(),
        _ if filename.ends_with(".dockerfile") => DOCKER_ICO.to_string(),
        _ if filename.ends_with(".sql") => SQL_ICO.to_string(),
        _ if filename.ends_with(".vim") => VIM_ICO.to_string(),
        _ if filename.ends_with(".sh") => SHELL_ICO.to_string(),
        _ if filename.ends_with(".tex") => TEX_ICO.to_string(),
        _ if filename.ends_with(".r") => R_ICO.to_string(),
        _ if filename.ends_with(".kt") => KOTLIN_ICO.to_string(),
        _ if filename.ends_with(".scala") => SCALA_ICO.to_string(),
        _ if filename.ends_with(".ts") => TS_ICO.to_string(),
        _ if filename.ends_with(".cs") => CSHARP_ICO.to_string(),
        _ if filename.ends_with(".psd") => PSD_ICO.to_string(),
        _ if filename.ends_with(".jpg") || filename.ends_with(".jpeg") => JPG_ICO.to_string(),
        _ if filename.ends_with(".png") => PNG_ICO.to_string(),
        _ if filename.ends_with(".gif") => GIF_ICO.to_string(),
        _ if filename.ends_with(".bmp") || filename.ends_with(".tiff") => BMP_ICO.to_string(),
        _ if filename.ends_with(".svg") => SVG_ICO.to_string(),
        _ if filename.ends_with(".mp3")
            || filename.ends_with(".wav")
            || filename.ends_with(".flac") =>
        {
            MP3_ICO.to_string()
        }
        _ if filename.ends_with(".mp4")
            || filename.ends_with(".mkv")
            || filename.ends_with(".avi")
            || filename.ends_with(".mov") =>
        {
            MP4_ICO.to_string()
        }
        _ if filename.ends_with(".h") => HEADERFILE_ICO.to_string(),
        _ if filename.ends_with(".zip") || filename.ends_with(".rar") => ZIP_ICO.to_string(),
        _ if filename.ends_with(".tar") || filename.ends_with(".gz") => TAR_ICO.to_string(),
        _ if filename.ends_with(".dmg") => DMG_ICO.to_string(),
        _ if filename.ends_with(".iso") => ISO_ICO.to_string(),
        _ if filename.ends_with(".txt") => TXT_ICO.to_string(),
        _ if filename.ends_with(".doc") || filename.ends_with(".docx") => DOC_ICO.to_string(),
        _ if filename.ends_with(".xls") || filename.ends_with(".xlsx") => XLS_ICO.to_string(),
        _ if filename.ends_with(".csv") => CSV_ICO.to_string(),
        _ if filename.ends_with(".ico") => ICO_ICO.to_string(),
        _ if filename.ends_with(".conf") => CONF_ICO.to_string(),
        _ if filename.ends_with(".reg") => REG_ICO.to_string(),
        _ if filename.ends_with(".bin") || !filename.contains('.') => BIN_ICO.to_string(),
        _ => "".to_string(), // Default icon
    }
}
pub fn get_color(filename: &str, isdir: bool, is_sym: bool) -> String {
    match filename {
        _ if filename.ends_with(".exe") => BRIGHT_RED.to_string(),
        _ if isdir && filename.starts_with('.') => BRIGHT_CYAN.to_string(),
        _ if isdir => BRIGHT_CYAN.to_string(),
        _ if is_sym || filename.ends_with(".lnk") => BRIGHT_MAGENTA.to_string(),
        _ if filename.starts_with('.') => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".py") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".rs") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".toml") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".h") => BRIGHT_RED.to_string(),
        _ if filename.ends_with(".java") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".html") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".md") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".js") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".c") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".cpp") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".go") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".pdf") => BRIGHT_RED.to_string(),
        _ if filename.ends_with(".pptx") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".class") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".css") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".xml") => BRIGHT_WHITE.to_string(),
        _ if filename.ends_with(".json") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".yaml") || filename.ends_with(".yml") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".dockerfile") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".sql") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".vim") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".sh") => BRIGHT_WHITE.to_string(),
        _ if filename.ends_with(".tex") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".r") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".kt") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".scala") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".ts") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".cs") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".psd") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".jpg") || filename.ends_with(".jpeg") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".png") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".gif") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".bmp") || filename.ends_with(".tiff") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".svg") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".mp3")
            || filename.ends_with(".wav")
            || filename.ends_with(".flac") =>
        {
            BRIGHT_RED.to_string()
        }
        _ if filename.ends_with(".mp4")
            || filename.ends_with(".mkv")
            || filename.ends_with(".avi")
            || filename.ends_with(".mov") =>
        {
            BRIGHT_RED.to_string()
        }
        _ if filename.ends_with(".zip") || filename.ends_with(".rar") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".tar") || filename.ends_with(".gz") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".dmg") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".iso") => BRIGHT_WHITE.to_string(),
        _ if filename.ends_with(".txt") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".doc") || filename.ends_with(".docx") => BRIGHT_BLUE.to_string(),
        _ if filename.ends_with(".xls") || filename.ends_with(".xlsx") => BRIGHT_GREEN.to_string(),
        _ if filename.ends_with(".csv") => BRIGHT_CYAN.to_string(),
        _ if filename.ends_with(".ico") => BRIGHT_MAGENTA.to_string(),
        _ if filename.ends_with(".conf") => BRIGHT_RED.to_string(),
        _ if filename.ends_with(".reg") => BRIGHT_YELLOW.to_string(),
        _ if filename.ends_with(".bin") || !filename.contains('.') => BRIGHT_WHITE.to_string(),
        _ => BRIGHT_WHITE.to_string(), // Default color
    }
}
