use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use walkdir::{DirEntry, WalkDir};

/// ACA Snapshotter - cria snapshot de estrutura + conteúdo de arquivos para LLMs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pasta base para gerar snapshot (default: .)
    #[arg(short, long, default_value = ".")]
    dir: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let root_dir = PathBuf::from(args.dir);
    let mut output = File::create("snapshot.md")?;
    let mut failed_files = Vec::new();

    writeln!(output, "<file_map>")?;
    print_dir_tree(&root_dir, &root_dir, 0, &mut output)?;
    writeln!(output, "</file_map>\n")?;

    writeln!(output, "<file_contents>")?;
    print_file_contents(&root_dir, &mut output, &mut failed_files)?;
    writeln!(output, "</file_contents>\n")?;

    // NOVO: Bloco de arquivos que falharam
    if !failed_files.is_empty() {
        writeln!(output, "<files_failed_to_read>")?;
        for file in failed_files {
            writeln!(output, "{}", file)?;
        }
        writeln!(output, "</files_failed_to_read>")?;
    }

    println!("✔️ Snapshot gerado em snapshot.md");
    Ok(())
}

/// Ignorar pastas lixo
fn is_ignored_dir(entry: &DirEntry) -> bool {
    let ignored = [
        "__pycache__", ".ruff_cache", ".venv", ".pytest_cache", ".git", "target",
    ];
    entry.file_type().is_dir()
        && entry
            .file_name()
            .to_str()
            .map(|s| ignored.contains(&s))
            .unwrap_or(false)
}

fn print_dir_tree(
    _base_path: &Path,
    path: &Path,
    level: usize,
    output: &mut File,
) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|e| {
            let binding = e.file_name();
            let name = binding.to_string_lossy();
            !["__pycache__", ".ruff_cache", ".venv", ".pytest_cache", ".git", "target"]
                .contains(&name.as_ref())
        })        
        .collect::<Vec<_>>();

    let entries_len = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let is_last = i == entries_len - 1;

        let mut prefix = String::new();
        if level > 0 {
            prefix.push_str(&"│   ".repeat(level - 1));
            prefix.push_str(if is_last { "└── " } else { "├── " });
        }

        writeln!(output, "{}{}", prefix, name)?;

        if path.is_dir() {
            print_dir_tree(_base_path, &path, level + 1, output)?;
        }
    }

    Ok(())
}

fn print_file_contents(
    root: &Path,
    output: &mut File,
    failed_files: &mut Vec<String>,
) -> io::Result<()> {
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| !is_ignored_dir(e))
    {
        let path = entry.path();
        let display_path = path.strip_prefix(root).unwrap_or(path);
        let display_path_str = display_path.display().to_string().replace("\\", "/");
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("txt");

        // Tenta ler o arquivo
        match fs::read_to_string(path) {
            Ok(contents) => {
                writeln!(output, "<path: {}>\n", display_path_str)?;
                writeln!(output, "```{}\n{}\n```\n", ext, contents)?;
                writeln!(output, "</path: {}>\n", display_path_str)?;
            }
            Err(_) => {
                // Se falhar, adiciona na lista
                failed_files.push(display_path_str);
            }
        }
    }
    Ok(())
}
