[← Aurion](../README.md) · **English** / [Русский](ru/DEVELOPMENT.md)

# Development

[User guide](USAGE.md) · **Development** · [Protocol](PROTOCOL.md)

## Environment and building

Aurion runs on **Windows 10/11**. Building from source requires:

- Node.js 24 and npm 11;
- Rust stable with rustfmt and Clippy, MSVC Build Tools, and the Windows SDK;
- WebView2;
- [Npcap](https://npcap.com/) to capture data from the running game.

The `cargo --version` command must work in your terminal.

From the project directory, run:

```sh
npm ci
npm run build:release
```

Built application: `src-tauri/target/release/aurion.exe`.

### Release archive

After a successful release build, package the executable with the project license and the
generated `licenses/` directory, which includes the font licenses. In PowerShell:

```powershell
Compress-Archive -LiteralPath @(
  'src-tauri/target/release/aurion.exe',
  'LICENSE',
  'src-tauri/target/release/licenses'
) -DestinationPath 'src-tauri/target/release/Aurion-0.1.1-windows-x64.zip'
```

Publish the archive alongside the source code for the same release tag.

## Running and checking

Rust and Tauri 2 handle capture, protocol parsing, game state, HUD evaluation,
and storage. Vue 3 and TypeScript provide the editor, HUD scene, and combat journal.

<table width="100%">
  <thead>
    <tr>
      <th align="left" width="30%">Command</th>
      <th align="left" width="70%">Purpose</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong><code>npm run dev</code></strong></td>
      <td>Tauri and Vite with reloading</td>
    </tr>
    <tr>
      <td><strong><code>npm run start</code></strong></td>
      <td>Build the frontend and launch the app</td>
    </tr>
    <tr>
      <td><strong><code>npm run check</code></strong></td>
      <td>Frontend, Rust build, rustfmt, Clippy, and JSON Schema</td>
    </tr>
    <tr>
      <td><strong><code>npm run graph</code></strong></td>
      <td>Module boundaries and import cycles</td>
    </tr>
  </tbody>
</table>

Automated tests are currently removed. Decide what to cover after the code work is
finished. Build and static checks do not establish correct behavior in the game.

Run `make check` and `make graph` after changes, or `npm run check` and
`npm run graph` without GNU Make. The checks include the Rust build.

## Replay

Set these variables to replay a PCAP instead of capturing live traffic:

<table width="100%">
  <thead>
    <tr>
      <th align="left" width="42%">Variable</th>
      <th align="left" width="58%">Value</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_FILE</code></strong></td>
      <td>Path to the PCAP</td>
    </tr>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_SERVER_PORTS</code></strong></td>
      <td>Comma-separated server ports; default <code>1111</code></td>
    </tr>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_SPEED</code></strong></td>
      <td>Speed, <code>0 < x ≤ 100</code>; default <code>1</code></td>
    </tr>
    <tr>
      <td><strong><code>AURION_DATA_DIRECTORY</code></strong></td>
      <td>Separate absolute data directory to protect the working profile</td>
    </tr>
  </tbody>
</table>
