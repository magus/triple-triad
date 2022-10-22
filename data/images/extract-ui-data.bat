@echo off

SET GAMEPATH="C:\Program Files (x86)\SquareEnix\FINAL FANTASY XIV - A Realm Reborn"
SET REPOPATH="C:\Users\noah\github\xiv-data"
SET FFMPEG="C:\Users\noah\software\ffmpeg-5.0.1-full_build\bin\ffmpeg.exe"
SET /p VERSION=<%GAMEPATH%\game\ffxivgame.ver
SET DATAPATH="%CD%\%VERSION%"

ECHO Setting SC definition to the latest game version...
COPY %GAMEPATH%\game\ffxivgame.ver Definitions\game.ver

ECHO [%TIME%] Extracting images...

@REM paths discovered via TexTools search for "triad"
.\SaintCoinach.Cmd.exe %GAMEPATH% "image ui/uld/cardtripletriad_hr1.tex"
.\SaintCoinach.Cmd.exe %GAMEPATH% "image ui/uld/tripletriadbattle_hr1.tex"
.\SaintCoinach.Cmd.exe %GAMEPATH% "image ui/uld/tripletriadbg_hr1.tex"
.\SaintCoinach.Cmd.exe %GAMEPATH% "image ui/uld/tripletriadpanel_hr1.tex"

@REM discovered by exporting all files and manually searching
@REM .\SaintCoinach.Cmd.exe %GAMEPATH% "uihd 000000 999999"
.\SaintCoinach.Cmd.exe %GAMEPATH% "uihd 071301 071302"
.\SaintCoinach.Cmd.exe %GAMEPATH% "uihd 076531 076549"
.\SaintCoinach.Cmd.exe %GAMEPATH% "uihd 082100 082500"
.\SaintCoinach.Cmd.exe %GAMEPATH% "uihd 121600 121630"

for /d %%i in (%DATAPATH%\ui\icon\*) do (cd "%%i" & rmdir /S /Q hq 2>NUL)
CD "%DATAPATH%\.."

ECHO [%TIME%] Compressing images...
"C:\Program Files\7-Zip\7z.exe" a %DATAPATH%\ui.zip %DATAPATH%\ui\* > NUL


ECHO [%TIME%] Extract complete.
pause
