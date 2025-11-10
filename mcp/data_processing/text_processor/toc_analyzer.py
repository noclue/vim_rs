#!/usr/bin/env python3
"""
TOC Level Detection using Arial Glyph Advance Widths
"""

import re
from collections import defaultdict

# Arial font glyph advance widths (from user)
GLYPH_WIDTHS = {
    " ": 4.4453125, "!": 4.4453125, '"': 5.6796875, "#": 8.8984375,
    "$": 8.8984375, "%": 14.2265625, "&": 10.671875, "'": 3.0546875,
    "(": 5.328125, ")": 5.328125, "*": 6.2265625, "+": 9.34375,
    ",": 4.4453125, "-": 5.328125, ".": 4.4453125, "/": 4.4453125,
    "0": 8.8984375, "1": 7.7228125, "2": 8.8984375, "3": 8.8984375,
    "4": 8.8984375, "5": 8.8984375, "6": 8.8984375, "7": 8.8984375,
    "8": 8.8984375, "9": 8.8984375, ":": 4.4453125, ";": 4.4453125,
    "<": 9.34375, "=": 9.34375, ">": 9.34375, "?": 8.8984375,
    "@": 16.2421875, "A": 10.671875, "B": 10.671875, "C": 11.5546875,
    "D": 11.5546875, "E": 10.671875, "F": 9.7734375, "G": 12.4453125,
    "H": 11.5546875, "I": 4.4453125, "J": 8, "K": 10.671875,
    "L": 8.8984375, "M": 13.328125, "N": 11.5546875, "O": 12.4453125,
    "P": 10.671875, "Q": 12.4453125, "R": 11.5546875, "S": 10.671875,
    "T": 9.7734375, "U": 11.5546875, "V": 10.671875, "W": 15.1015625,
    "X": 10.671875, "Y": 10.671875, "Z": 9.7734375, "[": 4.4453125,
    "\\": 4.4453125, "]": 4.4453125, "^": 7.5078125, "_": 8.8984375,
    "`": 5.328125, "a": 8.8984375, "b": 8.8984375, "c": 8,
    "d": 8.8984375, "e": 8.8984375, "f": 4.15921875, "g": 8.8984375,
    "h": 8.8984375, "i": 3.5546875, "j": 3.5546875, "k": 8,
    "l": 3.5546875, "m": 13.328125, "n": 8.8984375, "o": 8.8984375,
    "p": 8.8984375, "q": 8.8984375, "r": 5.328125, "s": 8,
    "t": 4.4453125, "u": 8.8984375, "v": 8, "w": 11.5546875,
    "x": 8, "y": 8, "z": 8, "{": 5.34375, "|": 4.15625,
    "}": 5.34375, "~": 9.34375, "®": 10.671875  # Added for VMware®
}

def calculate_width(text):
    """Calculate visual width of text using glyph advance widths"""
    total = 0.0
    for char in text:
        total += GLYPH_WIDTHS.get(char, 8.8984375)  # Default to average if missing
    return total

def parse_toc_line(line):
    """Parse TOC line and extract components"""
    # Pattern: "Title....Page" with variable dots and optional spaces
    match = re.match(r'^(.+?)(\.{3,})(\s*)(\d+)$', line)
    if match:
        title = match.group(1).rstrip()
        dots = match.group(2)
        spaces = match.group(3)
        page = int(match.group(4))

        # Calculate widths
        title_width = calculate_width(title)
        dots_width = calculate_width(dots)
        spaces_width = calculate_width(spaces)
        page_width = calculate_width(match.group(4))
        total_width = title_width + dots_width + spaces_width + page_width

        return {
            'line': line,
            'title': title,
            'dots': len(dots),
            'page': page,
            'title_width': title_width,
            'dots_width': dots_width,
            'spaces_width': spaces_width,
            'page_width': page_width,
            'total_width': total_width
        }
    return None

def cluster_widths(toc_entries, tolerance=10.0):
    """Cluster TOC entries by total width to determine hierarchy levels"""
    # Group by similar widths
    width_groups = defaultdict(list)

    for entry in toc_entries:
        width = entry['total_width']
        # Find existing cluster within tolerance
        found_cluster = None
        for cluster_width in width_groups.keys():
            if abs(width - cluster_width) <= tolerance:
                found_cluster = cluster_width
                break

        if found_cluster is not None:
            width_groups[found_cluster].append(entry)
        else:
            width_groups[width].append(entry)

    # Sort clusters by width (ascending - narrowest is Level 1)
    # Narrower = less indented = higher level in hierarchy
    sorted_clusters = sorted(width_groups.items(), key=lambda x: x[0], reverse=False)

    # Assign levels
    for level, (cluster_width, entries) in enumerate(sorted_clusters, start=1):
        for entry in entries:
            entry['level'] = level

    return sorted_clusters

