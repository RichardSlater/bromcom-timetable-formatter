#!/usr/bin/env python3
"""Generate a deterministic synthetic Bromcom-style timetable PDF for tests."""
from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import List
import io

OUTPUT_PATH = Path("test/fixtures/synthetic_timetable.pdf")


@dataclass
class TextItem:
    text: str
    x: float
    y: float


TEXT_ITEMS: List[TextItem] = [
    TextItem("Week 1", 75, 780),
    TextItem("Alex Testington (11XX)", 75, 760),
    # Day headers
    TextItem("Monday", 140, 720),
    TextItem("Tuesday", 220, 720),
    TextItem("Wednesday", 300, 720),
    TextItem("Thursday", 400, 720),
    TextItem("Friday", 500, 720),
    # Period labels
    TextItem("PD", 60, 690),
    TextItem("L1", 60, 660),
    TextItem("L2", 60, 630),
    TextItem("L3", 60, 600),
    TextItem("L4", 60, 570),
    TextItem("L5", 60, 540),
    # Monday lessons
    TextItem("Personal Dev.", 140, 690),
    TextItem("HU1", 140, 675),
    TextItem("Ms Test A", 140, 660),
    TextItem("Geography", 140, 630),
    TextItem("HU2", 140, 615),
    TextItem("Mr Test B", 140, 600),
    TextItem("Mathematics", 140, 570),
    TextItem("MA3", 140, 555),
    TextItem("Ms Test C", 140, 540),
    # Tuesday sample
    TextItem("Science", 220, 690),
    TextItem("SC4", 220, 675),
    TextItem("Mr Proton", 220, 660),
    # Wednesday sample
    TextItem("French", 300, 630),
    TextItem("LA2", 300, 615),
    TextItem("Ms Azure", 300, 600),
]


def encode_bromcom_text(source: str) -> bytes:
    """Apply the inverse of decode_bromcom_text (subtract 29)."""
    encoded = bytearray()
    for ch in source:
        encoded.append((ord(ch) - 29) % 256)
    return bytes(encoded)


def build_content_stream() -> bytes:
    lines = ["BT", "/F1 12 Tf"]
    for item in TEXT_ITEMS:
        encoded = encode_bromcom_text(item.text)
        hex_text = encoded.hex()
        lines.append(f"1 0 0 1 {item.x:.2f} {item.y:.2f} Tm <{hex_text}> Tj")
    lines.append("ET")
    lines.append("")
    content = "\n".join(lines).encode("ascii")
    return content


def write_pdf(path: Path) -> None:
    content = build_content_stream()
    content_length = len(content)

    buf = io.BytesIO()

    def write(raw: str) -> None:
        buf.write(raw.encode("ascii"))

    offsets = [0]  # index 0 reserved

    def write_obj(obj_number: int, body: str) -> None:
        offsets.append(buf.tell())
        write(f"{obj_number} 0 obj\n{body}\nendobj\n")

    write("%PDF-1.4\n")

    write_obj(1, "<< /Type /Catalog /Pages 2 0 R >>")
    write_obj(2, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>")
    page_dict = (
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] "
        "/Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>"
    )
    write_obj(3, page_dict)
    write_obj(4, f"<< /Length {content_length} >>\nstream\n" + content.decode("ascii") + "endstream")
    write_obj(5, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>")

    xref_start = buf.tell()
    write("xref\n")
    size = len(offsets)
    write(f"0 {size}\n")
    write("0000000000 65535 f \n")
    for offset in offsets[1:]:
        write(f"{offset:010d} 00000 n \n")
    write(
        "trailer\n"
        f"<< /Size {size} /Root 1 0 R >>\n"
        "startxref\n"
        f"{xref_start}\n"
        "%%EOF\n"
    )

    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(buf.getvalue())


if __name__ == "__main__":
    write_pdf(OUTPUT_PATH)
    print(f"Wrote synthetic PDF to {OUTPUT_PATH}")
