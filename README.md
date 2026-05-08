# ussd-sim 📞

A production-grade USSD session simulator for developers. Test your USSD applications locally without a real SIM card, phone, or telecom connection.

---

## What is USSD?

USSD (Unstructured Supplementary Service Data) is the technology behind codes like `*737#`, `*901#`, and `*99#` that you dial on your phone to access banking, airtime, and other services.

When you dial a USSD code:

```
Your Phone → MTN/Airtel Network → Your App (HTTP request)
                                          ↓
Your Phone ← MTN/Airtel Network ← Your App (HTTP response)
```

The telecom gateway sends an HTTP request to your app and your app responds with either:
- `CON <message>` — continue the session and show a menu
- `END <message>` — end the session and show a final message

**The problem:** To test a USSD app today, you need a real SIM card, a telco sandbox account, and a live network connection. This makes local development slow and painful.

**The solution:** `ussd-sim` replaces the telecom gateway locally — so you can test your full USSD flow from your terminal in seconds.

---

## Features

- 📞 **Interactive sessions** — dial any USSD code and interact with your app in real time
- 💾 **Session logging** — every session is saved to a JSON file automatically
- 🔁 **Replay mode** — replay any saved session against your app automatically
- ⚙️ **Config file** — save your default app URL, phone number, and settings
- 🛡️ **Max steps protection** — detects and stops infinite loops
- ⏱️ **Timeout handling** — configurable request timeout per session
- 🔍 **Debug tracing** — verbose mode shows every request and response
- 🦀 **Written in Rust** — fast, reliable, zero runtime overhead

---

## Installation

### From source

