# 📦 snapshotter

Um utilitário CLI em Rust para gerar snapshots estruturados de projetos de código, prontos para serem enviados para LLMs (Large Language Models).

## ✅ O que ele faz

- Gera a árvore de diretórios (`<file_map>`)
- Coleta o conteúdo de todos os arquivos (`<file_contents>`)
- Separa arquivos problemáticos (`<files_failed_to_read>`)
- Ignora automaticamente pastas lixo (`__pycache__`, `.venv`, etc.)
- Compatível com qualquer projeto (Python, Rust, JS, etc.)

## 💻 Como usar

### Instalação

Clone e compile:
```bash
git clone https://github.com/seu-usuario/snapshotter.git
cd snapshotter
cargo build --release
