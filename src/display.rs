pub struct Display;

impl Display {
    pub fn banner(code: &str, app: &str, phone: &str) {
        println!();
        println!("┌─────────────────────────────────────┐");
        println!("│         USSD Session Simulator       │");
        println!("└─────────────────────────────────────┘");
        println!("  📞 Code  : {}", code);
        println!("  🌐 App   : {}", app);
        println!("  📱 Phone : {}", phone);
        println!("{}", "─".repeat(41));
    }

    pub fn menu(message: &str) {
        println!();
        println!("{}", message);
        println!("{}", "─".repeat(41));
    }

    pub fn end(message: &str) {
        println!();
        println!("{}", message);
        println!("{}", "─".repeat(41));
        println!("✅ Session ended.");
        println!();
    }

    pub fn error(message: &str) {
        eprintln!();
        eprintln!("❌ Error: {}", message);
        eprintln!();
    }

    pub fn step(step: u32, text: &str) {
        println!("  [Step {}] Input so far: \"{}\"", step, text);
    }

    pub fn log_saved(path: &str) {
        println!("  💾 Session log saved: {}", path);
    }

    pub fn replay_banner(code: &str, app: &str, phone: &str, total_steps: usize) {
        println!();
        println!("┌─────────────────────────────────────┐");
        println!("│         USSD Session Replay          │");
        println!("└─────────────────────────────────────┘");
        println!("  📞 Code        : {}", code);
        println!("  🌐 App         : {}", app);
        println!("  📱 Phone       : {}", phone);
        println!("  🔢 Total Steps : {}", total_steps);
        println!("{}", "─".repeat(41));
    }

    pub fn replay_step(
        step: u32,
        text_sent: &str,
        user_input: Option<&str>,
        response: &str,
    ) {
        println!();
        println!("  [Step {}]", step);
        println!("  📤 Sent  : \"{}\"", text_sent);
        println!("  📥 Got   : {}", response);
        if let Some(input) = user_input {
            println!("  ⌨️  Input : \"{}\"", input);
        }
        println!("{}", "─".repeat(41));
    }

    pub fn replay_done() {
        println!();
        println!("✅ Replay complete.");
        println!();
    }

    pub fn config_saved() {
        println!();
        println!("✅ Config saved.");
        println!();
    }
}