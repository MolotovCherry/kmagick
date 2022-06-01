---
name: Bug report
about: Create a report to help us improve
title: "[BUG] Your bug title here"
labels: bug
assignees: ''

---

## Prerequisites

Please answer the following questions for yourself before submitting an issue.

- [ ] I am running the latest version
- [ ] I checked the documentation and examples and found no answer
- [ ] I checked that it wasn't user error; it was not an ordinary exception that I could've found out and fixed by running `wand.getException()`
- [ ] I checked that this isn't an [ImageMagick](https://github.com/ImageMagick/ImageMagick) bug itself, and is an actual problem with the bindings
- [ ] I understand that kmagick does nothing to the images, but is only a direct binding to the imagmagick c api. I understand that imagemagick itself is the one processing the images. So I've checked and understood how to use the imagemagick c api and made sure the problem isn't due to a misuse or misunderstanding of the api itself
- [ ] I checked to make sure that this issue has not already been filed
- [ ] I'm reporting the issue to the correct repository
- [ ] I'm using the correct architecture (e.g. an x86 emulator runs x86, not arm)

## Context

Please provide any relevant information about your setup. This is important in case the issue is not reproducible except for under certain conditions.

* Android SDK version (if using Android):
* Product (Android Studio, Flutter, etc):
* Kotlin version:
* JDK version:
* ImageMagick version:
* Operating System:

## Expected Behavior

Please describe the behavior you are expecting

## Current Behavior

What is the current behavior?

## Failure Information (for bugs)

Please help provide information about the failure if this is a bug. If it is not a bug, please remove the rest of this template.

### Steps to Reproduce

Please provide detailed steps for reproducing the issue.

1. step 1
2. step 2
3. you get it...

### Failure Logs

Please include any relevant log snippets or files here.

<!-- If this is a crash or similar, please run a logcat to catch the complete error and show the details here. Logcat always displays the exception/crash; sometimes you have to fish around for it though -->
<details><summary><strong>Exception / Backtrace</strong></summary>
<p>

```
<PUT LONG EXCEPTION/BACKTRACE HERE>
```
  
</p>
</details>
