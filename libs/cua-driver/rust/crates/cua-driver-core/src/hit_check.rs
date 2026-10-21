//! The `click` `verify_hit` check: the element a hit test finds at a resolved
//! element's centre must be that element or one of its descendants before the
//! pixel click is posted.

use crate::protocol::ToolResult;

/// How many ancestors of the hit element the check compares with the target.
pub const MAX_HIT_DEPTH: usize = 16;
/// The refusal code when the hit test finds another element at the centre.
pub const HIT_MISMATCH: &str = "hit_mismatch";
/// The refusal code when the hit test itself fails.
pub const HIT_CHECK_UNAVAILABLE: &str = "hit_check_unavailable";
/// The refusal code when the target's window does not come to the front.
pub const FOREGROUND_UNAVAILABLE: &str = "foreground_unavailable";

/// What the hit test at the target's centre found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hit {
    /// The target or an element inside it.
    Target,
    /// Another element.
    Other,
    /// No answer: the hit test or the walk up from it failed.
    Unavailable,
}

/// Fronts the target's window, reads the target's centre (`Ok(None)` when it
/// has none on the screen, `Err` when the read fails), hit-tests there and
/// posts the click, back to back with nothing awaited between them; the point
/// clicked, or the refusal when nothing was sent.
pub fn checked_click<P: Copy + std::fmt::Debug>(
    front: impl FnOnce() -> bool,
    centre: impl FnOnce() -> Result<Option<P>, ()>,
    hit: impl FnOnce(P) -> Hit,
    post: impl FnOnce(P) -> Result<(), String>,
) -> Result<P, ToolResult> {
    if !front() {
        return Err(hit_refusal(
            FOREGROUND_UNAVAILABLE,
            "click.verify_hit: the element's window did not come to the front; nothing was sent."
                .to_owned(),
        ));
    }
    let at = match centre() {
        Ok(Some(at)) => at,
        Ok(None) => {
            return Err(hit_refusal(
                HIT_MISMATCH,
                "click.verify_hit: the element has no on-screen centre; nothing was sent."
                    .to_owned(),
            ))
        }
        Err(()) => {
            return Err(hit_refusal(
                HIT_CHECK_UNAVAILABLE,
                "click.verify_hit: reading the element's centre failed; nothing was sent."
                    .to_owned(),
            ))
        }
    };
    match hit(at) {
        Hit::Target => {}
        Hit::Other => {
            return Err(hit_refusal(
                HIT_MISMATCH,
                format!(
                    "click.verify_hit: the element on top at {at:?} is not the target element or inside it; nothing was sent."
                ),
            ))
        }
        Hit::Unavailable => {
            return Err(hit_refusal(
                HIT_CHECK_UNAVAILABLE,
                format!("click.verify_hit: the hit test at {at:?} failed; nothing was sent."),
            ))
        }
    }
    post(at).map_err(|e| {
        ToolResult::error(format!(
            "click.verify_hit: the click at {at:?} failed after it was sent: {e}"
        ))
    })?;
    Ok(at)
}

/// The depth of the first of `start` and its ancestors that `is_target`
/// accepts, at most `max_depth` parents up; `None` when the walk ends first.
pub fn first_matching_ancestor<T, E>(
    start: T,
    max_depth: usize,
    mut parent: impl FnMut(&T) -> Result<Option<T>, E>,
    mut is_target: impl FnMut(&T) -> Result<bool, E>,
) -> Result<Option<usize>, E> {
    let mut current = start;
    for depth in 0..=max_depth {
        if is_target(&current)? {
            return Ok(Some(depth));
        }
        if depth == max_depth {
            break;
        }
        match parent(&current)? {
            Some(next) => current = next,
            None => break,
        }
    }
    Ok(None)
}

