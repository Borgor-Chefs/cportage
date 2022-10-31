# About

This program was made to easily (also graphically) change your USE flags for the system and individual packages.

## Features

At the time of writing this, these are the initial features that the program should have;

- the ability to graphically manage the global (`make.conf`) and individual package (`package.use/`) USE flags
	- CRUD operations for individual (`package.use/`) flags

## Program Requirements

This is a python program built atop the `portage` library developed by the Gentoo team and it should be installed by default.

The rest are:

- `urwid` - this is the graphical library being used

## System Modifications

- this program creates a new group called `cportage`
	- you will need to **manually** add users to this group `usermod -aG <user> cportage`
	- this group will have read & write permissions to various files and directories
- this program changes the `make.conf` *file* into a *folder.*
	- for users' sake, the old file will be backed up to `make.conf.old` and left in `/etc/portage/`
	- it will also be copied to `/etc/portage/make.conf/main.conf`
	- the `MAKEOPTS` will be moved into `/etc/portage/make.conf/make.conf`

> See System (File) Requirements

### System (File) Requirements

This program requires `xxxrw-xxx <user> cportage` read & write access to;

- the `/etc/portage/package.use/` directory.
	- this includes subdirectories and subfiles
- the `/etc/portage/make.conf/` **directory**

> `make.conf` can be a directory, see `man make.conf`.
> the files within this directory are summed together to form the usual `make.conf` file

#### package.use

something goes here

#### make.conf

something goes here

