# ORION CLI Custom Project

### DEV NOTES 030824:
- Still does not compile and uses depracated features of Clap @TOFIX
- Make sure that main function of other library can be passed in.
- Colorize dat der welcome image.

**Welcome to the Orion Project by Synavate Labs**

```plaintext
:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:::::       ____   _____   _____  ____   __            :::::
:::::/ __ \ |  __ \ |_   _|/ __ \ | \ | |           :::::
:::::| |  | || |__) |  | | | |  | ||  \| |           :::::
:::::| |  | ||  _  /   | | | |  | || . ` |           :::::
:::::| |__| || | \ \  _| |_| |__| || |\  |           :::::
:::::\____/ |_|  \_\|_____|\____/ |_| \_|           :::::
:::::_____  _       _____                        :::::
:::::/ ____|| |     |_   _|                      :::::
:::::| |     | |       | |                        :::::
:::::| |____ | |____  _| |_                      :::::
:::::\_____||______||_____|                      :::::
:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
```

### Features:

🚀 Import into any library or project
🔧 Run your main function from the CLI
🧪 More to come, this is mostly an experimental project
❌ Environtment Error Handling built in

### Config
`Cargo.toml`
```toml
[package]
name = "orion_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
colored = "2.0"
clap = { version = "4.0", features = ["derive"] }
thiserror = "1.0"
env_logger = "0.11.5"

[lib]
name = "orion_cli"
path = "src/lib.rs"

[[bin]]
name = "orion_cli"
path = "src/main.rs"
```

**Example:**

`cli`
```
#Config Logging

export MY_LOG=debug
export MY_LOG_STYLE=auto
export RUST_LOG = {DEFAULT_LOG_LEVEL} #Error, Debug, Info, Critical
```


Your Library or Binary:
`rust`
```
// Stuff and things

pub async fn main():
    orion_cli::orion_main()
    
    // Your code here brah!

    OK((Legen...daryStatus)) 
```

----------------
Thanks for coming by! 👋

**- Snyata**
***core@synavate.tech***





