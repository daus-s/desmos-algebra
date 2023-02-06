sed '/^[ \t\r\n]*$/d' testgrammar.file | while read line; do
    if [[ ${line:0:1} != "#" ]]; then
      echo "$line" | ./TestDesmos
    fi
done