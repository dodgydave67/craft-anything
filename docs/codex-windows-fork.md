# Craft Anything — Codex Windows build

Unofficial personal build based on Craft Anything by the ArtCraft team.
Source baseline: `f6bc86889cf72b7541520b695f4cdfd6ce32995f` (2026-11-18).
The exact patched commit is in the download's BUILD.txt.

## What changed

- Includes the upstream Codex session-resume implementation absent from v0.6.1. Follow-ups resume the project's saved thread instead of starting over.
- Lets Codex request runtime_ui_read and runtime_ui_walkthrough_record without editing config.toml by hand. Craft Anything's consent cards still decide whether those calls run.
- Directs Clean-room desktop builds to recover recorded observations and the unfinished step, including after a reply such as “done”.
- Makes screenshot reminders respect the latest requested state and live observations. A startup-screen score does not establish editor or behavioural parity.

## Use this download

1. Finish or pause your current build and close all Craft Anything windows.
2. Extract the entire ZIP into a new folder you can write to. Keep both EXE files together.
3. Open craft-anything.exe from that folder. This build uses the normal Craft Anything settings and projects; it does not use an isolated portable settings store. Your installed version remains available. Back up an important project folder before opening it with a newer build.
4. In Settings → AI, check your existing Codex CLI backend and model. Keep your chosen GPT model if your Codex account supports it.
5. If you also use Craft Anything from a standalone Codex terminal, run Connect-Codex.cmd once. It updates only the Craft Anything MCP connection and keeps a backup of the old Codex config. Builds started from this app receive its companion CLI automatically.
6. Open the existing project, keep Clean room selected, and leave the original app running. Send: “Continue the unfinished live observation pass. Recover the handoff and evidence, inspect the current document editor, then implement and verify that state. Do not stop at the uploaded home screen.”
7. Answer any Craft Anything Allow card. When an action cannot be automated, do the specific action requested and reply “done”; it should continue the saved thread.

## Scope and verification

This is an unsigned x64 community build, not an official release. The Windows workflow compiles both programs, runs AI/setup regressions, checks the consent-gated UI tools against a synthetic Windows application, and checks the CLI and desktop version commands before publishing a download. Check the linked workflow result before treating those checks as passed.

Microsoft Publisher and the user's Codex sign-in are not available on the build runner. There is no claim of an end-to-end Publisher recreation test, identical model behaviour, or full Claude Code feature parity. Codex still uses its own sandbox; Claude Code's mid-turn notes, permission hooks and read-only forked questions are separate capabilities. The existing runtime tools automate visible-control actions, not arbitrary typing or drag gestures.

R2.2 personal-fork work; claimed by Codex on branch `bugfix/r2.2-codex-windows-walkthrough`.
