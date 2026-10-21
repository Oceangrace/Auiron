[← Aurion](../../README.ru.md) · [English](../DEVELOPMENT.md) / **Русский**

# Разработка

[Руководство](USAGE.md) · **Разработка** · [Протокол](PROTOCOL.md)

## Окружение и сборка

Aurion работает на **Windows 10/11**. Для сборки из исходников нужны:

- Node.js 24 и npm 11;
- Rust stable с rustfmt и Clippy, MSVC Build Tools и Windows SDK;
- WebView2;
- [Npcap](https://npcap.com/) для получения данных из запущенной игры.

Команда `cargo --version` должна работать в терминале.

В папке проекта выполните:

```sh
npm ci
npm run build:release
```

Готовое приложение: `src-tauri/target/release/aurion.exe`.

### Архив релиза

После успешной release-сборки упакуйте приложение вместе с лицензией проекта и созданной
папкой `licenses/`, которая содержит лицензии шрифтов. В PowerShell:

```powershell
Compress-Archive -LiteralPath @(
  'src-tauri/target/release/aurion.exe',
  'LICENSE',
  'src-tauri/target/release/licenses'
) -DestinationPath 'src-tauri/target/release/Aurion-0.1.0-windows-x64.zip'
```

Публикуйте архив вместе с исходниками соответствующего тега релиза.

## Запуск и проверки

Rust и Tauri 2 отвечают за захват, протокол, игровое состояние, вычисление HUD
и хранение данных. Vue 3 и TypeScript — за редактор, сцену HUD и боевой журнал.

<table width="100%">
  <thead>
    <tr>
      <th align="left" width="30%">Команда</th>
      <th align="left" width="70%">Назначение</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong><code>npm run dev</code></strong></td>
      <td>Запуск с перезагрузкой Tauri и Vite</td>
    </tr>
    <tr>
      <td><strong><code>npm run start</code></strong></td>
      <td>Сборка фронтенда и запуск приложения</td>
    </tr>
    <tr>
      <td><strong><code>npm run check</code></strong></td>
      <td>Фронтенд, Rust-сборка, rustfmt, Clippy и JSON Schema</td>
    </tr>
    <tr>
      <td><strong><code>npm run graph</code></strong></td>
      <td>Границы модулей и циклы импортов</td>
    </tr>
  </tbody>
</table>

Автоматические тесты пока удалены. Необходимое покрытие определим после завершения
работы над кодом. Сборка и статические проверки не подтверждают корректность в игре.

После изменений обязательны `make check` и `make graph`; без GNU Make —
`npm run check` и `npm run graph`. Проверка включает Rust-сборку.

## Воспроизведение записи

Приложение воспроизводит PCAP вместо живого захвата при заданных переменных:

<table width="100%">
  <thead>
    <tr>
      <th align="left" width="42%">Переменная</th>
      <th align="left" width="58%">Значение</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_FILE</code></strong></td>
      <td>путь к PCAP</td>
    </tr>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_SERVER_PORTS</code></strong></td>
      <td>порты сервера через запятую, по умолчанию <code>1111</code></td>
    </tr>
    <tr>
      <td><strong><code>AURION_AION2_REPLAY_SPEED</code></strong></td>
      <td>скорость, <code>0 < x ≤ 100</code>, по умолчанию <code>1</code></td>
    </tr>
    <tr>
      <td><strong><code>AURION_DATA_DIRECTORY</code></strong></td>
      <td>отдельная абсолютная папка данных, чтобы не трогать рабочий профиль</td>
    </tr>
  </tbody>
</table>
