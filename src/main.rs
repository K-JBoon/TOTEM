use std::collections::HashMap;

mod totp;
mod config;

use totp::TOTP;
use config::TokenConfig;

use clap::Clap;
extern crate strfmt;
use strfmt::strfmt;

/// totpgen is a tool for managing and generating TOTP tokens on the command line quickly
/// You can configure your tokens with the CLI interface or directly in your config directory
#[derive(Clap)]
#[clap(version = "0.1", author = "Klaas-Jan Boon <klaas-janboon@live.nl>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    DeleteToken(DeleteToken),
    ListTokens(ListTokens),
    InsertToken(InsertToken),
    GenerateToken(GenerateToken)
}

/// Delete the token with the given ID
#[derive(Clap)]
struct DeleteToken {
    #[clap(short,long)]
    id: String
}

/// List all configured tokens
#[derive(Clap)]
struct ListTokens { }

/// Insert or update a token in your configuration
#[derive(Clap)]
struct InsertToken {
    /// A unique ID for this token
    #[clap(short, long)]
    id: String,
    /// The secret to be used for this TOTP token
    #[clap(short, long)]
    secret: String,
    /// The length to generate for this TOTP token
    #[clap(short, long)]
    digits: usize,
    /// The timestep for this TOTP token
    #[clap(short, long)]
    timestep: u64,
    /// An optional formatting rule for the output of this token
    #[clap(short, long)]
    format: Option<String>
}

/// Generate a token for the given ID and current time
#[derive(Clap)]
struct GenerateToken {
    /// The ID of the token to generate
    input: String,
    /// Ignore the specified formatting for the token in the output
    #[clap(short, long)]
    ignore_formatting: bool
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut cfg = config::get_config().expect("Failed to read config file.");

    match opts.subcmd {
        SubCommand::InsertToken(params) => {
            let token_config = TokenConfig {
                secret: params.secret,
                digits: params.digits,
                timestep: params.timestep,
                format: params.format
            };

            cfg.tokens.insert(params.id, token_config);
            config::store_config(cfg).expect("Failed to write config file.");
        },
        SubCommand::DeleteToken(params) => {
            cfg.tokens.remove(&params.id);
            config::store_config(cfg).expect("Failed to write config file.");
        },
        SubCommand::ListTokens(_) => {
            println!("Token ID: Token Configuration");
            for (key, val) in cfg.tokens.iter() {
                println!("{}: {:?}", key, val);
            }
        },
        SubCommand::GenerateToken(params) => {
            let token_config = cfg.tokens.get(&params.input).expect("No token with the given ID was found.");
            let secret = token_config.secret.clone().into_bytes();
            let totp = TOTP::new(&secret, token_config.digits, token_config.timestep);
            let token = totp.generate_token();

            if !params.ignore_formatting {
                if let Some(format_string) = token_config.format.clone() {
                    let mut vars = HashMap::new();
                    vars.insert("id".to_string(), params.input);
                    vars.insert("secret".to_string(), token_config.secret.clone());
                    vars.insert("digits".to_string(), format!("{}", token_config.digits));
                    vars.insert("timestep".to_string(), format!("{}", token_config.timestep));
                    vars.insert("token".to_string(), token);

                    println!("{}", strfmt(&format_string, &vars).expect("Error formatting token output. Available variables are: {id}, {secret}, {digits}, {timestep}, {token}"));
                } else {
                    println!("{}", token);
                }
            } else {
                    println!("{}", token);
            }
        }
    }
}
