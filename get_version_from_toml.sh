VERSION_DATA=$(cat Cargo.toml  | grep "^version" | tr " = " "\n" |  head -n 4 )
echo "${VERSION_DATA##*$'\n'}" | sed "s/\"//g"