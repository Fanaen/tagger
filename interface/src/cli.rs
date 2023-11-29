use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tagger")]
#[command(bin_name = "tagger")]
pub struct TaggerCli {
    #[command(subcommand)]
    pub command: TaggerCommand,
}

#[derive(Debug, Subcommand)]
pub enum TaggerCommand {
    /// To change the file name, add date or time, add and/or remove tags.
    ///
    /// Intended to mirror these tools :
    ///  * https://github.com/novoid/filetags
    ///  * https://github.com/novoid/appendfilename
    ///  * https://github.com/novoid/date2name
    #[clap(verbatim_doc_comment)]
    Edit {
        /// Add text after the date and before the file name
        #[arg(short = 'p', long)]
        prefix: Option<String>,
        /// Add text after the file name and before tag separator (-- by default)
        #[arg(short = 's', long)]
        suffix: Option<String>,
        /// Add a tag after the tag separator (-- by default)
        #[arg(short = 'a', long)]
        add_tag: Vec<String>,
        /// Remove a tag after the tag separator (-- by default) if it's there
        #[arg(short = 'r', long)]
        remove_tag: Vec<String>,
        /// Add the date (without time) before the filename
        #[arg(short = 'd', long)]
        add_date: bool,
        /// Add the date and the time before the filename
        #[arg(short = 't', long)]
        add_datetime: bool,
        /// Add the time (without the date) before the filename
        #[arg(long)]
        add_time: bool,
        /// The files/folders to edit.
        targets_path: Vec<String>,
    },
    /// To maintain a background presence and get global shortcuts up and running.
    Daemon {
        // TODO
    },
}

pub fn parse() -> TaggerCli {
    // Wild allows us to get filename expansion on Windows too.
    let args = wild::args();
    TaggerCli::parse_from(args)
}
