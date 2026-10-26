# loomhand/cua

A fork of [trycua/cua](https://github.com/trycua/cua) that carries Linux X11, macOS and Windows fixes to `cua-driver`, the screen executor in [loomhand](https://github.com/loomhand) cloud desktops. Every fix is sent upstream; this fork only holds them until they land. Upstream's own README is [README.md](../README.md).

## Branches

| Branch | What |
|---|---|
| `main` | trycua/cua `main`, fast-forward only, never changed here |
| `dev` (default) | `main` plus the patches below and the fork workflows |
| `fix/*` | head branches of the open upstream pull requests |

## Patches on dev

| Patch | Upstream |
|---|---|
| never borrow keycode 8 for a Linux text character | [#5592](https://github.com/trycua/cua/pull/5592) |
| stop the Linux focus restore when the saved window is gone | [#5593](https://github.com/trycua/cua/pull/5593) |
| rebase Chromium screen extents only from a frame the size of the target window | [#5414](https://github.com/trycua/cua/pull/5414) |
| Windows: wait 40 ms after a foreground desktop click, as after a background one | [#5410](https://github.com/trycua/cua/pull/5410) |
| macOS: bring_to_front returns at once when the exact window is already the focused frontmost window | [#5411](https://github.com/trycua/cua/pull/5411) |
| macOS: bring_to_front skips activation only when AppKit agrees the target is frontmost | [#5411](https://github.com/trycua/cua/pull/5411) |
| Windows: a desktop click that closes its own target window succeeds | [#5412](https://github.com/trycua/cua/pull/5412) |
| macOS: capture the desktop with captureImage(in:) when there is no overlay to leave out | [#6028](https://github.com/trycua/cua/pull/6028) |
| Linux: restore the active window by bouncing the core focus when only the window manager's record moved (xfwm4 takes the session keyboard's focus as the desktop's) | [#5642](https://github.com/trycua/cua/pull/5642) |
| macOS: paint the cursor overlay as a tile around the cursors, not a full-screen frame | [#5640](https://github.com/trycua/cua/pull/5640) |
| macOS and Windows: `click` with `verify_hit` fronts an element's window and posts a pixel click at its centre only when the hit test there finds the element | fork only |
| macOS: background text to kitty goes in as an AXValue write, which kitty commits to its pty without a key window; a blank kitty read-back is unverified, not zero delivered | fork only |
| macOS: find a window on another Space through the app's focused and main window before the remote-token probe, and note when an own-renderer app's off-Space pixels are stale | fork only |
| macOS: a background AX write into a native single-line field outside web content sends the net-zero key edit (space, backspace), as a document window gets | removed in v0.38.0-loomhand.3: motivation disproved (WeChat's search box is not in its AX tree), no native field needed it, and it cost 2 keys and 150 ms per eligible AX write |
| macOS: `list_windows` ranks `z_index` by the live on-screen order and reports `null` for off-screen windows, so the maximum is the frontmost window (third-party pull request, author kept) | [#5395](https://github.com/trycua/cua/pull/5395) |

Before a box image moves its cua-driver pin to `dev`:

- a desktop box has 19 free keycodes, and `dev` can use 18 of them, since it holds back 8;
- the window routes (XSendEvent and XTest to the focused window) type a text with more distinct missing characters than free spare keycodes in batches, upstream since [#5168](https://github.com/trycua/cua/pull/5168);
- the virtual-keyboard route and targeted XTest still fail such a text before typing anything; measured before #5168, a single call with 32, 64 or 120 distinct CJK characters typed 0 of 3, and calls of at most 8 distinct characters typed 18 of 18;
- so the pin moves once the client's typing goes through the window routes, or keeps runs of at most 8 distinct characters on the other routes.

Upstream already:

- skip the Space snapshot in the macOS background gate and the window-change detector ([#5712](https://github.com/trycua/cua/pull/5712), salvaged from [#5641](https://github.com/trycua/cua/pull/5641));
- route keys for a popup an MPX click opened through the master keyboard of the session that clicked ([#5652](https://github.com/trycua/cua/pull/5652), from [#5601](https://github.com/trycua/cua/pull/5601));
- leave the driver's own overlay windows out of Windows `list_windows` ([#5566](https://github.com/trycua/cua/pull/5566), from [#5413](https://github.com/trycua/cua/pull/5413));
- keep an ended session's master pointer until the menu its click opened closes ([#5254](https://github.com/trycua/cua/pull/5254), salvaged from [#4966](https://github.com/trycua/cua/pull/4966));
- leave focus on a dialog the clicked app opened ([#5258](https://github.com/trycua/cua/pull/5258), salvaged from [#4965](https://github.com/trycua/cua/pull/4965));
- derive the macOS desktop point ratio from the display mode instead of a screenshot per action ([#5340](https://github.com/trycua/cua/pull/5340));
- wait for the client, by `_NET_WM_PING` with its own serial and per window its keys went to, before using or restoring a borrowed keycode ([#5172](https://github.com/trycua/cua/pull/5172), salvaged from [#4924](https://github.com/trycua/cua/pull/4924));
- type a long text in spare-keycode batches ([#5168](https://github.com/trycua/cua/pull/5168));
- stop a batched text before rebinding keycodes the client has not confirmed reading ([#5244](https://github.com/trycua/cua/pull/5244), salvaged from [#5238](https://github.com/trycua/cua/pull/5238));
- keep the Chromium screen rebase off a top-level frame that matches its window-manager frame ([#4967](https://github.com/trycua/cua/pull/4967), merged as `6e050d720`);
- fail `type_text` before typing anything when a character cannot be typed, with one spare keycode per distinct character ([#5139](https://github.com/trycua/cua/pull/5139), salvaged from [#4922](https://github.com/trycua/cua/pull/4922)), and give each character of a text its own spare keycode ([#5158](https://github.com/trycua/cua/pull/5158), the change [#4923](https://github.com/trycua/cua/pull/4923) proposed).

## Sync policy

`main` follows trycua/cua `main` and `dev` is rebased onto it by hand, by a repository admin:

```bash
git fetch upstream main
git push origin upstream/main:refs/heads/main                # fast-forward only
git switch dev && git rebase upstream/main                   # a patch upstream merged drops out; resolve conflicts here
git push --force-with-lease=refs/heads/dev:origin/dev origin refs/heads/dev:refs/heads/dev
gh workflow list -R loomhand/cua --all -L 1000 --json id,path,state --jq '.[] | select(.state == "active") | select(.path | test("/fork-") | not) | .id' | xargs -n1 gh workflow disable -R loomhand/cua
```

- The Dev Build pre-release is the tag `dev`, so a push names the branch as `refs/heads/dev` in full.
- Upstream's own workflows stay disabled in this fork; only the `Fork:` workflows run. A sync can bring new upstream workflows, which start active, so the last line runs after every push to `main`.
- A sync leaves the `fix/*` branches of open upstream pull requests alone; a branch is rebased only when its pull request needs it.
- `cmgs/*` branches are never rebased by a sync.

## Builds

- `Fork: CI` checks every push to `dev`: format, the `cua-driver` build and the Linux platform unit tests.
- `Fork: Build and Release Dev` publishes `cua-driver` for Linux x86_64 and arm64 as the [Dev Build](https://github.com/loomhand/cua/releases/tag/dev) pre-release at tag `dev`, with `SHA256SUMS` and `build-info.json`, built in Debian 11 like upstream's release.
- `Fork: Release` publishes `cua-driver` for Linux (x86_64, arm64), macOS (arm64, x86_64) and Windows (x86_64) at each `v*-loomhand.*` tag, unsigned, with `SHA256SUMS`; a published release is never overwritten, so consumers pin its archives by sha256.

## License

MIT, as upstream ([LICENSE.md](../LICENSE.md)). The fork's patches are under the same terms.
