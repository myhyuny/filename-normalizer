@ECHO OFF

IF "%1" == "build" GOTO build
IF "%1" == "install" GOTO install
IF NOT "%1" == "" (
    GOTO command
)

:build
cargo.exe build --release
IF NOT EXIST bin MKDIR bin
COPY /Y target\release\normalize.exe bin
GOTO quit

:install
IF NOT EXIST bin\normalize.exe (
    ECHO Build first
    GOTO quit
)

SET out=%ProgramFiles%\Filename Normalizer\
IF NOT "%2" == "" (
    SET out=%2\
)
IF NOT EXIST "%out%" (
    MKDIR "%out%"
    IF %ERRORLEVEL% == 1 GOTO quit
)
COPY /Y bin\normalize.exe "%out%"
IF %ERRORLEVEL% == 1 GOTO quit
COPY /Y script\uninstall.bat "%out%"
IF %ERRORLEVEL% == 1 GOTO quit

SET target="%out%normalize.exe"
SET args=-r
SET lnk=%APPDATA%\Microsoft\Windows\SendTo\Normalize file name.lnk

SET command=^
$WsShell = New-object -ComObject WScript.Shell;^
$shortcut = $WsShell.CreateShortcut('%lnk%');^
$shortcut.TargetPath = '%target%';^
$shortcut.Arguments = '%args%';^
$shortcut.Save();

powershell.exe -command "& {%command%}"

GOTO quit

:command
cargo.exe %* 
GOTO quit

:quit
