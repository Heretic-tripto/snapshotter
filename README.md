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
````

Binário final estará em:

```
target/release/snapshotter(.exe)
```

### Uso

```bash
snapshotter --dir caminho/para/pasta
```

Se não passar `--dir`, usa a pasta atual:

```bash
snapshotter
```

Exemplo Windows:

```powershell
snapshotter --dir V:/meuprojeto/src
```

## 🎯 Exemplo de saída

````markdown
<file_map>
├── src
│   ├── main.rs
│   └── lib.rs
</file_map>

<file_contents>
<path: src/main.rs>

```rs
fn main() { println!("Hello, world!"); }
```

\</path: src/main.rs>
\</file\_contents>

## 🚀 Contribuição

Pull requests e sugestões são super bem-vindos!

## 📝 Licença

MIT