cargo build --target .cargo/windows-msvc.json -Zjson-target-spec

@echo off
echo select vdisk file="%cd%\smOS.vhd">dprt
echo attach vdisk>>dprt

diskpart /s %cd%\dprt
cd target\windows-msvc\debug

copy kernel.exe T:\boot\
cd ..\..\..\

echo select vdisk file="%cd%\smOS.vhd">dprt
echo detach vdisk>>dprt

diskpart /s %cd%\dprt
del dprt

C:\Users\valium\Desktop\bochs-build\bin\bochs -f bochs.bxrc -debugger