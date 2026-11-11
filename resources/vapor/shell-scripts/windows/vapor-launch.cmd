@echo off
setlocal EnableExtensions

set "TARGET=x86_64-pc-windows-gnullvm"
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%..") do set "STEAM_ROOT=%%~fI"

if not defined VAPOR_HOME set "VAPOR_HOME=%USERPROFILE%\Vapor"
set "APP_ROOT=%VAPOR_HOME%"
set "VAPOR=%APP_ROOT%\bin\%TARGET%\vapor.exe"
set "ENTRYPOINT=%STEAM_ROOT%\bin\%TARGET%\vapor-entrypoint.exe"
if not exist "%ENTRYPOINT%" set "ENTRYPOINT=%APP_ROOT%\bin\%TARGET%\vapor-entrypoint.exe"
set "LOG_DIR=%APP_ROOT%\data\logs"
set "LAUNCH_LOG=%LOG_DIR%\launch-wrapper.log"

set "HOLD_TERMINAL=0"
if /I "%~1"=="--hold" goto hold_arg
goto launch_target_arg

:hold_arg
set "HOLD_TERMINAL=1"
shift /1

:launch_target_arg
set "LAUNCH_TARGET=%~1"
if "%LAUNCH_TARGET%"=="" goto default_launch_target
shift /1
goto launch_target_ready

:default_launch_target
set "LAUNCH_TARGET=shell"

:launch_target_ready
if not exist "%APP_ROOT%\Vapor-Home.toml" goto missing_vapor
if not exist "%LOG_DIR%" mkdir "%LOG_DIR%" >nul 2>nul
set "PATH=%APP_ROOT%\bin\%TARGET%;%APP_ROOT%\bin;%PATH%"

set "FORWARD_ARGS="
:collect_args
if "%~1"=="" goto args_done
set "FORWARD_ARGS=%FORWARD_ARGS% ^"%~1^""
shift /1
goto collect_args

:args_done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: launch_target=%LAUNCH_TARGET% vapor_home="%APP_ROOT%" steam_root="%STEAM_ROOT%" hold=%HOLD_TERMINAL%
pushd "%APP_ROOT%" >nul 2>nul
if errorlevel 1 if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: could not change directory to Vapor Home

echo Vapor launch wrapper
echo   launch target: %LAUNCH_TARGET%
echo   Vapor Home: "%APP_ROOT%"
echo   log: "%LAUNCH_LOG%"
echo.

if /I "%LAUNCH_TARGET%"=="installer" goto gui
if /I "%LAUNCH_TARGET%"=="vapor-installer" goto gui
if /I "%LAUNCH_TARGET%"=="play" goto play
if /I "%LAUNCH_TARGET%"=="shell" goto shell
if /I "%LAUNCH_TARGET%"=="vapor-shell" goto shell
if /I "%LAUNCH_TARGET%"=="launcher" goto gui
if /I "%LAUNCH_TARGET%"=="studio" goto gui
if /I "%LAUNCH_TARGET%"=="ide" goto gui
if /I "%LAUNCH_TARGET%"=="chooser" goto gui
goto command

:gui
if not exist "%ENTRYPOINT%" goto missing_entrypoint
"%ENTRYPOINT%" "%LAUNCH_TARGET%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:play
if not exist "%ENTRYPOINT%" goto missing_entrypoint
"%ENTRYPOINT%" play %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:shell
if not exist "%VAPOR%" goto missing_cli
set "VAPOR_PLAIN_INPUT=1"
"%VAPOR%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
set "VAPOR_PLAIN_INPUT="
goto done

:command
if not exist "%VAPOR%" goto missing_cli
"%VAPOR%" "%LAUNCH_TARGET%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:missing_vapor
if exist "%ENTRYPOINT%" (
    "%ENTRYPOINT%" chooser
    set "STATUS=%ERRORLEVEL%"
    goto done_no_hold_message
)
echo error: Vapor is not installed for this user.
echo Download Vapor from https://vapor.ghf-studios.site/ and then launch this app again.
set "STATUS=9009"
goto done_no_hold_message

:missing_cli
echo error: the Vapor command is missing from Vapor Home.
echo Checked: "%VAPOR%"
echo Repair Vapor from its installer, then try again.
set "STATUS=9009"
goto done

:missing_entrypoint
echo error: the Vapor launch helper is missing.
echo Checked: "%ENTRYPOINT%"
set "STATUS=9009"
goto done

:done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: exited with status %STATUS%
if not "%HOLD_TERMINAL%"=="1" goto done_no_hold_message
echo.
echo Vapor exited with status %STATUS%.
echo Log: "%LAUNCH_LOG%"
echo Starting an interactive command prompt. Type exit when you are done.
if defined ComSpec (
    "%ComSpec%" /D /K
) else (
    cmd.exe /D /K
)

:done_no_hold_message
popd >nul 2>nul
exit /b %STATUS%
