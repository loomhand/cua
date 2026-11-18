// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Cua AI, Inc.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CaptureSource {
    Window,
    Screen { occluded: bool },
}

impl CaptureSource {
    pub(crate) fn occluded(self) -> bool {
        matches!(self, Self::Screen { occluded: true })
    }

    pub(crate) fn composited(self) -> bool {
        matches!(self, Self::Screen { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::CaptureSource;

    #[test]
    fn only_a_screen_copy_is_composited_and_only_a_covered_one_is_occluded() {
        for (source, occluded, composited) in [
            (CaptureSource::Window, false, false),
            (CaptureSource::Screen { occluded: false }, false, true),
            (CaptureSource::Screen { occluded: true }, true, true),
        ] {
            assert_eq!(source.occluded(), occluded, "{source:?}");
            assert_eq!(source.composited(), composited, "{source:?}");
        }
    }
}
