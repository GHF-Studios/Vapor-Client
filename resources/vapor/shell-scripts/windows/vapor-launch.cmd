@echo off
setlocal EnableExtensions

set "TARGET=x86_64-pc-windows-gnullvm"

set "SCRIPT_DIR=%~dp0"
pushd "%SCRIPT_DIR%\.." >nul 2>nul
if errorlevel 1 goto app_root_from_script_fallback
set "APP_ROOT=%CD%"
popd >nul 2>nul
goto app_root_ready

:app_root_from_script_fallback
set "APP_ROOT=%SCRIPT_DIR%\.."
goto app_root_ready

:app_root_ready

set "VAPOR=%APP_ROOT%\bin\%TARGET%\vapor.exe"
set "INSTALLER=%APP_ROOT%\bin\%TARGET%\vapor-installer.exe"
set "ENTRYPOINT=%APP_ROOT%\bin\%TARGET%\vapor-entrypoint.exe"
set "LOG_DIR=%APP_ROOT%\.vapor\logs"
set "LAUNCH_LOG=%LOG_DIR%\launch-wrapper.log"

if not exist "%LOG_DIR%" mkdir "%LOG_DIR%" >nul 2>nul

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

set "PATH=%APP_ROOT%\bin\%TARGET%;%APP_ROOT%\bin;%PATH%"

set "FORWARD_ARGS="
:collect_args
if "%~1"=="" goto args_done
set "FORWARD_ARGS=%FORWARD_ARGS% ^"%~1^""
shift /1
goto collect_args

:args_done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: launch_target=%LAUNCH_TARGET% app_root="%APP_ROOT%" hold=%HOLD_TERMINAL%
pushd "%APP_ROOT%" >nul 2>nul
if errorlevel 1 if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: could not change directory to app root

echo Vapor launch wrapper
echo   launch target: %LAUNCH_TARGET%
echo   app root: "%APP_ROOT%"
echo   log: "%LAUNCH_LOG%"
echo.

if /I "%LAUNCH_TARGET%"=="installer" goto installer
if /I "%LAUNCH_TARGET%"=="vapor-installer" goto installer
if not exist "%VAPOR%" goto missing_vapor

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

:installer
if not exist "%INSTALLER%" goto missing_installer
if defined FORWARD_ARGS (
    "%INSTALLER%" %FORWARD_ARGS%
) else (
    "%INSTALLER%" install
)
set "STATUS=%ERRORLEVEL%"
goto done

:play
"%VAPOR%" --startup-script default %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:shell
set "VAPOR_PLAIN_INPUT=1"
"%VAPOR%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
set "VAPOR_PLAIN_INPUT="
goto done

:command
"%VAPOR%" "%LAUNCH_TARGET%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:missing_vapor
echo error: expected Vapor executable is missing
echo   target:  %TARGET%
echo   checked: %VAPOR%
echo.
echo This Steam install is missing the platform binary for this launch option.
echo Update or reinstall the app on the selected Steam branch, then try again.
set "STATUS=9009"
goto done

:missing_installer
echo error: expected Vapor Installer executable is missing
echo   target:  %TARGET%
echo   checked: %INSTALLER%
echo.
echo This Steam install is missing the platform installer for this launch option.
echo Update or reinstall the app on the selected Steam branch, then try again.
set "STATUS=9009"
goto done

:missing_entrypoint
echo error: expected Vapor app entrypoint is missing
echo   target:  %TARGET%
echo   checked: %ENTRYPOINT%
set "STATUS=9009"
goto done

:done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: vapor exited with status %STATUS%
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
