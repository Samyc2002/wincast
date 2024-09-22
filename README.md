# Wincast

A simple TUI tool built with rust for windows system wide search.
This is similar to spotlight or raycast in MacOS, but for Windows.
This uses a local `db.sqlite` file to store the data, so there is no concern about data theft.

![Demo](demo.gif)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Issues and Suggestions](#issues-and-suggestions)

## Installation

To install using cargo:

```pwsh
$ cargo install wincast
```

To compile from source:

```pwsh
$ git clone https://github.com/Samyc2002/wincast
$ cd wincast
$ cargo build --release
```

For complied binary, please check the [releases page](https://github.com/Samyc2002/wincast/releases)

## Usage

Using the tool is simple. Just run the `wincast` command and you're good to go.


```pwsh
$ wincast
```

*NOTE*: Please make sure to syncronize to index the apps before using the tool. To do so, use the `wincast` command to start the tool and hit `<Ctrl>+s` to syncronize.

## Features

- Indexing
    - Apps (present)
    - Files (in progress)
    - Folders (in progress)
    - Media (in progress)
- Search
    - System wide (present)
    - Google Drive (in progress)
    - Google (present)
    - Youtube (present)
    - YouTube Music (present)
    - Google Maps (in progress)

## Issues and Suggestions

If you encounter any issues, please open an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `bug` label.

If you'd like to have a feature request, please open an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `enhancement` label.

If you'd like to contribute, please open a PR to the `staging branch` and create an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `feature` label and the PR link in the description.

If you'd like to add documentation, please open a PR to the `staging branch` and create an issue [from here](https://github.com/Samyc2002/wincast/issues/new) with a `documentation` label and the PR link in the description.
