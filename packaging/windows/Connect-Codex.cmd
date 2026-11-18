@echo off
setlocal
echo Updating Craft Anything's Codex connection. Your previous config is backed up.
"%~dp0craft-anything-cli.exe" setup --client codex --yes
if errorlevel 1 goto failed
echo.
echo Connection updated. Close any old Craft Anything window, then open craft-anything.exe from this folder.
pause
exit /b 0
:failed
echo.
echo Setup failed. Keep this window open and share the error above.
pause
exit /b 1
