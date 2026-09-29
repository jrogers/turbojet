#!/usr/bin/env python3
"""Repairs an Orchestra file converted from the FIX Unified Repository (2010 Edition).

FIX 4.3's repository defines a repeating group inside a component inline: its NumInGroup field,
followed by its members indented one level (Instrument's NoSecurityAltID, for example). The
FIX Trading Community's unified2orchestra.xslt expects each group to have its own component ID,
so for these it writes a groupRef to the enclosing component's own ID and drops the members.

This script defines each such group from the repository's MsgContents.xml, named and numbered
after the group with the same NumInGroup field in a later Orchestra file (FIX 4.4's), and points
the groupRef at it. It also drops the conversion time the stylesheet records (dc:date), so the
output depends only on its inputs.

Usage: fix-inline-groups.py <converted.xml> <MsgContents.xml> <names-from.xml> <output.xml>
"""
import sys
import xml.etree.ElementTree as ET

FIXR = "http://fixprotocol.io/2020/orchestra/repository"
N = "{" + FIXR + "}"
for prefix, uri in [("fixr", FIXR), ("dc", "http://purl.org/dc/elements/1.1/"),
                    ("functx", "http://www.functx.com"), ("xs", "http://www.w3.org/2001/XMLSchema")]:
    ET.register_namespace(prefix, uri)


def main(converted, contents, names_from, output):
    tree = ET.parse(converted)
    root = tree.getroot()
    metadata = root.find(N + "metadata")
    for date in metadata.findall("{http://purl.org/dc/elements/1.1/}date"):
        metadata.remove(date)
    groups = root.find(N + "groups")
    later = {}
    for group in ET.parse(names_from).getroot().find(N + "groups"):
        later[group.find(N + "numInGroup").get("id")] = (group.get("id"), group.get("name"))

    rows = {}
    for row in ET.parse(contents).getroot():
        rows.setdefault(row.findtext("ComponentID"), []).append(row)

    defined = {g.get("id") for g in groups}
    for component in root.find(N + "components"):
        cid = component.get("id")
        broken = [r for r in component if r.tag == N + "groupRef" and r.get("id") == cid]
        if not broken:
            continue
        inline = inline_groups(rows.get(cid, []))
        if len(inline) != len(broken):
            sys.exit(f"component {component.get('name')}: {len(broken)} groupRefs, {len(inline)} inline groups")
        for ref, (count, members) in zip(broken, inline):
            gid, name = later.get(count, (None, None))
            if gid is None:
                sys.exit(f"no later group counted by tag {count} to name the one in {component.get('name')}")
            ref.set("id", gid)
            if gid in defined:
                continue
            group = ET.SubElement(groups, N + "group", {"id": gid, "name": name, "added": "FIX.4.3"})
            ET.SubElement(group, N + "numInGroup", {"id": count})
            for tag, required in members:
                attrs = {"id": tag, "added": "FIX.4.3"}
                if required:
                    attrs["presence"] = "required"
                ET.SubElement(group, N + "fieldRef", attrs)
            defined.add(gid)
            print(f"{component.get('name')}: defined {name} ({gid}), counted by {count}")
    tree.write(output, encoding="UTF-8", xml_declaration=True)


def inline_groups(rows):
    """(count tag, [(member tag, required)]) for each group indented under a component's field."""
    rows = sorted(rows, key=lambda r: float(r.findtext("Position")))
    found = []
    for i, row in enumerate(rows):
        indent = row.findtext("Indent")
        members = []
        for member in rows[i + 1:]:
            if member.findtext("Indent") != str(int(indent) + 1):
                break
            members.append((member.findtext("TagText"), member.findtext("Reqd") == "1"))
        if members:
            found.append((row.findtext("TagText"), members))
    return found


if __name__ == "__main__":
    if len(sys.argv) != 5:
        sys.exit(__doc__)
    main(*sys.argv[1:])
