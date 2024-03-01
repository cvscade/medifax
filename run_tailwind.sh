minify_option=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --minify)
            minify_option="--minify"
            shift
            ;;
        *)
            # Ignore any other command line options
            shift
            ;;
    esac
done

rm -rf ui/css/tailwind.css ui/css/temporary.css
echo "@tailwind base;\n@tailwind components;\n@tailwind utilities;" > ui/css/temporary.css
npx tailwindcss -i ui/css/temporary.css -o ui/css/temporary.css $minify_option
mv ui/css/temporary.css ui/css/tailwind.css
