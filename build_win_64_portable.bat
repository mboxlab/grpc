@echo off
set file_name=grpc.exe

set target=x86_64-pc-windows-msvc
set target_dir=%cd%\target\%target%\release
set out=%cd%\grpc.exe

rustup target add %target%
cargo build --release --target=%target%

move %target_dir%\%file_name% %out%
pause