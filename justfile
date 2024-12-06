default: precommit

precommit:
    cargo fmt
    cargo clippy

new day:
    cp -r dayTemplate day{{day}}
    git add day{{day}}
    sed -i "s/\"daytemplate\"/\"day{{day}}\"/" day{{day}}/Cargo.toml
    sed -i "s/\"dayTemplate\"/\"day{{day}}\"/" day{{day}}/src/main.rs
    sed -i "s/\"dayTemplate\"/\"day{{day}}\",\n    \"dayTemplate\"/" Cargo.toml
    just precommit

run day:
    cargo run -p day{{day}}

watch day:
    cargo watch -- cargo run -p day{{day}}