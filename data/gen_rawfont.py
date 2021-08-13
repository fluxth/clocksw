#!/usr/bin/env python3
#
# BDF to rawfont packer for clocksw
# aka. the embedding pixel font data to elf binary thingy
#
# (c) fluxth, 2020
#

import sys
import os.path
import argparse
import struct
from bdflib import reader
from bdflib.model import Font, Glyph

from typing import Optional, List, Tuple


def char(str):
    pass


class GlyphRange:
    start_codepoint: int
    glyph_range: List[Glyph] = []

    def __init__(self, start_codepoint: int):
        self.start_codepoint = start_codepoint
        self.glyph_range = list()

    def __repr__(self):
        return str(self)

    def __str__(self):
        items = len(self.glyph_range)
        return f"<GlyphRange Items={items} ({chr(self.start_codepoint)} -> {chr(self.start_codepoint + items)})>"

    def add(self, glyph):
        return self.glyph_range.append(glyph)


class RawFont:
    font: Font
    ranges: List[GlyphRange] = []
    char_count: int = 0

    size: int = 0
    baseline: Tuple[int, int] = (0, 0)  # (upper, lower)

    max_width: int = 0
    max_height: int = 0

    def __init__(self, font: Font):
        self.font = font
        self.size = int(font[b"POINT_SIZE"])
        self.baseline = (font[b"FONT_ASCENT"], font[b"FONT_DESCENT"])

    def add_range(self, start_chr: char, end_char: char) -> bool:
        this_range = GlyphRange(ord(start_chr))
        for codepoint in range(ord(start_chr), ord(end_char) + 1):
            try:
                glyph = self.font[codepoint]
            except KeyError:
                return False

            this_range.add(glyph)
            self.char_count += 1

        self.ranges.append(this_range)
        return True

    def calculate_max_bounds(self) -> bool:
        for r in self.ranges:
            for glyph in r.glyph_range:
                width = glyph.bbW
                if width > self.max_width:
                    self.max_width = width + glyph.bbX

                height = glyph.bbH
                if height > self.max_height:
                    self.max_height = height - glyph.bbY

        return True

    def get_normalized_glyph(self, glyph: Glyph) -> Optional[List[int]]:
        if self.max_width == 0 or self.max_height == 0:
            if not self.calculate_max_bounds():
                return None

            return self.get_normalized_glyph(self)

        data = []

        # check width
        gw = len(glyph.data)
        places = glyph.bbW + 4 - (glyph.bbW % 4)

        for y in range(self.size):
            if y < gw:
                bin_str = bin(glyph.data[gw - y - 1])[2:]

                bin_list = [c for c in bin_str.rjust(places, "0")]
                bin_list.reverse()

                payload = int("".join(bin_list), base=2)
                data.append(payload)

        return data, places


