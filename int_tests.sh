#!/bin/sh
#int_test.sh ver. 20201112171841 Copyright 2020 alexx, MIT License
# rdfa:deps="[./html2xpath]"

# integration tests

usage(){
    printf "Usage: $(basename $0) [-h]
    -h  This help message
    \n";
    exit 0
}

[ "$1" ]&& echo "$1"|grep -q '\-h' && usage

#<a href="newsfaq.html">FAQ</a>
#| <img alt="test me" class="center head_img" src="/img/blank.gif" />

working(){
htmlwalk test.html a href > A
./html2xpath test.html a href > B
diff A B|grep . || echo "$(wc -l A B)"

echo "[test.html a href|wc -l]"
./target/release/htmlwalk test.html a href 2>/dev/null|wc -l
./html2xpath test.html a href|wc -l
}

echo "[test.html a|head -n1]"
./target/release/htmlwalk test.html a|head -n1
./html2xpath test.html a|head -n1

exit 0
advanced() {
echo "[test.html a \$file]"
./target/release/htmlwalk a href test.html|head -n1
./html2xpath a href test.html|head -n1
}

specific() {
# if an attr is specified with attr="value" then the parent tag is wanted
htmlwalk test.html a id="up_25057794"
./html2xpath test.html a id="up_25057794"
}

next() {
echo "[a href text] ?? can we do this without sed?" 
./target/release/htmlwalk test.html a|head -n1|sed 's,</.*,,;s,.*>,,'
./html2xpath test.html a|head -n1|sed 's,</.*,,;s,.*>,,'

echo "[a href]"
./target/release/htmlwalk test.html a href 2>/dev/null|wc -l
echo "[img]"
./target/release/htmlwalk test.html img 2>/dev/null
echo "[img src]"
./target/release/htmlwalk test.html img src 2>/dev/null


echo "[a href text]"

echo "[img]"
./target/release/htmlwalk test.html img 2>/dev/null
./html2xpath test.html img 2>/dev/null
echo "[img class.head_img]"
./target/release/htmlwalk test.html img class.head_img 2>/dev/null
./html2xpath test.html img class.head_img 2>/dev/null
echo "[class.head_img]"
./target/release/htmlwalk test.html class.head_img 2>/dev/null
./html2xpath test.html class.head_img 2>/dev/null
echo "[.head_img]"
./target/release/htmlwalk test.html .head_img 2>/dev/null
./html2xpath test.html .head_img 2>/dev/null

}