/// The refusal a `verify_hit` click returns when it posts nothing.
pub fn hit_refusal(code: &str, message: String) -> ToolResult {
    ToolResult::error(message.clone()).with_structured(serde_json::json!({
        "status": "refused",
        "code": code,
        "effect": "refused",
        "refusal": { "code": code, "message": message },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn walk(start: u32, target: u32, max_depth: usize) -> Result<Option<usize>, String> {
        first_matching_ancestor(
            start,
            max_depth,
            |n| Ok(n.checked_sub(1)),
            |n| Ok(*n == target),
        )
    }

    #[test]
    fn the_hit_element_itself_or_an_ancestor_within_the_depth_matches() {
        assert_eq!(walk(5, 5, 3), Ok(Some(0)));
        assert_eq!(walk(5, 2, 3), Ok(Some(3)));
    }

    #[test]
    fn an_ancestor_past_the_depth_or_past_the_root_does_not_match() {
        assert_eq!(walk(5, 1, 3), Ok(None));
        assert_eq!(walk(2, 7, 16), Ok(None));
    }

    #[test]
    fn a_failed_read_ends_the_walk_with_its_error() {
        let got = first_matching_ancestor(
            3_u32,
            MAX_HIT_DEPTH,
            |_| Err("parent read failed".to_owned()),
            |n| Ok(*n == 0),
        );
        assert_eq!(got, Err("parent read failed".to_owned()));
    }

    fn run(
        front: bool,
        hit: Hit,
        post: Result<(), String>,
    ) -> (Result<(i32, i32), ToolResult>, Vec<&'static str>) {
        let steps = std::cell::RefCell::new(Vec::new());
        let out = checked_click(
            || {
                steps.borrow_mut().push("front");
                front
            },
            || {
                steps.borrow_mut().push("centre");
                Ok(Some((3, 4)))
            },
            |_| {
                steps.borrow_mut().push("hit");
                hit
            },
            |_| {
                steps.borrow_mut().push("post");
                post
            },
        );
        (out, steps.into_inner())
    }

    fn code(result: &Result<(i32, i32), ToolResult>) -> serde_json::Value {
        let refused = result.as_ref().expect_err("a refusal");
        refused
            .structured_content
            .as_ref()
            .map_or(serde_json::Value::Null, |s| s["code"].clone())
    }

    #[test]
    fn a_check_fronts_reads_the_centre_hits_and_posts_in_that_order() {
        let (out, steps) = run(true, Hit::Target, Ok(()));
        assert_eq!(out.ok(), Some((3, 4)));
        assert_eq!(steps, ["front", "centre", "hit", "post"]);
    }

    #[test]
    fn a_window_that_stays_behind_or_a_failed_or_foreign_hit_sends_nothing() {
        let (out, steps) = run(false, Hit::Target, Ok(()));
        assert_eq!(
            (code(&out), steps),
            (serde_json::json!(FOREGROUND_UNAVAILABLE), vec!["front"])
        );
        let (out, steps) = run(true, Hit::Other, Ok(()));
        assert_eq!(
            (code(&out), steps),
            (
                serde_json::json!(HIT_MISMATCH),
                vec!["front", "centre", "hit"]
            )
        );
        let (out, steps) = run(true, Hit::Unavailable, Ok(()));
        assert_eq!(
            (code(&out), steps),
            (
                serde_json::json!(HIT_CHECK_UNAVAILABLE),
                vec!["front", "centre", "hit"]
            )
        );
    }

    #[test]
    fn a_centre_that_is_gone_or_unreadable_sends_nothing_and_names_which() {
        let posted = std::cell::Cell::new(false);
        let post = |_: (i32, i32)| {
            posted.set(true);
            Ok(())
        };
        let gone = checked_click(|| true, || Ok(None), |_| Hit::Target, post);
        assert_eq!(code(&gone), serde_json::json!(HIT_MISMATCH));
        let unread = checked_click(|| true, || Err(()), |_| Hit::Target, post);
        assert_eq!(code(&unread), serde_json::json!(HIT_CHECK_UNAVAILABLE));
        assert!(!posted.get());
    }

    #[test]
    fn a_target_covered_while_the_cursor_glided_is_not_clicked() {
        let covered = std::cell::Cell::new(false);
        let glide = || covered.set(true);
        glide();
        let posted = std::cell::Cell::new(false);
        let out = checked_click(
            || true,
            || Ok(Some((3, 4))),
            |_| {
                if covered.get() {
                    Hit::Other
                } else {
                    Hit::Target
                }
            },
            |_| {
                posted.set(true);
                Ok(())
            },
        );
        assert_eq!(code(&out), serde_json::json!(HIT_MISMATCH));
        assert!(!posted.get());
    }

    #[test]
    fn a_click_that_failed_after_sending_is_an_error_not_a_refusal() {
        let (out, steps) = run(true, Hit::Target, Err("mouse up".to_owned()));
        assert_eq!(steps, ["front", "centre", "hit", "post"]);
        assert_eq!(code(&out), serde_json::Value::Null);
    }

    #[test]
    fn a_refusal_names_its_code_and_says_nothing_was_sent() {
        let refused = hit_refusal(HIT_MISMATCH, "another element".to_owned());
        let v = serde_json::to_value(&refused).unwrap();
        assert_eq!(v["isError"], true, "{v}");
        let s = &v["structuredContent"];
        assert_eq!(
            (&s["code"], &s["effect"], &s["refusal"]["code"]),
            (
                &serde_json::json!("hit_mismatch"),
                &serde_json::json!("refused"),
                &serde_json::json!("hit_mismatch")
            )
        );
    }
}
