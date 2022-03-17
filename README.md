# curtain

Physical screen locking for macOS Screen Sharing based on [franrogers/curtains](https://github.com/franrogers/curtains).

![](lock-screen-curtain.png)

Lock screen while controlling the Mac remotely.

## Installation

```sh
brew install zhuorantan/curtain/curtain
```

## Usage

### Automatic

To enable automatic locking:

```sh
curtain auto enable -m "Optional message displayed on screen"
```

This will create a launch agent to lock screens when a Screen Sharing session is established.

To disable automatic locking:

```sh
curtain auto disable
```

### Manual

Lock physical screens and input devices:

```sh
curtain lock -m "Optional message displayed on screen"
```

A confirmation prompt would show to make sure you don't lock yourself out of your Mac. To skip the
prompt, use the `-y` option:

```sh
curtain lock -y
```

To unlock:

```sh
curtain unlock
```

## License

[MIT license](LICENSE)
