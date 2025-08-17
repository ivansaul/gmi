use colored::Colorize;
use std::io::stdin;
use topo::draw::draw;
use topo::error::Result;
use topo::helpers::list_csv_files;

fn main() -> Result<()> {
    for (i, path) in list_csv_files()?.into_iter().enumerate() {
        let name = path.file_stem().unwrap().to_string_lossy();
        match draw(&path) {
            Ok(_) => println!("[{}] {}.csv {}", i + 1, name, "✔".green().bold()),
            Err(e) => {
                println!("\n[{}] {}.csv {}", i + 1, name, "✘".red().bold());
                println!(" {}: {}", "error".red().bold(), e);
            }
        }
    }
    print!(
        "\n{}\n",
        " Made with ❤︎ by @ivansaul [ihuamanis@uni.pe]"
            .purple()
            .italic()
    );
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    Ok(())
}
