# Changelog

## 0.5
- Can have custom scenes added so you for instance have a goto ship
- Some breaking renames
- Seperate the display text, from the text link
- ugly code cleanup

## 0.4.4
- `TooltipMap` not beining initialised no longer causes a crash.


## 0.4.3
- Stop putting `TooltipLinkTimer` on every component that is hovered.


## 0.4.2
- Tooltips no longer timeout from lack of user interaction while they are being hovered upon.


## 0.4.1
- Locked tooltips can now be moved by clicking and dragging.


## 0.4.0
- Update to bevy 0.19.

## 0.3.1
- Silence potential expected error on startup.

## 0.3.0
- Update to bevy 0.18.
- Observers to only spawned for their usecase.
- Use new 0.18 textspan observers instead of crates emulation.
- The default time until despawn has been decreased.

## 0.2.4
- New systemparams for getting tooltips parts.
- Documentation fixes, mostly missing trailing fullstops.

## 0.2.3
- revert hover checks only running when cursor moved.

## 0.2.2
- Warning about text component needing to be added with links.

## 0.2.1
- Tiny bail features now added as crate features.

## 0.2.0
- Hover check now only runs when cursor has moved.
- Fixed typos and awesome readme badges.
- Added some missing documentation.
- TooltipsData changed the name breaking change.

## 0.1.0
- Initial release.
