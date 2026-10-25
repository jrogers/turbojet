"""Assembles the website's sources from the repository's own docs, so the site can't drift from them.

Called by scripts/site.sh, which runs mdBook after it:

- the book's sources, in BOOK/src: the guide's chapters (docs/guide/*.md), with a contents page
  made from the guide's own index, and the architecture overview (docs/ARCHITECTURE.md). A link
  to a file outside the book (DESIGN.md, say) goes to it on GitHub, at REF.
- the landing page, OUT/index.html: site/index.html with the version, the quick start's
  application (from examples/quickstart.rs) and the README's benchmark table filled in.

Usage: python3 site/build.py BOOK OUT REF
"""

import html
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = "https://github.com/jrogers/turbojet"


def main(book: Path, out: Path, ref: str) -> None:
    src = book / "src"
    src.mkdir(parents=True)
    shutil.copy(ROOT / "site" / "book.toml", book / "book.toml")
    # The logo as the book's favicon, in place of mdBook's own.
    (book / "theme").mkdir()
    shutil.copy(ROOT / "docs" / "assets" / "logo.svg", book / "theme" / "favicon.svg")
    shutil.copy(ROOT / "docs" / "assets" / "logo-512.png", book / "theme" / "favicon.png")

    guide = ROOT / "docs" / "guide"
    chapters = []
    # The guide's index is the book's first page, index.html: mdBook would render a README.md
    # there but leave links to it pointing at README.html.
    names = {p.name: ("index.md" if p.name == "README.md" else p.name) for p in guide.glob("*.md")}
    for path in sorted(guide.glob("*.md")):
        text = relink(path.read_text(), "docs/guide", set(names), ref).replace("](README.md)", "](index.md)")
        (src / names[path.name]).write_text(text)
        chapters.append(names[path.name])
    architecture = (ROOT / "docs" / "ARCHITECTURE.md").read_text()
    architecture = architecture.replace("](guide/README.md)", "](index.md)")
    (src / "architecture.md").write_text(relink(architecture, "docs", {"index.md"}, ref))
    (src / "SUMMARY.md").write_text(summary((guide / "README.md").read_text(), chapters))

    out.mkdir(parents=True, exist_ok=True)
    shutil.copy(ROOT / "docs" / "assets" / "logo.svg", out / "logo.svg")
    page = (ROOT / "site" / "index.html").read_text()
    page = page.replace("{{version}}", version())
    page = page.replace("{{quickstart}}", highlight(quickstart()))
    page = page.replace("{{send}}", highlight(send()))
    page = page.replace("{{benchmarks}}", benchmarks())
    for name, value in figures().items():
        page = page.replace("{{" + name + "}}", value)
    assert "{{" not in page, "a placeholder in site/index.html has nothing to fill it"
    (out / "index.html").write_text(page)


def summary(index: str, chapters: list[str]) -> str:
    """The book's contents, in the order the guide's index lists its chapters."""
    lines = ["# Summary", "", "[The Turbojet guide](index.md)", ""]
    targets = dict(re.findall(r"^\[([\w-]+)\]: (\S+\.md)$", index, re.M))
    for title, ref in re.findall(r"^\d+\. \[([^\]]+)\]\[([\w-]+)\]", index, re.M):
        target = targets[ref]
        assert target in chapters, f"the guide's index lists {target}, which isn't a chapter"
        lines.append(f"- [{title}]({target})")
    lines += ["", "---", "", "[Architecture](architecture.md)", ""]
    return "\n".join(lines)


def relink(text: str, directory: str, in_book: set[str], ref: str) -> str:
    """Points relative links to files the book doesn't have at them on GitHub, at `ref`."""

    def github(target: str) -> str:
        path, _, anchor = target.partition("#")
        if path in in_book or "://" in path or path.startswith(("#", "mailto:")) or not path:
            return target
        url = f"{REPO}/blob/{ref}/{Path(directory, path).as_posix()}"
        return f"{url}#{anchor}" if anchor else url

    text = re.sub(r"\]\(([^)\s]+)\)", lambda m: f"]({github(m.group(1))})", text)
    return re.sub(r"^(\[[^\]]+\]): (\S+)$", lambda m: f"{m.group(1)}: {github(m.group(2))}", text, flags=re.M)


def version() -> str:
    manifest = (ROOT / "crates" / "turbojet" / "Cargo.toml").read_text()
    return re.search(r'^version = "([^"]+)"', manifest, re.M).group(1)