def main():
    # Sample TOC lines from the user's data
    toc_lines = """Release Notes.....................................................................................................................................74
VMware vCenter Photon OS Security Patches...........................................................................................................74
ESX Installation and Setup............................................................................................................... 76
About ESX Installation and Requirements..................................................................................................................76
How to Install ESX....................................................................................................................................................77
Preparing for Installing ESX..................................................................................................................................... 77
Download the ESX Installer...............................................................................................................................78
Required Information for ESX Installation......................................................................................................... 78
Media Options for Booting the ESX Installer.................................................................................................... 78
ESX Requirements.................................................................................................................................................. 81
ESX System Storage Overview....................................................................................................................... 82
ESX Hardware Requirements.......................................................................................................................... 86
Using Remote Management Applications......................................................................................................... 88
What is VMware vSphere Distributed Services Engine®............................................................................................92
High Availability with VMware vSphere Distributed Services Engine.......................................................................94
Customizing Installations with vSphere ESX Image Builder.....................................................................................95
How the vSphere ESX Image Builder Works...........................................................................................................96
Image Profiles.................................................................................................................................................... 97
Working with Acceptance Levels.......................................................................................................................98
Structure of ImageProfile, SoftwarePackage, and ImageProfileDiff Objects.......................................................... 100
Configure vSphere ESX Image Builder.................................................................................................................. 104
ESX Image Profile Tasks.......................................................................................................................................107
Add a Software Depot..................................................................................................................................... 107
Clone an Image Profile....................................................................................................................................108
Compare Image Profiles.................................................................................................................................. 115
Installing ESX................................................................................................................................................................125
Installing ESX Interactively..................................................................................................................................... 125
Interactive ESX Installation..............................................................................................................................126
Installing ESX by Using a Script............................................................................................................................ 128
Scripted ESX Installation................................................................................................................................. 128
Network Booting the ESX Installer......................................................................................................................... 146
Overview of the Network Boot Installation Process........................................................................................ 146
Boot the ESX Installer by Using PXE and TFTP............................................................................................ 149
Installing ESX Using vSphere Auto Deploy.......................................................................................................... 158
Understanding vSphere Auto Deploy.............................................................................................................158
vSphere Auto Deploy Tasks by Using the vSphere Client..............................................................................177
Configure a Host Profile to Use Stateless Caching........................................................................................ 201
vSphere Auto Deploy Best Practices.............................................................................................................. 213
Troubleshooting vSphere Auto Deploy................................................................................................................... 227
vSphere Auto Deploy Rule Takes Long to Complete......................................................................................227
Setting Up ESX............................................................................................................................................................. 232
Initial ESX Configuration......................................................................................................................................... 232
ESX Autoconfiguration..................................................................................................................................... 232
Configuring Network Settings................................................................................................................................. 237
Network Access to Your ESX Host................................................................................................................. 238
Configure IP Settings from the Direct Console..............................................................................................239
Configuring System Logging...................................................................................................................................242
Configure Syslog on ESX Hosts..................................................................................................................... 243
VMware ESX Upgrade......................................................................................................................266
Overview of the ESX Host Upgrade Process............................................................................................................269
ESX Requirements................................................................................................................................................ 272""".strip().split('\n')

    # Parse all lines
    toc_entries = []
    for line in toc_lines:
        entry = parse_toc_line(line)
        if entry:
            toc_entries.append(entry)

    print("=" * 100)
    print("TOC WIDTH ANALYSIS")
    print("=" * 100)

    # Cluster by width
    clusters = cluster_widths(toc_entries, tolerance=15.0)

    print(f"\nFound {len(clusters)} distinct levels:\n")

    for level, (cluster_width, entries) in enumerate(clusters, start=1):
        print(f"LEVEL {level} - Width: {cluster_width:.2f} ({len(entries)} entries)")
        print("-" * 100)
        for entry in entries[:5]:  # Show first 5 from each level
            print(f"  [{entry['page']:3d}] {entry['title'][:60]}")
        if len(entries) > 5:
            print(f"  ... and {len(entries) - 5} more")
        print()

    # Show detailed breakdown for a few examples
    print("\n" + "=" * 100)
    print("DETAILED WIDTH BREAKDOWN (Sample)")
    print("=" * 100)

    samples = [toc_entries[0], toc_entries[1], toc_entries[6], toc_entries[17]]
    for entry in samples:
        print(f"\nLine: {entry['line']}")
        print(f"  Title: '{entry['title']}' (width: {entry['title_width']:.2f})")
        print(f"  Dots:  {entry['dots']} dots (width: {entry['dots_width']:.2f})")
        print(f"  Page:  {entry['page']} (width: {entry['page_width']:.2f})")
        print(f"  Total: {entry['total_width']:.2f}")
        print(f"  Level: {entry.get('level', '?')}")

    # Validate the hypothesis
    print("\n" + "=" * 100)
    print("VALIDATION: Do widths cluster cleanly?")
    print("=" * 100)

    level_widths = defaultdict(list)
    for entry in toc_entries:
        level_widths[entry['level']].append(entry['total_width'])

    for level in sorted(level_widths.keys()):
        widths = level_widths[level]
        avg = sum(widths) / len(widths)
        min_w = min(widths)
        max_w = max(widths)
        stddev = (sum((w - avg) ** 2 for w in widths) / len(widths)) ** 0.5
        print(f"Level {level}: avg={avg:.2f}, range=[{min_w:.2f}, {max_w:.2f}], stddev={stddev:.2f}")

if __name__ == "__main__":
    main()
