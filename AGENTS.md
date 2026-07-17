# Agent Instructions

## Security Controls

### GPG Signing — NEVER Disable

GPG commit signing is enabled and enforced on this repository. **Do not disable it, bypass it, or work around it under any circumstances.**

If a `git commit`, `git rebase`, or `git cherry-pick` fails with a GPG signing error (timeout, card error, "signing failed"):

1. **Stop. Do not retry with `commit.gpgsign=false` or any other GPG bypass.**
2. **Tell the user:** "GPG signing failed. Please check your YubiKey is plugged in and unlocked, then retry."
3. Wait for the user to confirm before proceeding.

Disabling or bypassing cryptographic signing is a security violation. In a corporate environment this would constitute a disciplinary offence. **Never trade security for expediency.**
