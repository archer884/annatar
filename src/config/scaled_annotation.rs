use artano::Annotation;
use regex::Regex;

#[derive(Debug)]
pub struct ScaledAnnotation {
    pub scale_multiplier: f32,
    pub annotation: Annotation,
}

impl ScaledAnnotation {
    fn new(scale_multiplier: f32, annotation: Annotation) -> Self {
        Self {
            scale_multiplier,
            annotation,
        }
    }
}

pub struct ScaledAnnotationParser {
    pattern: Regex,
}

impl ScaledAnnotationParser {
    pub fn new() -> Self {
        Self {
            pattern: Regex::new(r#"\\(?P<scale>\d+(\.\d+)?)\s+(?P<caption>.+)"#).unwrap(),
        }
    }

    pub fn bottom(&self, scale: f32, s: &str) -> ScaledAnnotation {
        let (annotation_scale, annotation) = parse_scaled_annotation(&self.pattern, s);
        ScaledAnnotation::new(
            annotation_scale.unwrap_or(scale),
            Annotation::bottom(annotation),
        )
    }

    pub fn middle(&self, scale: f32, s: &str) -> ScaledAnnotation {
        let (annotation_scale, annotation) = parse_scaled_annotation(&self.pattern, s);
        ScaledAnnotation::new(
            annotation_scale.unwrap_or(scale),
            Annotation::middle(annotation),
        )
    }

    pub fn top(&self, scale: f32, s: &str) -> ScaledAnnotation {
        let (annotation_scale, annotation) = parse_scaled_annotation(&self.pattern, s);
        ScaledAnnotation::new(
            annotation_scale.unwrap_or(scale),
            Annotation::top(annotation),
        )
    }
}

fn parse_scaled_annotation(pattern: &Regex, s: &str) -> (Option<f32>, String) {
    // The plan here is to provide in-band scaling per-annotation via the following format: we
    // check the beginning of any given annotation for \<float>. If we find that, we treat it
    // as entirely separate from the annotation, up to and including all trailing white space.
    //
    // For instance, an annotation of the form `\1.2    frenchmen can't spell` would equate to
    // the message "frenchmen can't spell" at a size multiplier of 1.2.
    //
    // It's valid to escape the leading \ with \\.

    // Leading slash is escaped.
    if s.starts_with("\\\\") {
        return (None, s[1..].into());
    }

    // Annotation does not contain in-band scaling.
    if !s.starts_with('\\') {
        return (None, s.into());
    }

    match pattern.captures(s) {
        None => (None, s.into()),

        // These unwrap assumptions may look horrendously unsafe, but the design of the regular
        // expression itself makes failure here very unlikely.
        Some(captures) => (
            Some(
                captures
                    .name("scale")
                    .and_then(|s| s.as_str().parse().ok())
                    .unwrap(),
            ),
            captures.name("caption").map(|s| s.as_str().into()).unwrap(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{ScaledAnnotation, ScaledAnnotationParser};

    #[test]
    fn in_band_scaling_works() {
        let parser = ScaledAnnotationParser::new();
        let caption = "\\2.0 Hello, world!";
        let ScaledAnnotation {
            scale_multiplier,
            annotation,
        } = parser.top(1.0, caption);

        assert_eq!(2.0, scale_multiplier);
        assert_eq!("Hello, world!", annotation.text);
    }

    #[test]
    fn escaped_leading_slashes_work() {
        let parser = ScaledAnnotationParser::new();
        let caption = "\\\\ESCAPED SLASHES!\\";

        let ScaledAnnotation {
            scale_multiplier,
            annotation,
        } = parser.top(1.0, caption);

        assert_eq!(1.0, scale_multiplier);
        assert_eq!("\\ESCAPED SLASHES!\\", annotation.text);
    }
}
