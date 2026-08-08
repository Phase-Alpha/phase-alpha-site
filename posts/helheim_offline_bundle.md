---
title: Taking Helheim Emacs Offline - A Fork with Offline Bundle Support
description: Adding offline installation capabilities to the Helheim Emacs configuration for machines without internet access.
date: 2026-01-01
image_path: '/helheim_offline.jpeg'
tags: [tech]
---

Emacs is many things—a text editor, a Lisp machine, a way to waste entire evenings tweaking configuration files. What it isn't, traditionally, is something you can easily set up on a machine without internet access. Helheim Emacs, an already excellent modal editor configuration built on top of [Hel](https://github.com/anuvyklack/hel)'s Helix emulation layer, had this problem. I decided to fix it.

## The Problem

[Helheim Emacs](https://github.com/anuvyklack/helheim-emacs) is a modern, opinionated Emacs configuration that combines the best of what Emacs has to offer: modal editing with Hel, Org-mode for knowledge management, intelligent buffer organization, and seamless language server integration. It's built around the philosophy that Emacs shouldn't just be a text editor—it should be your Lisp machine, fully tailored to how you work.

But here's the issue: Helheim relies on [Elpaca](https://github.com/progfolio/elpaca), an Emacs package manager that downloads packages from the internet. On machines with no internet access, you're stuck. Running `emacs --init-directory ./helheim-emacs` would simply fail to fetch the required packages.

This became a practical problem for me. I have servers and workstations in environments where internet access is restricted or non-existent. I wanted to use Helheim on all of them, not just my laptop.

## The Solution: Offline Bundle Support

I forked Helheim Emacs and added a comprehensive offline installation workflow. The approach is straightforward but effective:

1. **On a machine with internet**: Run Emacs with Helheim to download all packages, then run a script that bundles everything into a single tarball.
2. **On the offline machine**: Extract the bundle and launch Emacs.

That's it.

### The Implementation

The fork adds two key components:

**1. create-offline-bundle.sh** — A shell script that handles the bundling process:

```bash
#!/usr/bin/env bash
# Run this after initial Helheim setup on a machine with internet
./create-offline-bundle.sh
```

The script:
- Validates that packages are already downloaded
- Creates a temporary copy of the entire Helheim directory
- Cleans up unnecessary files (git history, caches, temporary files)
- Compresses everything into a dated tarball: `helheim-emacs-offline-20260101.tar.gz`

The cleanup step is important. Git repositories in the Elpaca cache can be large, and removing them saves significant space without losing functionality. The bundles typically end up around 500MB-1GB depending on which packages you've installed.

**2. OFFLINE-INSTALLATION.md** — Comprehensive documentation covering:
- How to create bundles
- System dependencies (Emacs 29.1+, git, ripgrep)
- Font installation
- Optional language servers for IDE features
- Troubleshooting common issues

### Key Considerations

**Architecture Compatibility**: If you create a bundle on an Intel Mac but need to use it on an ARM machine, Emacs will need to recompile some native extensions on first run. This requires a compiler but happens automatically.

**Git is Still Required**: Even offline, some Emacs packages use git internally for version checking or other operations. You need git installed even if you have no network connectivity.

**Language Servers Are Optional**: Helheim integrates with Eglot for language server protocol support. The offline bundle works without them, falling back to dumb-jump (regex-based symbol search), but you can install them on the offline machine for better IDE features.

## Why This Matters

Emacs configurations are deeply personal. After weeks of tuning, your Emacs becomes an extension of how you think and work. Being able to transport that exact configuration—down to every installed package and every compiled extension—to any machine is valuable.

The offline bundle approach solves this elegantly without compromising Helheim's philosophy. It's not a watered-down version or a limited offline mode. It's the complete Helheim experience, bundled and portable.

## What Changed

If you're curious about the technical details, here's what was added to the upstream Helheim:

- **create-offline-bundle.sh**: 71 lines of bash that handle the bundling logic
- **OFFLINE-INSTALLATION.md**: Comprehensive 173-line installation guide
- Minor updates to **init.example.el**: Ensuring Eglot autoloads correctly
- Changes to **xref configuration**: Better handling of fallback search backends

The changes are minimal and don't affect the core Helheim experience. If you run Helheim with internet access, you'll never notice these additions.  

<img src="/helheim_offline.jpeg" class="image fit">

## The Fork

If you want to use this offline bundle functionality, you can find it in my fork:

[jigypeper/helheim-emacs on GitHub](https://github.com/jigypeper/helheim-emacs)

The upstream [anuvyklack/helheim-emacs](https://github.com/anuvyklack/helheim-emacs) remains the official, fully-featured version. My fork is specifically focused on enabling offline usage while staying as close to upstream as possible.

## A Personal Fork

This fork is primarily for my own use—a version of Helheim Emacs tailored to my specific workflow and needs, including the offline bundling capability. If you have similar requirements (offline machines, network-restricted environments, or just want to explore the offline bundle approach), you're welcome to use it. Otherwise, the upstream [anuvyklack/helheim-emacs](https://github.com/anuvyklack/helheim-emacs) is the place to be.

One of the strengths of open-source software is the ability to fork and customize. I'm not trying to replace Helheim or propose this as the "better" version—it's simply Helheim configured for my specific use case. If you find value in the offline bundling approach, feel free to adopt the same pattern for your own fork.

[Helheim Emacs Documentation](https://github.com/anuvyklack/helheim-emacs)

[Hel - Helix Emulation Layer](https://github.com/anuvyklack/hel)
