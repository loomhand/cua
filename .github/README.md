# loomhand/cua

A fork of [trycua/cua](https://github.com/trycua/cua) that carries Linux X11 fixes to `cua-driver`, the screen executor in [loomhand](https://github.com/loomhand) cloud desktops. Every fix is sent upstream; this fork only holds them until they land. Upstream's own README is [README.md](../README.md).

## Branches

| Branch | What |
|---|---|
| `main` | trycua/cua `main`, fast-forward only, never changed here |
| `dev` (default) | `main` plus the patches below and the fork workflows |
| `fix/*` | head branches of the open upstream pull requests; never rebased by automation |
| `cmgs/x11-typing`, `cmgs/backup-20261013/x11-typing` | the earlier patched line; released desktop images pin commits on them, so they stay |

## Patches on dev

| Patch | Upstream |
|---|---|
| fail `type_text` when a character cannot be typed | [#4922](https://github.com/trycua/cua/pull/4922) |
| give each character of a text its own spare keycode | [#4923](https://github.com/trycua/cua/pull/4923) |
| wait for the focused client before using, restoring or rebinding a borrowed keycode | [#4924](https://github.com/trycua/cua/pull/4924) |
| reuse or restore a borrowed keycode only on the reply to its own ping | [#4924](https://github.com/trycua/cua/pull/4924) |
| never borrow the MPX warm-up keycode for a character | fork only |
| never borrow keycode 8, which a virtual keyboard cannot send | fork only |
| leave focus on a dialog the clicked app opened | [#4965](https://github.com/trycua/cua/pull/4965) |
| keep an ended session's master pointer until the menu its click opened closes | [#4966](https://github.com/trycua/cua/pull/4966) |
| stop the focus restore when the saved window is gone | fork only |
| rebase Chromium screen extents only from a frame the size of the target window | fork only |

Upstream already: keep the Chromium screen rebase off a top-level frame that matches its window-manager frame ([#4967](https://github.com/trycua/cua/pull/4967), merged as `6e050d720`).

## Sync policy

- `Fork: Upstream sync` runs daily: it fast-forwards `main` to trycua/cua `main` and rebases `dev` onto it. A patch that upstream merged drops out of the rebase.
- A rebase conflict leaves `dev` as it was and opens an issue with the conflicting files; resolve it by hand on `dev`.
- Upstream's own workflows stay disabled in this fork; only the `Fork:` workflows run.
- `fix/*` and `cmgs/*` branches are never touched by automation.

## Builds

- `Fork: CI` checks every push to `dev`: format, the `cua-driver` build and the Linux platform unit tests.
- `Fork: Build and Release Dev` publishes `cua-driver` for Linux x86_64 and arm64 as the [Dev Build](https://github.com/loomhand/cua/releases/tag/dev) pre-release at tag `dev`, with `SHA256SUMS` and `build-info.json`, built in Debian 11 like upstream's release.

## License

MIT, as upstream ([LICENSE.md](../LICENSE.md)). The fork's patches are under the same terms.
