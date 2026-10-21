# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-10-21

### Changed

- Renamed the GitHub repository from Auiron to Aurion and updated download links.

### Fixed

- Release checks now use the permanent GitHub repository ID, so repository renames
  do not break update notifications.

## [0.1.1] - 2026-10-21

### Added

- Release checks at startup and every four hours, with update notices in the editor,
  a toolbar indicator, and installed version and manual checks in settings.
- A release-page button for downloading updates manually.

### Fixed

- Class-restricted HUDs remaining hidden after starting Aurion inside an existing game
  session: observed own class-specific skills can now restore a missing character class.
- The editor now explains when the HUD is waiting for character data after Aurion
  starts inside an existing game session. The first-launch instructions cover this case.

## [0.1.0] - 2026-10-21

Initial release of Aurion for AION 2.

Requires Windows 10/11, Microsoft Edge WebView2, and Npcap for live capture.

### Added

- Transparent game overlay with a visual HUD editor for icons, bars, text, layers,
  and nested groups. Supports positioning, resizing, alignment, and screen scaling.
- Configurable triggers, conditions, formulas, animations, sounds, and spoken alerts.
- Player, target, and party frames with health, mana, buffs, debuffs, stacks,
  and effect timers where available from observed game data.
- Skill cooldowns, chain steps, cast bars, charge preparation, and recent skill history.
- Periodic healing forecasts with separate displays for useful healing and overhealing.
- World boss timers and configurable alerts before a scheduled spawn.
- Animated combat simulation in the editor with adjustable state and party size,
  kept separate from recorded combat history.
- Configurable DPS/HPS windows and a combat journal with fight and run history,
  charts, skill breakdowns, rotation timelines, effect uptime, deaths, comparisons,
  and CSV export.
- Profile import, export, updates, and author-provided translations. Imported author
  and version fields are read-only in the editor.
- English and Russian localization, a compact toolbar, system tray controls,
  and saved panel positions and settings.
- Passive incoming traffic capture and PCAP replay for offline analysis.
- A ready-to-import Cleric HUD with an accompanying in-game interface preset,
  gameplay GIFs, and editor screenshots.
- Installation, usage, development, and protocol documentation, with issue reporting
  guidelines and bug report and feature request forms in English and Russian.
- GNU GPL-3.0-or-later license.

### Known issues

- A boss reset without a death event, or a wipe with unidentified party members,
  can leave the fight open and merge the next pull into the same record.
- Party support is experimental; members or their statistics may be missing after an instance change.
- Cancelled charges can keep their bar visible until the catalog duration expires.
- A game panel opened while holding Alt may not hide the HUD.

See the [user guide](docs/USAGE.md#known-limitations) for details.