def main() -> int:
    parser = argparse.ArgumentParser(description="BDF to rawfont converter.")
    parser.add_argument("input", type=str, help="input filename (.bdf)")
    parser.add_argument("-o", "--output", type=str, help="output filename")
    parser.add_argument(
        "-t", "--table", type=str, help="output table filename (.rftable)"
    )
    parser.add_argument(
        "-y", action="store_true", help="overwrite existing output file"
    )
    parser.add_argument(
        "--preview", action="store_true", help="print output to stdout then exit"
    )

    g = parser.add_mutually_exclusive_group()
    g.add_argument(
        "--complex", action="store_true", help="output complex (.rawfontc) format"
    )
    g.add_argument(
        "--reduced", action="store_true", help="output reduced (.rawfont) format"
    )

    args = parser.parse_args()

    in_file = os.path.abspath(args.input)
    out_file = args.output

    uses_table = not args.complex
    table_file = args.table

    filename = in_file
    if args.output is None:
        out_split = in_file.split(".")
        suffix = "-Reduced" if args.reduced else ""
        if not len(out_split) == 1:
            filename = ".".join(out_split[:-1])

        filename += suffix
        out_file = filename + ".rawfont"

        if args.complex:
            out_file += "c"

    if uses_table and table_file is None:
        table_file = filename + ".rftable"

    font = read_bdf_file(in_file)
    if font is None:
        print(f"Error: Unable to read input file '{in_file}' as BDF font")
        return 1

    if os.path.isfile(out_file) and not args.y and not args.preview:
        print(f"Warning: Output file '{out_file}' already exists!")
        choice = input("Overwrite file? [y/N] ")
        if not choice.lower() == "y":
            return 1

    if uses_table:
        if os.path.isfile(table_file) and not args.y and not args.preview:
            print(f"Warning: Table output file '{table_file}' already exists!")
            choice = input("Overwrite file? [y/N] ")
            if not choice.lower() == "y":
                return 1

    rf_text_type = "complex" if args.complex else ("reduced" if args.reduced else "")
    print(
        "Generating {}rawfont for '{}' @ {} pt...".format(
            f"{rf_text_type} " if rf_text_type != "" else "",
            font[b"FACE_NAME"].decode("utf-8"),
            font[b"POINT_SIZE"],
        )
    )

    gen_rawfont = generate_regular_rawfont
    write_rawfont = write_regular_rawfont

    if args.reduced:
        gen_rawfont = generate_reduced_rawfont
        write_rawfont = write_reduced_rawfont

    elif args.complex:
        gen_rawfont = generate_complex_rawfont
        write_rawfont = write_complex_rawfont
        if args.table:
            print(
                "Warning: unused option '--table' present when generating complex rawfont"
            )

    raw_font = RawFont(font)
    if not gen_rawfont(raw_font):
        print("Error: Font generation error")
        return 1

    if args.preview:
        print_preview(raw_font)
        print("Preview done, exiting.")
        return 0

    print("Max dimensions:", (raw_font.max_width, raw_font.max_height))

    if write_rawfont(out_file, raw_font, table_filename=table_file):
        print(
            "Complete: {}rawfont written to output file '{}'.".format(
                f"{rf_text_type.title()} " if rf_text_type != "" else "", out_file
            )
        )
        if uses_table:
            print(
                "Complete: {}rftable written to output file '{}'.".format(
                    f"{rf_text_type.title()} " if rf_text_type != "" else "", table_file
                )
            )
        return 0

    print(f"Error: Unable to write output file '{out_file}'")
    return 1


def read_bdf_file(filename: str) -> Optional[Font]:
    font = None
    try:
        with open(filename, "rb") as f:
            font = reader.read_bdf(f)
    except (FileNotFoundError, StopIteration):
        pass

    return font


class Writer:
    file_handle = None
    endian_marker = ""

    def __init__(self, file_handle, endian="little"):
        self.file_handle = file_handle

        if endian == "little":
            self.endian_marker = "<"
        elif endian == "big":
            self.endian_marker = ">"

    def write_char(self, v):
        return self.file_handle.write(struct.pack("c" * len(v), *[c for c in v]))

    def write_i8(self, v):
        return self.file_handle.write(struct.pack("b", v))

    def write_u8(self, v):
        return self.file_handle.write(struct.pack("B", v))

    def write_u16(self, v):
        return self.file_handle.write(struct.pack(self.endian_marker + "H", v))

    def write_u32(self, v):
        return self.file_handle.write(struct.pack(self.endian_marker + "I", v))

    def write_u64(self, v):
        return self.file_handle.write(struct.pack(self.endian_marker + "Q", v))


def print_preview(rf: RawFont):
    print("----- PREVIEW -----")
    print(f"Total characters: {rf.char_count}")
    print(f"Ranges ({len(rf.ranges)}): {rf.ranges}")
    print()

    for rk, r in enumerate(rf.ranges):
        for ck, glyph in enumerate(r.glyph_range):
            print(f"Range {rk}, Char {ck}:", glyph.name.decode("ascii"))
            print("Size:", (glyph.bbW, glyph.bbH), "Pos:", (glyph.bbX, glyph.bbY))
            # (glyph.bbW + glyph.bbX, glyph.bbH - glyph.bbY)

            data, places = rf.get_normalized_glyph(glyph)
            print("Payload: ", data)
            for row in range(len(data)):
                for i in range(places):
                    if i >= 0:
                        print("@" if data[row] & (1 << i) else ".", end="")
                print()
            print()


def generate_regular_rawfont(rf: RawFont) -> bool:
    failed = False

    # only ascii standard range
    failed = failed or not rf.add_range(" ", "~")

    failed = failed or not rf.calculate_max_bounds()
    return not failed


