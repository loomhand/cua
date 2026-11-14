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
| rebase Chromium screen extents only from a frame the size of the target window | [#5414](https://github.com/trycua/cua/pull/5414) |
| Windows: wait 40 ms after a foreground desktop click, as after a background one | [#5410](https://github.com/trycua/cua/pull/5410) |
| macOS: bring_to_front returns at once when the exact window is already the focused frontmost window | [#5411](https://github.com/trycua/cua/pull/5411) |
| macOS: bring_to_front skips activation only when AppKit agrees the target is frontmost | [#5411](https://github.com/trycua/cua/pull/5411) |
| Windows: a desktop click that closes its own target window succeeds | [#5412](https://github.com/trycua/cua/pull/5412) |
| macOS and Windows: `click` with `verify_hit` fronts an element's window and posts a pixel click at its centre only when the hit test there finds the element, and on macOS like any desktop click refuses a point on a security or consent prompt (upstream [#7338](https://github.com/trycua/cua/pull/7338)); a Linux `click` refuses `verify_hit` as an unknown argument | RFC [#6031](https://github.com/trycua/cua/issues/6031) |
| macOS: background text to kitty goes in as an AXValue write, which kitty commits to its pty without a key window; a blank kitty read-back is unverified, not zero delivered | [#6029](https://github.com/trycua/cua/pull/6029) |
| macOS: find a window on another Space through the app's focused and main window before the remote-token probe, and note when an own-renderer app's off-Space pixels are stale | [#6030](https://github.com/trycua/cua/pull/6030) |
| macOS: `list_windows` ranks `z_index` by the live on-screen order and reports `null` for off-screen windows, so the maximum is the frontmost window (third-party pull request, author kept); upstream's contiguous renumbering after filters ([#2172](https://github.com/trycua/cua/pull/2172)) keeps that order and leaves `null` alone | [#5395](https://github.com/trycua/cua/pull/5395) |
| macOS: that live order comes from an ids-only on-screen window list, read only where `z_index` is read, so no other window enumeration makes a second pass | fork only |
| macOS: a pixel action takes the capture scale that a read of the same window at the same bounds, under the same display layout, already proved, instead of capturing the window again | fork only |
| macOS: the background left-click route stamps the event's window location with the window-local point, as WindowServer stamps a real click, not the screen point; an app that reads that field (WeChat 4.1 on a two-display Mac) clicked off by the window's origin; upstream [#5492](https://github.com/trycua/cua/pull/5492) stamps it on macOS 27 only, and this patch drops that version gate | [#6126](https://github.com/trycua/cua/pull/6126) |
| macOS: a background pixel click on a text input whose AXPress leaves it unfocused (a Qt line edit takes the press as a no-op) gives it the keyboard focus, as a real click does | [#6118](https://github.com/trycua/cua/pull/6118) |
| Linux: a launch settles on the window of a process its launcher forked (LibreOffice's oosplash), and a command line matches a window class by its program, not its last argument | [#6119](https://github.com/trycua/cua/pull/6119) |
| Linux: with no desktop variable set, a session whose GNOME Shell owns its bus name counts as GNOME, so the advertisement is IsEnabled only and Orca does not start | [#6120](https://github.com/trycua/cua/pull/6120) |
| macOS: a desktop capture wakes a display that is off and waits out the login shield, or refuses with `display_asleep`, instead of trapping in ScreenCaptureKit (SIGILL) | [#6157](https://github.com/trycua/cua/pull/6157) |
| macOS: background `type_text` and `press_key` with `locked_input:true` send pid-routed keys while the screen is locked, unverified; every other route still refuses with `screen_locked` | [#6158](https://github.com/trycua/cua/pull/6158) |
| macOS: a `locked_input` reply names where the keys went: the lock screen for loginwindow's pid, otherwise the key window of that pid as it was before the lock | with [#6158](https://github.com/trycua/cua/pull/6158) |
| macOS: the key-window records `type_text` posts never defocus the window they make key, so a Qt window without a focused accessibility element (WeChat) keeps its focused field | [#6156](https://github.com/trycua/cua/pull/6156) |
| Linux: a pid whose app did not answer `GetChildren` gets `app_unresponsive` at once, without a walk, until a `GetState` probe to its root gets a reply | [#6991](https://github.com/trycua/cua/pull/6991) |
| Windows: a service-mode daemon claims its pipe name with its first instance (`FILE_FLAG_FIRST_PIPE_INSTANCE`), as an embedded one already did, so it never joins a name another server holds | fork only, reported upstream |
| Windows: the daemon's clients refuse a named pipe whose server process runs as another Windows account | fork only, reported upstream |
| Windows: `type_text`'s tail repair and read-back accept text that replaced a selection, so the repair no longer types a shared end again ("abcc"), and an unchanged value is reported as dropped or typed over an identical selection, not as a firm drop; the read-back also matches a multi-line text that an Edit control stores with CRLF | [#7065](https://github.com/trycua/cua/pull/7065) |
| all platforms: `type_text` takes `literal`, which types the text without stripping a trailing run of unbalanced agent-protocol closing tags, for a client that splits a text into lines and checks the whole text once | [#6994](https://github.com/trycua/cua/pull/6994) |
| Windows: no-element `type_text` writes through the focused UIA document at its caret or selection (one selection, at most 1M characters), with read-back and a verified caret, instead of Unicode packets that Windows 11 Notepad collapsed; the write leaves no modified marker or undo step and skips Autocorrect | [#6995](https://github.com/trycua/cua/pull/6995) |
| Windows: a focused-document probe that has not answered within 2 s, or that is still running from the previous call, fails the call with nothing written and no keys sent, instead of falling back to Unicode keys that a busy Notepad replays garbled | [#6996](https://github.com/trycua/cua/pull/6996) |
| Linux: XTest typing (desktop and foreground `type_text`) turns Caps Lock off for the text and back on after it, so a locked Caps Lock no longer inverts the text's case | [#6817](https://github.com/trycua/cua/pull/6817) |
| Linux: a window capture reads only the part of the window on the screen and pads the rest black, so a window past the screen edge no longer fails with BadMatch | the same fix is third-party [#5742](https://github.com/trycua/cua/pull/5742) |
| Linux: with `CUA_DRIVER_RS_REMOVE_MPX_MASTERS=1` in the daemon's environment, the XInput2 master pairs the driver takes are removed when released, never pooled, as before upstream [#7358](https://github.com/trycua/cua/pull/7358): each session adds a fresh pair, end_session, the idle teardown and the popup waiter remove it, a failed attach removes it, and startup removes pairs whose holder, or whose free pair's creator, is gone; unset, upstream's pooling is unchanged. Removal crashes running Qt 6.5+ apps ([#7325](https://github.com/trycua/cua/issues/7325)); a pooled master keyboard makes Chrome on X11 drop core keyboard input | [#7501](https://github.com/trycua/cua/pull/7501) |
| Linux: a window whose frame matches the window geometry exactly, while no other frame does, is named without the 24 px margin, so cascaded same-size windows of one pid (Chrome) no longer fall back to a walk of the whole application | [#6983](https://github.com/trycua/cua/pull/6983) |
| macOS: the permission guide, its banner and its status lines name the app the driver runs in (the shown name of the nearest enclosing .app), else Cua Driver | [#6985](https://github.com/trycua/cua/pull/6985) |
| macOS: the permission guide opens at the left edge of the visible frame, clear of the Settings switches and the access prompt | [#6984](https://github.com/trycua/cua/pull/6984) |
| macOS: a batched `type_text` is retyped only when a native field shows its old value followed by the text's first characters, never in a terminal, and an unconfirmed terminal insert reports no delivered count; a field that showed no change is not retyped here, while the upstream pull request keeps upstream's retype of it | [#7276](https://github.com/trycua/cua/pull/7276) |
| macOS: an unconfirmed terminal insert says a terminal may not echo what it reads and the text must not be typed again, with no foreground escalation | [#7277](https://github.com/trycua/cua/pull/7277) |
| macOS: a read-back that changed without growing is unverifiable; an `AXSelectedText` write that changed the field is not followed by keys, and the key rung gives up once the value stops moving | issue [#7244](https://github.com/trycua/cua/issues/7244) |
| macOS: a terminal confirms typed text only by a copy its scrollback did not already hold, and never when its value could not be read before the keys | [#7278](https://github.com/trycua/cua/pull/7278) |

Before a box image moves its cua-driver pin to `dev`:

- a desktop box has 19 free keycodes, and `dev` can use 18 of them, since it holds back 8;
- the window routes (XSendEvent and XTest to the focused window) type a text with more distinct missing characters than free spare keycodes in batches, upstream since [#5168](https://github.com/trycua/cua/pull/5168);
- the virtual-keyboard route and targeted XTest still fail such a text before typing anything; measured before #5168, a single call with 32, 64 or 120 distinct CJK characters typed 0 of 3, and calls of at most 8 distinct characters typed 18 of 18;
- so the pin moves once the client's typing goes through the window routes, or keeps runs of at most 8 distinct characters on the other routes.

Upstream already:

- wait for the spawned process's environment in the NO_AT_BRIDGE test ([#7080](https://github.com/trycua/cua/pull/7080), which superseded [#7067](https://github.com/trycua/cua/pull/7067));
- confirm the Linux virtual keyboard's key events run by run and unplug it when keys stay unread ([#7145](https://github.com/trycua/cua/pull/7145), salvaged from [#7076](https://github.com/trycua/cua/pull/7076)), and keep borrowed keycodes bound until 150 ms after a text's last key on them ([#7146](https://github.com/trycua/cua/pull/7146), salvaged from [#7079](https://github.com/trycua/cua/pull/7079));
- read the macOS keyboard layout on the main queue in a process that runs AppKit, so `press_key` no longer traps in Text Input Sources ([#6939](https://github.com/trycua/cua/pull/6939), which replaced the fork's own fix for the same trap);
- capture the macOS desktop with captureImage(in:) when there is no overlay to leave out ([#6881](https://github.com/trycua/cua/pull/6881), from [#6028](https://github.com/trycua/cua/pull/6028));
- stop the macOS launch watchdog once the driver itself activates an app ([#6803](https://github.com/trycua/cua/pull/6803)'s focus arbiter, which replaced the fork's launch hold);
- never borrow keycode 8 for a Linux text character ([#5595](https://github.com/trycua/cua/pull/5595), from [#5592](https://github.com/trycua/cua/pull/5592));
- stop the Linux focus restore when the saved window is gone ([#5596](https://github.com/trycua/cua/pull/5596), from [#5593](https://github.com/trycua/cua/pull/5593));
- paint the macOS cursor overlay as a region around the cursors, not a full-screen frame ([#5650](https://github.com/trycua/cua/pull/5650), from [#5640](https://github.com/trycua/cua/pull/5640));
- restore the X11 active window by bouncing the core focus when only the window manager's record moved ([#5651](https://github.com/trycua/cua/pull/5651), from [#5642](https://github.com/trycua/cua/pull/5642));
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
