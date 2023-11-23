pub fn clear_terminal() {
    #[cfg(target_os = "windows")]
    print!("{}[2J", 27 as char);
}