def write_regular_rawfont(
    filename: str, rf: RawFont, table_filename: Optional[str] = None
) -> bool:
    rftable = []
    with open(filename, "wb") as f:
        w = Writer(f)

        byte_index = 0
        for r in rf.ranges:
            for glyph in r.glyph_range:
                data, places = rf.get_normalized_glyph(glyph)
                rftable.append((byte_index, places, glyph))

                if places > 64:
                    print("Error: Glyph too large, Max width: 64.")
                    return False

                write_function = w.write_u64
                byte_size = 8

                if places <= 8:
                    write_function = w.write_u8
                    byte_size = 1

                elif places <= 16:
                    write_function = w.write_u16
                    byte_size = 2

                elif places <= 32:
                    write_function = w.write_u32
                    byte_size = 4

                for row in data:
                    write_function(row)
                    byte_index += byte_size

            if byte_index > 65535:
                print("Error: Font too large, Max size: 66K.")
                return False

    if table_filename is None:
        print("Error: This rawfont type requires rftable output filename to be present")
        return False

    with open(table_filename, "wb") as f:
        w = Writer(f)

        w.write_u16(len(rftable))  # length

        for index, places, glyph in rftable:
            w.write_u16(index)  # byte index
            w.write_u8(places)  # bit size
            w.write_i8(glyph.bbX)  # x
            w.write_i8(glyph.bbY)  # y
            w.write_u8(glyph.bbW)  # w
            w.write_u8(glyph.bbH)  # h

        return True

    return False


def generate_reduced_rawfont(rf: RawFont) -> bool:
    failed = False

    # ascii symbols + numbers [43:58]
    failed = failed or not rf.add_range("+", ":")

    # ascii only `?` [63]
    failed = failed or not rf.add_range("?", "?")

    # ascii uppercase A to F [65:70]
    failed = failed or not rf.add_range("A", "F")

    failed = failed or not rf.calculate_max_bounds()
    return not failed


def write_reduced_rawfont(
    filename: str, rf: RawFont, table_filename: Optional[str] = None
) -> bool:
    return write_regular_rawfont(filename, rf, table_filename)


def generate_complex_rawfont(rf: RawFont) -> bool:
    failed = False

    # ascii standard (' ' -> '~') [32:126]
    failed = failed or not rf.add_range(" ", "~")

    failed = failed or not rf.calculate_max_bounds()
    return not failed


def write_complex_rawfont(
    filename: str, rf: RawFont, table_filename: Optional[str] = None
):
    def end_section():
        return struct.pack("B", 29)

    def start_enumeration():
        return struct.pack("B", 30)

    def end_item():
        return struct.pack("B", 31)

    with open(filename, "wb") as f:
        w = Writer(f)

        """--- START RAWFONT FILE"""
        """----- START RAWFONT HEADER"""
        f.write(struct.pack("cc", b"R", b"F"))  # RawFont magic
        w.write_u8(1)  # VERSION 1
        w.write_u16(rf.char_count)  # char count
        w.write_u8(rf.max_width)  # max font width
        w.write_u8(rf.max_height)  # max font height
        f.write(end_section())  # GS; end rf header
        """----- END RAWFONT HEADER"""

        """----- START RANGES HEADER"""
        w.write_u8(len(rf.ranges))  # range count
        f.write(start_enumeration())  # RS; start range list

        for i, r in enumerate(rf.ranges):
            w.write_u8(i)  # range id: 0
            w.write_u16(r.start_codepoint)  # start char_id
            w.write_u16(len(r.glyph_range))  # range len
            f.write(end_item())  # US; end range item

        f.write(end_section())  # GS; end range header
        """----- END RANGES HEADER"""

        """----- START PAYLOAD"""
        for r in rf.ranges:
            f.write(start_enumeration())  # RS; start range

            for glyph in r.glyph_range:
                w.write_i8(glyph.bbX)  # x
                w.write_i8(glyph.bbY)  # y
                w.write_u8(glyph.bbW)  # w
                w.write_u8(glyph.bbH)  # h
                w.write_u8(glyph.advance)  # adv

                data_type = "<Q"  # 64bit
                if glyph.bbW < 8:
                    data_type = "B"  # 8bit

                elif glyph.bbW < 16:
                    data_type = "<H"  # 16bit

                elif glyph.bbW < 32:
                    data_type = "<I"  # 32bit

                for d in glyph.data:
                    struct.pack(data_type, d)

                f.write(end_item())  # US; end char item

        f.write(end_section())  # GS; end payload
        """--- END RAWFONT FILE"""

        return True


if __name__ == "__main__":
    sys.exit(main())
