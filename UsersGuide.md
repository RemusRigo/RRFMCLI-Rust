# RRFMCLI: User's Guide



## Downloads & Runtimes

Download the latest release from the [Releases page](https://github.com/RemusRigo/RRFMCLI-Rust/releases).

|File|Required Runtime|
|-|-|
|`RRFMCLI.exe`|none (native Windows console app)|



## Install

Copy `RRFMCLI.exe` to a folder, for example `D:\My Apps\RRFMCLI\`.
To run it from any folder, add that folder to your `PATH`.

To build it yourself: install [Rust](https://www.rust-lang.org/tools/install), then run `Build Release.bat`
(`cargo build --release`); the executable is created in `target\release\`.



## Usage

```
RRFMCLI /UF /source:<sourcefile> /search:<searchpath>
```

Run without parameters to show the help.

|Switch|Description|
|-|-|
|`/UF`|Update file: copy the source file over every file with the **same name** found in the search path and its subfolders|
|`/source:<sourcefile>`|The source file, with its path|
|`/search:<searchpath>`|The folder to search (subfolders included)|

The switches must be given in this order. Wrap paths with spaces in quotes. The source file itself is skipped.

Example: update every `config.ini` under `D:\Sites` with `D:\Templates\config.ini`:

```
RRFMCLI /UF "/source:D:\Templates\config.ini" "/search:D:\Sites"
```

Output:

```
Updated: D:\Sites\Site1\config.ini
Updated: D:\Sites\Site2\config.ini

Total files updated: 2
```

> **Warning:** the files are overwritten without asking and there is no undo. Make a backup first.



## Uninstall

Delete `RRFMCLI.exe`.