Make sure you have [Rust](https://rustup.rs) installed, then:

```bash
git clone https://github.com/GOODBADBOY10/ussd-sim
cd ussd-sim
cargo install --path .
```

### From crates.io (coming soon)

```bash
cargo install ussd-sim
```

---

## Quick Start

### 1. Start your USSD app

Your app needs to accept POST requests with this JSON body:

```json
{
  "sessionId": "SIM-abc123",
  "serviceCode": "*737#",
  "phoneNumber": "+2348000000000",
  "text": ""
}
```

And respond with either:
```
CON Welcome to MyApp
1. Option One
2. Option Two
```
or:
```
END Thank you for using MyApp
```

### 2. Set your app URL (once)

```bash
ussd-sim config --set-url http://localhost:3000/ussd
```

### 3. Dial

```bash
ussd-sim dial --code "*737#"
```

### 4. Interact

```
┌─────────────────────────────────────┐
│         USSD Session Simulator       │
└─────────────────────────────────────┘
  📞 Code  : *737#
  🌐 App   : http://localhost:3000
  📱 Phone : +2348000000000
─────────────────────────────────────────
Welcome to TestBank
1. Check Balance
2. Transfer
3. Buy Airtime
─────────────────────────────────────────
  [Step 1] Input so far: ""
Enter input: 2

Enter account number:
─────────────────────────────────────────
  [Step 2] Input so far: "2"
Enter input: 0123456789

Enter amount:
─────────────────────────────────────────
  [Step 3] Input so far: "2*0123456789"
Enter input: 5000

Transfer successful!
─────────────────────────────────────────
✅ Session ended.
  💾 Session log saved: ~/.local/share/ussd-sim/logs/session_20240101_120000_abc12345.json
```

---

## Commands

### `dial` — Start an interactive session

```bash
ussd-sim dial --code "*737#"
```

| Flag | Description | Default |
|------|-------------|---------|
| `--code` | USSD code to dial e.g. `*737#` | required |
| `--app` | URL of your USSD app | from config |
| `--phone` | Phone number to simulate | from config or `+2348000000000` |
| `--max-steps` | Max steps before stopping | from config or `20` |
| `--timeout` | Request timeout in seconds | from config or `30` |
| `--save-log` | Save session log to file | `true` |

**Examples:**

```bash
# Basic dial
ussd-sim dial --code "*737#"

# Override app URL for this session only
ussd-sim dial --code "*737#" --app http://localhost:5000/ussd

# Simulate a specific phone number
ussd-sim dial --code "*737#" --phone "+2348012345678"

# Set a low max steps for testing loop protection
ussd-sim dial --code "*737#" --max-steps 3

# Dial without saving a log
ussd-sim dial --code "*737#" --save-log false

# Enable verbose debug output
ussd-sim --verbose dial --code "*737#"
```

---

### `replay` — Replay a saved session

Replays a previously saved session against your app automatically — useful for regression testing.

```bash
ussd-sim replay --file ~/.local/share/ussd-sim/logs/session_20240101_120000_abc12345.json
```

| Flag | Description | Default |
|------|-------------|---------|
| `--file` | Path to session log JSON file | required |
| `--app` | Override the app URL from the log | from log file |
| `--delay-ms` | Delay between steps in milliseconds | `500` |

**Examples:**

```bash
# Basic replay
ussd-sim replay --file ./session_abc123.json

# Replay against a different app URL
ussd-sim replay --file ./session_abc123.json --app http://staging.myapp.com/ussd

# Speed up replay (no delay)
ussd-sim replay --file ./session_abc123.json --delay-ms 0
```

---

### `config` — Manage configuration

```bash
ussd-sim config --show
```

| Flag | Description |
|------|-------------|
| `--show` | Show current config |
| `--set-url` | Set default app URL |
| `--set-phone` | Set default phone number |
| `--set-max-steps` | Set default max steps |
| `--set-timeout` | Set default timeout in seconds |
| `--set-log-dir` | Set custom log directory |

**Examples:**

```bash
# Show current config
ussd-sim config --show

# Set default app URL
ussd-sim config --set-url http://localhost:3000/ussd

# Set default phone number
ussd-sim config --set-phone "+2348012345678"

# Set custom log directory
ussd-sim config --set-log-dir /home/user/my-ussd-logs

# Set multiple values at once
ussd-sim config --set-url http://localhost:3000 --set-max-steps 10 --set-timeout 15
```

Config is stored at: `~/.config/ussd-sim/config.toml`

---

## Session Logs

Every session is automatically saved as a JSON file at:
```
~/.local/share/ussd-sim/logs/session_<timestamp>_<id>.json
```

Example log file:

```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "service_code": "*737#",
  "phone_number": "+2348000000000",
  "app_url": "http://localhost:3000",
  "started_at": "2024-01-01T12:00:00+01:00",
  "ended_at": "2024-01-01T12:00:15+01:00",
  "completed": true,
  "steps": [
    {
      "step": 1,
      "text_sent": "",
      "response": "CON Welcome to TestBank\n1. Check Balance\n2. Transfer",
      "user_input": "2"
    },
    {
      "step": 2,
      "text_sent": "2",
      "response": "CON Enter account number:",
      "user_input": "0123456789"
    },
    {
      "step": 3,
      "text_sent": "2*0123456789",
      "response": "END Transfer successful!",
      "user_input": null
    }
  ]
}
```

---

## Building a USSD App to Test With

Your app just needs to handle POST requests. Here is a minimal example in Node.js:

```javascript
const http = require("http");

const server = http.createServer((req, res) => {
  let body = "";
  req.on("data", chunk => body += chunk);
  req.on("end", () => {
    if (!body) {
      res.writeHead(400);
      res.end("END Bad request");
      return;
    }

    let parsed;
    try {
      parsed = JSON.parse(body);
    } catch (e) {
      res.writeHead(400);
      res.end("END Bad request");
      return;
    }

    const { text } = parsed;
    let response;

    if (text === "") {
      response = "CON Welcome\n1. Balance\n2. Transfer\n3. Airtime";
    } else if (text === "1") {
      response = "END Your balance is ₦10,000";
    } else if (text === "2") {
      response = "CON Enter account number:";
    } else if (text.startsWith("2*") && text.split("*").length === 2) {
      response = "CON Enter amount:";
    } else if (text.startsWith("2*") && text.split("*").length === 3) {
      response = "END Transfer successful!";
    } else if (text === "3") {
      response = "CON Enter phone number:";
    } else if (text.startsWith("3*")) {
      response = "END Airtime purchase successful!";
    } else {
      response = "END Invalid option";
    }

    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end(response);
  });
});

server.listen(3000, () => console.log("Running on http://localhost:3000"));
```

---

## How USSD Text Accumulates

Each time the user makes a selection, it is appended to the `text` field separated by `*`:

| Step | User Input | `text` sent to app |
|------|------------|--------------------|
| 1 | _(dials code)_ | `""` |
| 2 | `2` | `"2"` |
| 3 | `0123456789` | `"2*0123456789"` |
| 4 | `5000` | `"2*0123456789*5000"` |

This is the standard format used by all Nigerian telecom USSD gateways (MTN, Airtel, Glo, 9mobile).

---

## Debug Mode

Enable verbose tracing to see every request and response:

```bash
ussd-sim --verbose dial --code "*737#"
```

Or set the environment variable directly:

```bash
RUST_LOG=debug ussd-sim dial --code "*737#"
```

---

## Project Structure

```
ussd-sim/
├── src/
│   ├── main.rs       # Entry point and command routing
│   ├── cli.rs        # CLI argument definitions and validation
│   ├── error.rs      # Custom error types
│   ├── session.rs    # USSD session loop logic
│   ├── http.rs       # HTTP client for calling the app
│   ├── display.rs    # Terminal output formatting
│   ├── config.rs     # Config file management
│   ├── logger.rs     # Session log saving
│   └── replay.rs     # Session replay logic
```

---

## Contributing

Contributions are welcome! Please open an issue first to discuss what you would like to change.

```bash
git clone https://github.com/GOODBADBOY10/ussd-sim
cd ussd-sim
cargo build
cargo test
```

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

## Author

Built by [Ademola](https://github.com/GOODBADBOY10)
