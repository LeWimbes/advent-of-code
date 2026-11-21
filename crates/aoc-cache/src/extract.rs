//! Extracting data from a puzzle's HTML page.

const ANSWER_START: &str = "Your puzzle answer was <code>";
const ANSWER_END: &str = "</code>";
const CODE_BLOCK_START: &str = "<pre><code>";
const CODE_BLOCK_END: &str = "</code></pre>";

/// The answers submitted so far, in order.
pub(crate) fn answers(page: &str) -> Vec<String> {
    let mut answers = Vec::new();
    let mut rest = page;
    while let Some(start) = rest.find(ANSWER_START) {
        rest = &rest[start + ANSWER_START.len()..];
        let Some(end) = rest.find(ANSWER_END) else {
            break;
        };
        answers.push(decode_html(&rest[..end]));
        rest = &rest[end..];
    }
    answers
}

/// All code blocks on the page, in order.
///
/// Part 2 is appended to the page once part 1 is solved, so the position of a block never changes.
pub(crate) fn code_blocks(page: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut rest = page;
    while let Some(start) = rest.find(CODE_BLOCK_START) {
        rest = &rest[start + CODE_BLOCK_START.len()..];
        let Some(end) = rest.find(CODE_BLOCK_END) else {
            break;
        };
        blocks.push(decode_html(&rest[..end]));
        rest = &rest[end..];
    }
    blocks
}

/// Remove tags such as `<em>` and decode the entities used on the website.
fn decode_html(html: &str) -> String {
    let mut text = String::with_capacity(html.len());
    let mut chars = html.chars().peekable();
    while let Some(c) = chars.next() {
        // Like browsers, only treat `<` as the start of a tag if a letter, `/` or `!` follows.
        // Some pages contain unescaped `<` in code, e.g. `a<2006:qkq` in 2023 day 19.
        let starts_tag = c == '<'
            && chars
                .peek()
                .is_some_and(|&next| next.is_ascii_alphabetic() || next == '/' || next == '!');
        if starts_tag {
            chars.by_ref().find(|&c| c == '>');
        } else {
            text.push(c);
        }
    }

    // `&amp;` has to come last, so that e.g. `&amp;lt;` becomes `&lt;` and not `<`.
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = r#"<main>
<article class="day-desc"><h2>--- Day 1: Test ---</h2>
<p>For example:</p>
<pre><code>1 <em>2</em>
3 &lt;4&gt;
</code></pre>
<p>Something <code>inline</code>.</p>
</article>
<p>Your puzzle answer was <code>42</code>.</p>
<article class="day-desc"><h2 id="part2">--- Part Two ---</h2>
<pre><code><em>a &amp;&amp; b</em></code></pre>
</article>
<p>Your puzzle answer was <code>abc</code>.</p>
</main>"#;

    #[test]
    fn extracts_code_blocks_in_order() {
        assert_eq!(code_blocks(PAGE), ["1 2\n3 <4>\n", "a && b"]);
    }

    #[test]
    fn extracts_answers_in_order() {
        assert_eq!(answers(PAGE), ["42", "abc"]);
    }

    #[test]
    fn handles_pages_without_data() {
        assert_eq!(code_blocks("<p>Nothing here</p>"), Vec::<String>::new());
        assert_eq!(answers("<p>Nothing here</p>"), Vec::<String>::new());
    }

    #[test]
    fn keeps_unescaped_less_than_signs() {
        assert_eq!(
            decode_html("px{a<2006:qkq,m>2090:A,rfg}"),
            "px{a<2006:qkq,m>2090:A,rfg}"
        );
        assert_eq!(decode_html("x < <em>y</em>"), "x < y");
    }

    #[test]
    fn decodes_double_escaped_entities_once() {
        assert_eq!(decode_html("&amp;lt;"), "&lt;");
    }
}
