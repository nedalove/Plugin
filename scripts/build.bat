@ECHO OFF
setlocal
set p=%~dp0

cargo build --release --target i686-pc-windows-msvc

endlocal