@ECHO OFF

SET lnk=%APPDATA%\Microsoft\Windows\SendTo\Normalize file name.lnk
SET target=%~dp0normalize.exe
SET args=-r

SET command=^
$WsShell = New-object -ComObject WScript.Shell;^
$shortcut = $WsShell.CreateShortcut('%lnk%');^
$shortcut.TargetPath = '%target%';^
$shortcut.Arguments = '%args%';^
$shortcut.Save();

powershell.exe -command "& {%command%}"
