set -e


toml set ././Cargo.toml lib.crate-type ["cdylib"] > ././Cargo-local.toml
rm -rf ././Cargo.toml
mv ././Cargo-local.toml ././Cargo.toml
sed -i 's/\"\[cdylib\]\"/\[\"cdylib\"\]/g' ././Cargo.toml

toml set ././Cargo.toml package.name "module" > ././Cargo-local.toml
rm -rf ././Cargo.toml
mv ././Cargo-local.toml ././Cargo.toml

cargo build --manifest-path ././Cargo.toml \
    --target wasm32-unknown-unknown --release

rm -rf ./build
mkdir ./build

mv ././target/wasm32-unknown-unknown/release/module.wasm ./build/bg_module.wasm

# wasm-bindgen ././target/wasm32-unknown-unknown/release/module.wasm --out-dir ./build --out-name bg_module.wasm

wasm-snip ./build/bg_module.wasm -o ./build/snipped_module.wasm
# rm -rf ./build/bg_module.wasm

wasm-opt --asyncify -Os ./build/snipped_module.wasm -o ./build/wrap.wasm
# rm -rf ./build/snipped_module.wasm
