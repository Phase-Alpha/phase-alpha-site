---
title: Getting Helheim Emacs Running on Air-Gapped Machines
description: Vendoring packages and running a separate branch so Helheim Emacs works on machines that will never see the internet.
date: 2026-01-01
image_path: '/helheim_offline.jpeg'
tags: [tech]
---

I've lost plenty of evenings to tweaking Emacs configuration files, and I'd do it again. What Emacs has never handled well is starting up with no internet access, and that's the wall I hit with Helheim.

[Helheim Emacs](https://github.com/anuvyklack/helheim-emacs) is a modal editing configuration built on [Hel](https://github.com/anuvyklack/hel), a Helix emulation layer, with Org-mode, sensible buffer handling, and Eglot for language servers. I like it. The catch is Elpaca, the package manager it leans on to pull everything down from the internet the first time you run it. Point it at a machine with no network and it just sits there failing to clone packages. I have machines like that: servers and workstations that will never be online, and I wanted the same editor on them, not some stripped-down version.

So I keep a fork, [jigypeper/helheim-custom](https://github.com/jigypeper/helheim-custom), split across three branches doing three different jobs. `upstream` tracks the real Helheim project commit for commit, and a `.upstream-ref` file in the repo just holds the SHA it's currently synced to. `main` is where my own tweaks live on top of a normal, online setup. `airgapped` is the one that actually runs on the machines with no network.

Getting airgapped working wasn't really about writing a bundling script. It was about vendoring. All 83 packages Helheim would normally fetch through Elpaca sit in `site-lisp/` as ordinary files, committed straight into the repo. There's nothing left to download because nothing needs downloading.

That sounds simple, and mostly it was, except that copying someone else's package directory into your own repo tends to drag their `.git` folder in with it, so git records the whole thing as a gitlink instead of actual files. The packages looked like they were there and weren't. It took a commit titled `fix: add new site-lisp packages as real files (were recorded as gitlinks)` before I noticed and sorted it properly.

Getting a build out to the airgapped machines goes through GitHub Actions rather than me tarring things up by hand. Two release workflows do this: one fires on `v*` and `upstream/*` tags and builds whichever branch got tagged, the other only fires on `airgapped/v*` tags and explicitly checks out the `airgapped` branch first. Tag `airgapped/v2026.07.10`, wait for the action, download the tarball, extract it on a machine that will never see a network connection. The archive skips `.git`, compiled `.elc` files, and the generated `.user-lisp-autoloads.el`, and that's really all it needs to skip.

Even that had a dumb bug in it. The first version of the tar command ran inside the same directory it was writing the archive into, so tar tried to include the file it hadn't finished writing yet and choked on referencing itself. Fixed by writing to `/tmp` first and moving the finished tarball into place afterward. Small thing, but the kind of bug you only catch by actually running the release against a real checkout.

<img src="/helheim_offline.jpeg" class="image fit">

The `airgapped` branch also carries its own changes to `init.el`, separate from anything about packaging. Font setup runs directly at startup instead of waiting on a daemon and client to attach, since there's no client coming. Tree-sitter mode remaps get stripped unconditionally rather than only when tree-sitter happens to be missing, on the assumption the machine won't have the grammars either way. `org-directory` points at a plain local notes folder instead of an Obsidian vault, and `org-vault-sync`, which normally syncs notes over git on startup and shutdown, is dropped entirely, since there's no network for it to sync over. The Helheim wrappers for C++, Emacs Lisp, and JSON are gone in favor of Emacs's own built-in modes, plus the vendored `json-mode` where it's useful. Eglot looks up `clangd` with `executable-find` instead of a path I hardcoded for one machine, and `harper-ls` is gone too, since it's one more thing that wants updates from the internet. Even the GDB attach helper that exists on the upstream branch got dropped on airgapped, along with a handful of smaller fixes that only make sense once you're actually running it daily: an ibuffer predicate that fired at the wrong time, a tab-bar history guard, a header-line face that assumed something it shouldn't have.

Xref gave me trouble more than once. First it needed `consult-xref` required before the xref display function got set, then in a later fix it needed `helheim-xref-lib` explicitly loaded before the remaps, otherwise the functions it was remapping to didn't exist as commands yet. Fixing xref plumbing once doesn't seem to mean it stays fixed.

None of this touches `main` or `upstream`. Run Helheim normally with an internet connection and you'd never see any of it. It only exists because I wanted the same editor on machines that can't have it any other way, which meant treating "no internet" as something to build for rather than work around each time.

If you're dealing with the same problem, the [airgapped branch](https://github.com/jigypeper/helheim-custom/tree/airgapped) is there to look at, though it's shaped around my own machines rather than written to be general-purpose. For everything else, the real project is [anuvyklack/helheim-emacs](https://github.com/anuvyklack/helheim-emacs), and that's where the actual work happens.

[Helheim Emacs](https://github.com/anuvyklack/helheim-emacs)

[Hel - Helix Emulation Layer](https://github.com/anuvyklack/hel)
