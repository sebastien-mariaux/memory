# MEMORY

Monitor available memory on your Linux system.

When reaching the warning threshold, you will get a notification popup.

When reaching the alert threshold, the program will start closing applications.

## Installation
Just copy the binary file wherever you want.

Run `$ memory&`

You might also find some way to daemonize it so that you don't have to launch it manually.

## Configuration
To generate the configuration file, run `memory config`

This will create a `.memory-config.toml` file in your home directory.

You can modify the settings in this file as you wish. 