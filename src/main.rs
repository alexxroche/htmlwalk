// Copyright 2020 alexx
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    htmlwalk::walk();
    //htmlwalk::walk(&html_tag, &attr); // this is less ambiguous
    //walk(&html_tag, &attr); // requires `use htmlwalk::walk;`
}

/*
   <a href="#test1">How do we get the anchor text?</a>
html2xpath index.html img
<img alt="test me" class="center head_img" src="/img/blank.gif" />
html2xpath index.html img src
/img/blank.gif

*/
