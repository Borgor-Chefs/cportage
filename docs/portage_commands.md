# About

Because there's no documentation for Portage's Python API, except from a [Funtoo website](https://www.funtoo.org/Portage_API).

## Commands

These are the list of useful commands I've found

### Get All Installed Packages

```py
portage.db['/']['vartree'].getallnodes()
```

