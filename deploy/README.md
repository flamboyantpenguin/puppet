# Deployment - Nightmare we all must bear

Greetings. This document explains how this repository handles deployments. Deployment in simple terms means building binaries of the source code.

If you're looking to install this software in your machine, this is not the right place. Read `INSTALL.md` in the project root to learn more about installating the software.

We intent to release builds for all platforms possible with the resources we have. If you wish to see this software available for your OS or architecture, do let us know.

## How is it done?

One word - CI/CD. This project is mainly hosted on GitHub, however it is also mirrored to [DAWN Code Repository](https://code.dawn.org.in). This is to keep a backup and to run CI/CD pipelines on infastructure powered by DAWN. DCR on Gitea also offers fantastic ways to host packages - debian, alpine, fedora, arch etc.

### Edge Builds

Edge builds are binaries built on the latest commit on `master` branch. This is intended for testing and those interested in getting the latest and greatest.

### Release Builds

Release builds are binaries built on the latest tagged version branch - `v*`. This is the actual release version published to users worldwide. OS native packages, Flatpak etc will be built only on release.

## Availability

> [!NOTE]
> We will try to support new packages or system architectures not mentioned here upon request

| Package           | OS      | System Architecture | Auto Updates | Status | Build         |
| ----------------- | ------- | ------------------- | ------------ | ------ | ------------- |
| Windows Installer | Windows | amd64               | ❎           | 🟡     | Release       |
| AppImage          | Linux   | amd64               | ❎           | 🟢     | Release, Edge |
| Debian            | Linux   | amd64               | ✅           | 🔵     | Release       |
| Arch              | Linux   | amd64               | ✅           | 🔵     | Release       |
| Flatpak           | Linux   | amd64               | ✅           | 🔵     | Release       |

- 🔴: Critically broken
- 🔵: Planned, not yet started
- 🟡: In Progress
- 🟢: Available

## Backward Compatibility and glibc

Binaries are ideally built on the oldest platform possible. This makes our software available for most users. Our binaries are built with `glibc` (Yes Rust is best built static, ~~but our dependency `gstreamer` is a problem~~, right now iced-rs can't be built statically - will crash on Wayland. See [#2706 on iced-rs/iced](https://github.com/iced-rs/iced/issues/2706)).

For Arch, Fedora, flatpak etc - this is not a problem - we build on what's latest. However when we build AppImages or debian packages - backward compatibility is necessary.

Thus, we must ideally build our binary on a distro that offers an old glibc - debian 12 is oldstable. But the first place where this software was intended to be used was in a college lab - they were running old versions of Ubuntu. So we decided to build our backward compatible binaries on Ubuntu 22.04 - which still offers support till April 2027.

## Binaries

This section explains the ways in which provide binaries.

### AppImage

AppImages are a great way to distribute software. It removes the concept of install - essentially makes software something as simple as playing an audio or video by clicking on it to run. However, the ease of use it provides is currently balanced by the tremendous effort put forward by the those responsible for building AppImages. Still, thanks to some amazing projects, deployment is easier than it was before.

AppImages work by bundling a directory named AppDir - it consists of all the libraries, plugins and other dependencies needed for a software to run and squashes them into an executable (quite literally). This executable once run, extracts the contents into a temp directory and runs the software with the extracted files.

We use [linuxdeploy](https://github.com/linuxdeploy/linuxdeploy) to deploy our project as an AppImage. It handles AppDir creation and some other functions.

`gstreamer` is a dependency we rely on. So it is necessary to copy `gstreamer` plugins and libraries to the AppDir to get it working - this was done thanks to [linuxdeploy-plugin-gstreamer](https://github.com/linuxdeploy/linuxdeploy-plugin-gstreamer).

AppImages are also convinient for quick testing our software - this is why we plan to build it on edge as well - master branch binaries (unlike release binaries built on code tagges as releases). However, AppImage building is expensive, it comes with a lot of download and install.

Thus, we maintain a container image named [`appimager:${project-name}`](https://code.dawn.org.in/flamboyantpenguin/-/packages/container/appimager/puppet). This container contains dependencies for building our project, and dependencies for building AppImages. This makes it easy for us to just pull the container on each CI pipeline and build the AppImage without installing dependencies again and again.

However, do note that we might be forced to stop building edge AppImages in the future, if the process still becomes expensive.

### Windows

Under development. We do still provide msvc built binaries on packages, however running it needs complicated gstreamer setup.

```txt
puppet
Last Updated: 20 Sep 2026

DAWN/ペンギン
```
