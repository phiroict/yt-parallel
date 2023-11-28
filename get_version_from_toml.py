import toml

with open('Cargo.toml', 'r') as f:
    config = toml.load(f)

print(config["package"]["version"])