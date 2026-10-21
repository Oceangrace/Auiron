<p align="center">
  <img src="docs/assets/banner.svg" width="640" alt="Aurion — AION 2" />
</p>

<p align="center">
  <a href="#installation"><img src="docs/assets/badges/version.svg" alt="Version 0.1.1" /></a>
  <a href="#installation"><img src="docs/assets/badges/windows.svg" alt="Windows 10 / 11" /></a>
  <a href="docs/DEVELOPMENT.md"><img src="docs/assets/badges/rust.svg" alt="Rust edition 2024" /></a>
  <a href="docs/DEVELOPMENT.md"><img src="docs/assets/badges/tauri.svg" alt="Tauri 2" /></a><br />
  <a href="#installation"><img src="docs/assets/badges/installation.svg" alt="Installation" /></a>
  <a href="docs/USAGE.md"><img src="docs/assets/badges/docs.svg" alt="User guide" /></a>
  <a href="docs/DEVELOPMENT.md"><img src="docs/assets/badges/development.svg" alt="Development" /></a>
</p>

<p align="center">
  <strong>English</strong> / <a href="README.ru.md">Русский</a>
</p>

<img src="docs/assets/aurion-inline.svg" width="54" height="16" alt="Aurion" /> is a standalone Windows application that displays a transparent overlay window over AION 2.
Build your own interface with icons, bars, and text; control what appears through triggers and conditions;
track cooldowns, buffs, and debuffs; and analyze damage, healing, and combat events.

Aurion reads incoming game traffic without reading game memory or modifying game files.

## Features

<table width="100%">
  <tbody>
    <tr>
      <td width="30%"><strong>HUD editor</strong></td>
      <td width="70%">Icons, bars, text, layers, and groups.</td>
    </tr>
    <tr>
      <td width="30%"><strong>Skills & effects</strong></td>
      <td width="70%">Buffs, debuffs, stacks, cooldowns, chains, and casts.</td>
    </tr>
    <tr>
      <td width="30%"><strong>Triggers</strong></td>
      <td width="70%">Choose what to track and when each element appears.</td>
    </tr>
    <tr>
      <td width="30%"><strong>Combat journal</strong></td>
      <td width="70%">DPS/HPS, history, charts, fight comparisons, and CSV export.</td>
    </tr>
    <tr>
      <td width="30%"><strong>Profiles</strong></td>
      <td width="70%">Import, export, translations, and screen scaling.</td>
    </tr>
  </tbody>
</table>

Party support is experimental. See the [known limitations](docs/USAGE.md#known-limitations).

[Screenshots: editor, combat simulation, and journal analysis →](docs/screenshots.md)

## Installation

<p><strong>Requires:</strong> Windows 10/11 · <a href="https://npcap.com/#download">Npcap</a> · Microsoft Edge WebView2</p>

[Download Aurion for Windows →](https://github.com/Oceangrace/Auiron/releases/latest)

1. Install Npcap.
2. Extract the Aurion archive and launch **aurion.exe**.
3. Launch AION 2 and enter the game world. Data appears automatically.
4. Open the editor from the toolbar or system tray. Create or import a profile and save with <kbd>Ctrl</kbd> + <kbd>S</kbd>.

If Aurion starts while your character is already in the world, use a class-specific skill
to let it identify your class and load matching HUDs. If character data is still incomplete,
keep Aurion running and re-enter your character once to receive the full initial snapshot.

[Npcap settings, permissions, and controls →](docs/USAGE.md)

## Example profile

[![Cleric HUD in combat](docs/assets/hud/combat.gif)](docs/example.md)

**Cleric · Mirrored HUD** — player and target health, mana, skills, effects, party frames,
and recent skill history. Built with the same editor available to every user.

[Get the profile: Aurion import, in-game interface preset, and previews →](docs/example.md)

## Found a problem or have an idea?

If something is missing, looks wrong, or stops working, please let us know through
the repository's **Issues** tab. Reports about the example Cleric HUD are welcome too:
they help improve both the profile and Aurion.

Choose a bug report or feature request in English or Russian. Include your Aurion version,
what happened, and how to repeat it; a screenshot or short video is especially helpful for HUD issues.

[How to report a problem or suggest a feature →](CONTRIBUTING.md)

## Documentation

<table width="100%">
  <thead>
    <tr>
      <th align="left" width="30%">Guide</th>
      <th align="left" width="70%">Contents</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href="docs/USAGE.md">User guide</a></td>
      <td>Installation, settings, and keyboard shortcuts</td>
    </tr>
    <tr>
      <td><a href="docs/DEVELOPMENT.md">Development</a></td>
      <td>Environment, building, checks, and replay</td>
    </tr>
    <tr>
      <td><a href="docs/PROTOCOL.md">Protocol</a></td>
      <td>Available game data and parser limitations</td>
    </tr>
  </tbody>
</table>

## License

[![License: GPL-3.0](docs/assets/badges/license.svg)](LICENSE)

Aurion is free software under the [GNU General Public License v3.0](LICENSE) or any later
version. Bundled fonts keep their own licenses: SIL Open Font License 1.1, and Ubuntu Font
Licence 1.0 for Ubuntu Sans. The license texts are next to each font in `src/renderer/fonts/`.

Aurion is an independent fan project, not affiliated with or endorsed by NCSOFT.
AION 2, its names and game assets belong to their owners. Third-party tools may conflict
with the game's terms of service; use Aurion at your own risk.
