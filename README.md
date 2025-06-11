## yaml-parser
このリポジトリは、YAMLファイルをパースするためのシンプルなツールを提供します。

### 概要
yaml-parserは、指定されたYAMLファイルを読み込み、その内容をRustの構造体に変換します。これにより、YAML設定を簡単にアプリケーションで利用できるようになります。

### インストール
このクレートはCargoを通して利用できます。Cargo.tomlに以下を追加してください。

```TOML
[dependencies]
yaml-parser = "0.1.0" # 最新のバージョンを指定してください
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
```

### 使い方
yaml-parserを使ってYAMLファイルをパースする基本的な方法は以下の通りです。

```Rust

use serde::Deserialize;
use std::fs;
use std::io;

// 設定をマッピングするための構造体を定義します
#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    port: u16,
}

fn main() -> io::Result<()> {
    // YAMLファイルのパス
    let file_path = "config.yaml"; // このファイルはあなたのプロジェクトに存在する必要があります

    let contents = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&contents)
        .expect("YAMLファイルのパースに失敗しました");

    println!("データベースホスト: {}", config.database.host);
    println!("サーバーポート: {}", config.server.port);

    Ok(())
}
```

#### config.yaml の例
上記のRustコードでパースするconfig.yamlファイルの例です。

```YAML

database:
  host: localhost
  port: 5432
  username: admin
  password: your_password
server:
  port: 8080
```

### 機能
YAMLファイルの読み込みとパース
カスタムRust構造体へのYAMLコンテンツのマッピング
エラーハンドリング

