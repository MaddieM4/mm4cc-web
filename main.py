#!/usr/bin/env python3
from pathlib import Path
from shutil import rmtree, copy, copytree

import markdown
import frontmatter
import handlebars

SRC = Path('pages')
DST = Path('site')
STATIC = Path('static')
TMPL = Path('templates')

hbs = handlebars.Compiler()
with open(TMPL / "index.html.hbs", "r", encoding="utf-8") as f:
    tmpl_index = hbs.compile(f.read())

def generate_favicon():
    DST.mkdir(parents=True, exist_ok=True)
    copy(STATIC / "favicon.ico", DST / "favicon.ico")

def generate_static():
    DST.mkdir(parents=True, exist_ok=True)
    copytree(STATIC, DST / "static", dirs_exist_ok=True)

def generate_page(path):
    pread = SRC / path
    pwrite = DST / path.lower().replace('.md', '.html')

    with open(pread, "r", encoding="utf-8") as f:
        page = frontmatter.loads(f.read())
    if page['visibility'] != "public":
        return

    html = tmpl_index({
        'meta': {
            'title': page['title'],
            'visibility': page['visibility'],
            'glu_commands': page.get('glu', default=None),
        },
        'html': markdown.markdown(page.content),
    })

    pwrite.parent.mkdir(parents=True, exist_ok=True)
    with open(pwrite, "w", encoding="utf-8") as f:
        f.write(html)

def generate_all():
    if DST.exists():
        rmtree(DST)    
    generate_favicon()
    generate_static()
    generate_page("Index.md")

generate_all()
