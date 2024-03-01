@echo off
set "minify_option="

:parse_args
if "%~1"=="" goto end_parse_args

if /i "%1"=="--minify" (
    set "minify_option=--minify"
    shift
) else (
    shift
)

goto parse_args

:end_parse_args

del /q ui\css\tailwind.css ui\css\temporary.css
echo @tailwind base;^
@tailwind components;^
@tailwind utilities; > ui\css\temporary.css
npx tailwindcss -i ui\css\temporary.css -o ui\css\temporary.css %minify_option%
move /y ui\css\temporary.css ui\css\tailwind.css