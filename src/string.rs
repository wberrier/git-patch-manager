// Some various string helpers

// TODO: is it bad form to have a "string" module?
// TODO: should this be an impl on str?

// Maybe there's more string algorithms out there?

// Woah, got a variation of this from stackoverflow
// https://stackoverflow.com/questions/38447780/how-to-crop-characters-off-the-beginning-of-a-string-in-rust/38455374#38455374
// NOTE: this is probably possible to return &str, couldn't figure it out, do this for now...
// Remove everything before a certain character, and if that char isn't found, return the input string
pub fn everything_before(s: &str, needle: &str) -> String {
    match s.find(needle) {
        Some(index) => s[..index].to_string(),
        None => s.to_string(),
    }
}
