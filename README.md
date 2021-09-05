# htmlwalk
a cmdline html DOM parser

```bash
usage: htmlwalk <file.html> [ tag[.class] || tag[#id] || .class || #id ] [ attr=["value"] || text ]  
or cat file.html | htmlwalk [ tag[.class] || tag[#id] || .class || #id ] [ attr=["value"] || text ]  
```

"text" indicates that the text link that is displayed for an anchor is desired. 
e.g. echo '<a>this bit</a>' | htmlwalk f.html a text => this bit

Example usages:
    htmlwalk index.html a href => [https://example.com/, https://example.com/menu, ... ]
    htmlwalk index.html img.icon src => https://example.com/img/icon.png
    htmlwalk index.html div#footer a#contact txt => Contact
    htmlwalk index.html name="sought"  [<a name="sought" href="/sought_link/">You want this!</a>, <div name="sought">foo</div>, ... ]

    # it is possible to drill down into a tag-within-a-tag
    htmlwalk index.html div#about a href => /about.html 

If you want to drill down by tag THEN by attribute:  htmlwalk index.html body div a class="hidden"
If you want to drill down by tag AND attribute:  htmlwalk index.html body.show div#about a.hidden

By attaching the attribute to its tag we can change the sequential processing.

[undefined behaviour]
If you pipe in an html file AND specify index.html at the same time?

Multiple attributes [NOT YET WRITTEN] can be used: 
	echo '<a class="other_class this_class" href="/">foo</a>' | htmlwalk a.this_class.other_class href => /
	echo '<img id="always_have_an_id" alt="bar" class="css_everything" src="/bar.png" />' | htmlwalk img#alway_have_an_id.css_everything src => /bar.png
