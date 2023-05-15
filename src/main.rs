extern crate skim;
use clap::Parser;
use pinyin::ToPinyin;
use skim::prelude::*;

use std::path::Path;

struct PinyinItem {
    inner: String,
}
fn to_pinyin(path: &str) -> String {
    let mut converted: Vec<String> = vec![];
    let chars: Vec<char> = path.chars().collect();
    for (idx, lst) in path.to_pinyin().enumerate() {
        if let Some(lst) = lst {
            converted.push(lst.plain().to_string());
        } else {
            converted.push(chars[idx].to_string());
        }
    }
    converted.join("")
}

impl SkimItem for PinyinItem {
    fn text(&self) -> Cow<str> {
        Cow::Owned(to_pinyin(&self.inner))
    }
    fn display<'a>(&'a self, _context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::parse(&self.inner)
    }
    fn output(&self) -> Cow<str> {
        Cow::Borrowed(&self.inner)
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: Option<String>,

    #[arg(short, long)]
    show_ignore: bool,

    #[arg(short, long)]
    multi: bool,

    #[arg(short, long)]
    dironly: bool,
}

pub fn main() {
    let cli = Cli::parse();
    let path = Path::new(cli.path.as_deref().unwrap_or("."));
    let show_ignore = cli.show_ignore;

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for entry in path.read_dir().expect("Read dir failed") {
        let entry = entry.unwrap();
        if let Some(entry_fn) = entry.path().file_name() {
            if cli.dironly && !entry.path().is_dir() {
                continue;
            }
            let entry_fn = entry_fn.to_str().unwrap();
            if !show_ignore && entry_fn.starts_with(".") {
                continue;
            }
            let _ = tx_item.send(Arc::new(PinyinItem {
                inner: path.join(entry_fn).to_str().unwrap().to_string(),
            }));
        }
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(cli.multi)
        .build()
        .unwrap();

    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        println!("{}", item.output());
    }
}