def quickstart() -> str:
    """The quick start's order desk: its Application, from examples/quickstart.rs."""
    example = (ROOT / "crates" / "turbojet" / "examples" / "quickstart.rs").read_text()
    start = example.index("/// Acknowledges every order.")
    end = example.index("\n}\n", example.index("impl Application for OrderDesk", start)) + 2
    return example[start:end]


def figures() -> dict[str, str]:
    """The landing page's headline numbers, read from the README so they move with it. A README
    rewording that loses one fails the build rather than leaving a stale number."""
    readme = (ROOT / "README.md").read_text()

    def find(pattern: str) -> re.Match:
        match = re.search(pattern, readme)
        assert match, f"README.md no longer has {pattern!r}: update site/build.py's figures"
        return match

    passed = find(r"passes (\d+) of QuickFIX's (\d+) scripted session acceptance scenarios")
    for stage in ("Decode", "Session", "Application"):
        # The allocation table's rows: "| Decode (into one message ...) | 0 | 0 | 0 |".
        row = find(rf"(?m)^\| {stage} \([^|]*\| ([^|]+)\|")
        assert row.group(1).strip() == "0", f"the {stage} stage allocates now: update the landing page"
    return {
        "order_to_ack": find(r"\| Session: order → ack, wire to wire[^|]*\| ([^|]+?) \|").group(1),
        "round_trip": find(r"\| Round trip over localhost TCP, one at a time, replying from `on_message` \| ([^|]+?) \|").group(1),
        "scenarios_passed": passed.group(1),
        "scenarios": passed.group(2),
    }


RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "else", "enum", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "trait", "type", "use", "where", "while",
}


def highlight(code: str) -> str:
    """Rust as HTML with spans for comments, strings, keywords, types and macros: enough for the
    landing page's one example, without a highlighting library."""
    token = re.compile(r'(?P<comment>//[^\n]*)|(?P<string>"(?:\\.|[^"\\])*")|(?P<word>[A-Za-z_][A-Za-z0-9_]*!?)|(?P<other>.)', re.S)
    out = []
    for m in token.finditer(code):
        text = html.escape(m.group(0), quote=False)
        if m.group("comment"):
            out.append(f'<span class="c">{text}</span>')
        elif m.group("string"):
            out.append(f'<span class="s">{text}</span>')
        elif m.group("word"):
            word = m.group("word")
            if word.endswith("!"):
                out.append(f'<span class="m">{text}</span>')
            elif word in RUST_KEYWORDS:
                out.append(f'<span class="k">{text}</span>')
            elif word[0].isupper():
                out.append(f'<span class="t">{text}</span>')
            else:
                out.append(text)
        else:
            out.append(text)
    return "".join(out)


def send() -> str:
    """The quick start's client sending its order, from examples/quickstart.rs, dedented."""
    example = (ROOT / "crates" / "turbojet" / "examples" / "quickstart.rs").read_text()
    start = example.rindex("\n", 0, example.index("let mut order = NewOrderSingle::new")) + 1
    end = example.index("\n", example.index("let seq = session.send(order)?.await?;")) + 1
    lines = example[start:end].split("\n")
    return "\n".join(line[4:] for line in lines).rstrip() + "\n"


def benchmarks() -> str:
    """The README's benchmark table, and the line saying what it was measured on, as HTML."""
    readme = (ROOT / "README.md").read_text()
    section = readme[readme.index("## Benchmarks\n") :]
    machine = next(line for line in section.split("\n") if line.startswith("Apple ") or "criterion medians" in line)
    rows = []
    for line in section.split("\n"):
        if line.startswith("| Benchmark"):
            rows = []
        if line.startswith("|") and rows is not None:
            rows.append(line)
        elif rows:
            break
    header, _, *body = rows
    cell = lambda c: inline(c.strip())
    out = ["<table>", "<thead><tr>" + "".join(f"<th>{cell(c)}</th>" for c in cells(header)) + "</tr></thead>", "<tbody>"]
    out += ["<tr>" + "".join(f"<td>{cell(c)}</td>" for c in cells(row)) + "</tr>" for row in body]
    out += ["</tbody>", "</table>", f"<p class=\"note\">{inline(machine)}</p>"]
    return "\n".join(out)


def cells(row: str) -> list[str]:
    return row.strip().strip("|").split("|")


def inline(text: str) -> str:
    """Markdown's inline code, escaped; nothing else in the table needs more."""
    return re.sub(r"`([^`]+)`", r"<code>\1</code>", html.escape(text, quote=False))


if __name__ == "__main__":
    main(Path(sys.argv[1]), Path(sys.argv[2]), sys.argv[3])
