import toml
import subprocess

with open('Cargo.toml', 'r') as f:
    config = toml.load(f)

subprocess.run(["git", "tag", "v"+config["package"]["version"]])
