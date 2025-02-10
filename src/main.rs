use anyhow::bail;

use clap::Parser;
use slk::{post_message, read_config, BasicMessage};

#[derive(Debug, clap::Parser)]
struct Options {
    #[clap(long)]
    color: Option<String>,
    #[clap(long)]
    title: Option<String>,
    #[clap(long)]
    title_link: Option<String>,
    #[clap(short, long)]
    text: String,
    #[clap(short = 'C', long, default_value = "default")]
    context: String,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    let config = read_config()?;

    println!("options: {:?}", options);
    println!("config: {:?}", config);

    let context = match config.context.get(&options.context) {
        Some(c) => c,
        None => bail!("could not find context {} in config", options.context),
    };

    let msg = BasicMessage {
        color: options.color,
        title: options.title,
        title_link: options.title_link,
        text: Some(options.text),
    };
    post_message(&context.webhook, msg)?;

    println!("posted message to slack");
    Ok(())
}
