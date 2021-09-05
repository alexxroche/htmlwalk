#!/bin/sh
#do.sh ver. 20201112171841 Copyright 2020 alexx, MIT License
# rdfa:deps="[cargo rustc]"

usage(){
    printf "Usage: $(basename $0) [-h]
    -h  This help message
    \n";
    exit 0
}

[ "$1" ]&& echo "$1"|grep -q '\-h' && usage

cargo build --release && {
#htmlwalk test.html a|tail -n1
#htmlwalk test.html a text|tail -n1
echo "htmlwalk 2.html a"
htmlwalk 2.html a -v2
echo "[i] and with html2xpath"
html2xpath 2.html a
#./int_tests.sh
}
