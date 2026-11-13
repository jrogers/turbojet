"""Makes a kernel source tree's vfio-pci map prefetchable BARs write-combined, for ENA's
low-latency queue under DPDK: the change AWS's amzn-drivers enav2-vfio-patch makes, applied by
pattern rather than by diff so that it holds on kernels newer than its patches. Run in
drivers/vfio; scripts/aws-bench.sh's dpdk run does.

Both mappings change. The user's (mmap's page protection) is what DPDK writes through; the
kernel's own (pci_iomap of the BAR) matters on x86, where PAT gives a later mapping of the same
memory the type of the first, so an uncached kernel mapping would keep DPDK's uncached too.
"""

import re
import sys
from pathlib import Path

PREFETCH = "(pci_resource_flags(pdev, {bar}) & IORESOURCE_PREFETCH)"

user = 0
core = Path("pci/vfio_pci_core.c")
text = core.read_text()
old = "\tvma->vm_page_prot = pgprot_noncached(vma->vm_page_prot);\n"
if text.count(old) != 1:
    sys.exit("vfio_pci_core.c: not one pgprot_noncached mmap line")
text = text.replace(
    old,
    f"\tif {PREFETCH.format(bar='index')}\n"
    "\t\tvma->vm_page_prot = pgprot_writecombine(vma->vm_page_prot);\n"
    "\telse\n"
    "\t\tvma->vm_page_prot = pgprot_noncached(vma->vm_page_prot);\n",
)
core.write_text(text)
user += 1

kernel = 0
iomap = re.compile(r"= pci_iomap\(pdev, (\w+), 0\);")
for path in Path("pci").glob("vfio_pci_*.c"):
    text = path.read_text()
    text, n = iomap.subn(
        lambda m: f"= {PREFETCH.format(bar=m[1])} ?\n\t\tioremap_wc(pci_resource_start(pdev, {m[1]}), "
        f"pci_resource_len(pdev, {m[1]})) :\n\t\tpci_iomap(pdev, {m[1]}, 0);",
        text,
    )
    if n:
        path.write_text(text)
        kernel += n
if kernel == 0:
    sys.exit("no pci_iomap(pdev, BAR, 0) of a BAR in pci/vfio_pci_*.c")
print(f"vfio-pci write-combined: {user} user mapping, {kernel} kernel mappings", file=sys.stderr)
