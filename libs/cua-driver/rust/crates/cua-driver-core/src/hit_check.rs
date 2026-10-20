//! The `click` `verify_hit` check: the element a hit test finds at a resolved
//! element's centre must be that element or one of its descendants before the
//! pixel click is posted.

use crate::protocol::ToolResult;

/// How many ancestors of the hit element the check compares with the target.
pub const MAX_HIT_DEPTH: usize = 16;
/// The refusal code when the hit test finds another element at the centre.
pub const HIT_MISMATCH: &str = "hit_mismatch";

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
