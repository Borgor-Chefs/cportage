# Structure

- `keys.py` is a shorthand remapping for keys, some of them don't reflect what is printed on the keyboard
- `display.py` is a wrapper around the `curses.window` of sorts, this is mainly to handle data
- `main.py` is the main script

# Program Requirements

This program needs...

- access to the user's `/etc/portage/package.use/` directory
	- this is to write package specify USE flags
		- e.g., if you were to edit the USE flags for `app-containers/docker` you would find the flags in `/etc/portage/package.use/app/containers`
	- would need read & write perms
- access to the system's `/var/db/pkg/` directory
	- reading all of the packages and listing them on the screen
	- would need read perms
- access to the user's `/etc/portage/make.conf` file
	- to display and change your global use flags
	- would need read & write perms

>Note that root access is **not required**, but what is required is some user that has access to these files/directories
>this hypothetical user should not be able to execute anything

