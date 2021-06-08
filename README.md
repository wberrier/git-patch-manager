# Git Patch Manager Overview

Source often needs to be patched while developing that source or
products that use that source.

Git is a great tool for managing source and history (ie: rebase).

Git Patch Manager aims to provide some finishing touches with the git
and git rebase workflows to provide a quilt-like workflow.

# Features

* [x] managed from a single configuration file
* [x] multiple "stacks" and/or "series" within a git repository
* [x] re-generate patches and series files (ie: quilt refresh)
* [ ] apply patches via "git am" for multiple "stacks"
* [ ] verification that generated patches produce desired result

# Other comparable tools

* quilt
  * much of the gpm workflow was based on this traditional workflow.
    The main disadvantage here was the lack of the `patch` tool
    supporting binary formats [1]
* stacked git
  * doesn't support "multiple stacks"
* git-series
  * more of a proof of concept.  Nice that it uses libgit2, but still
    has a bit of work to go
* yocto/devtool/bitbake
  * very similar workflow for projects that are building inside of yocto

1. https://lists.nongnu.org/archive/html/quilt-dev/2007-01/msg00001.html

# TODO

* [ ] add some easy rusty unit tests for iterating

