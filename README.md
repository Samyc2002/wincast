# Wincast

A simple CLI tool built with rust for windows system wide search.
This is similar to spotlight or raycast in MacOS, but for Windows.
This uses a local `db.sqlite` file to store the data, so there is no concern about data theft.

## Usage

For help text:


```pwsh
$ wincast --help

Usage: wincast.exe [OPTIONS]

Options:
  -q, --query <QUERY>  [default: ]
  -i, --index
  -h, --help           Print help
  -V, --version        Print version
```

To index the apps and files:

```pwsh
$ wincast --index

▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓███████▓▒░   ░▒▓██████▓▒░   ░▒▓██████▓▒░   ░▒▓███████▓▒░ ░▒▓████████▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓████████▓▒░  ░▒▓██████▓▒░     ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░
░▒▓█████████████▓▒░  ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░     ░▒▓█▓▒░

Indexing apps...
Indexing complete
```

To search the apps and files:

```pwsh
$ wincast --query Youtube

▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓███████▓▒░   ░▒▓██████▓▒░   ░▒▓██████▓▒░   ░▒▓███████▓▒░ ░▒▓████████▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓████████▓▒░  ░▒▓██████▓▒░     ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░
░▒▓█████████████▓▒░  ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░     ░▒▓█▓▒░

Matches found: 2/405

  ID   NAME            TYPE   PATH
  1    YouTube         app    [_path_to_Youtube_]
  2    YouTube Music   app    [_path_to_Youtube_Music_]
Enter ID to launch:
```

Entering the number under the ID column will launch the corresponding app.

## Features

- Indexing
    - Apps (present)
    - Files (in progress)
    - Folders (in progress)
    - Media (in progress)
- Search
    - System wide (present)
    - Google Drive (in progress)
    - Google (in progress)
    - Youtube (in progress)
    - YouTube Music (in progress)
    - Google Maps (in progress)

## Issues and Suggestions

If you encounter any issues or suggestions, please open an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `bug` label.

If you'd like to have a feature request, please open an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `enhancement` label.

If you'd like to contribute, please open a PR to the `staging branch` and create an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `feature` label and the PR link in the description.

If you'd like to add documentation, please open a PR to the `staging branch` and create an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `documentation` label and the PR link in the description.
