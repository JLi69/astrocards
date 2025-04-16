@echo off
Rem this creates a zip file with the astrocards executable inside

set DIR=astrocards-windows

cargo build --release

rmdir /S release\
mkdir release\%DIR%
copy target\release\astrocards.exe release\%DIR%
copy cfg.impfile release\%DIR%
Xcopy /E /I assets\ release\%DIR%\assets\
Xcopy /E /I sets\ release\%DIR%\sets\
cd release\
Rem NOTE: You need 7-zip installed to use this script
"C:\Program Files\7-Zip\7z.exe" a -tzip astrocards-windows.zip %DIR%
cd ..
