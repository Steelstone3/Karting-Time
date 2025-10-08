sudo apt install mingw-w64
sudo dnf install mingw64-gcc-c++

cargo build --release
cargo build --target=x86_64-pc-windows-gnu --release
