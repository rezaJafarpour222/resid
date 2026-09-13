use crate::{
    font::{loader::Font, shaper::Shaper},
    units::{Direction, Pt},
};
// SECTION: Common
fn load_font(name: &str) -> Font {
    Font::get_font(name).unwrap()
}
const FONTS: &[&str] = &[
    "Estedad",
    "Gandom",
    "Parastoo",
    "Samim",
    "Shabnam",
    "Vazirmatn",
];

// SECTION: loader.rs
#[test]
fn get_font_return_font_with_correct_famliy_and_data() {
    for f in FONTS {
        let font = load_font(f);
        assert_eq!(&font.family, f);
        assert!(!font.data.is_empty());
    }
}

#[test]
fn get_font_returns_error_for_unkown_font() {
    let font = Font::get_font("unknown_font");
    assert!(font.is_err());
}
#[test]
fn all_fonts_supports_persian_alphabet() {
    let persian_alphabet = [
        ('س', "U+0633"),
        ('ل', "U+0644"),
        ('ا', "U+0627"),
        ('م', "U+0645"),
        ('ب', "U+0628"),
        ('پ', "U+067E"),
        ('ت', "U+062A"),
        ('ث', "U+062B"),
        ('ج', "U+062C"),
        ('چ', "U+0686"),
        ('ح', "U+062D"),
        ('خ', "U+062E"),
        ('د', "U+062F"),
        ('ذ', "U+0630"),
        ('ر', "U+0631"),
        ('ز', "U+0632"),
        ('ژ', "U+0698"),
        ('ش', "U+0634"),
        ('ص', "U+0635"),
        ('ض', "U+0636"),
        ('ط', "U+0637"),
        ('ظ', "U+0638"),
        ('ع', "U+0639"),
        ('غ', "U+063A"),
        ('ف', "U+0641"),
        ('ق', "U+0642"),
        ('ک', "U+06A9"),
        ('گ', "U+06AF"),
        ('ن', "U+0646"),
        ('و', "U+0648"),
        ('ه', "U+0647"),
        ('ی', "U+06CC"),
        ('آ', "U+0622"),
        ('ء', "U+0621"),
    ];

    for font_name in FONTS {
        let font = load_font(font_name);
        let face = rustybuzz::Face::from_slice(font.data, 0)
            .unwrap_or_else(|| panic!("failed to create font face for {font_name}"));

        for (character, codepoint) in persian_alphabet {
            assert!(
                face.glyph_index(character).is_some(),
                "font '{font_name}' is missing Persian character '{character}' ({codepoint})"
            );
        }
    }
}
#[test]
fn all_fonts_support_english_alphabet() {
    let english_alphabet = [
        ('A', "U+0041"),
        ('B', "U+0042"),
        ('C', "U+0043"),
        ('D', "U+0044"),
        ('E', "U+0045"),
        ('F', "U+0046"),
        ('G', "U+0047"),
        ('H', "U+0048"),
        ('I', "U+0049"),
        ('J', "U+004A"),
        ('K', "U+004B"),
        ('L', "U+004C"),
        ('M', "U+004D"),
        ('N', "U+004E"),
        ('O', "U+004F"),
        ('P', "U+0050"),
        ('Q', "U+0051"),
        ('R', "U+0052"),
        ('S', "U+0053"),
        ('T', "U+0054"),
        ('U', "U+0055"),
        ('V', "U+0056"),
        ('W', "U+0057"),
        ('X', "U+0058"),
        ('Y', "U+0059"),
        ('Z', "U+005A"),
        ('a', "U+0061"),
        ('b', "U+0062"),
        ('c', "U+0063"),
        ('d', "U+0064"),
        ('e', "U+0065"),
        ('f', "U+0066"),
        ('g', "U+0067"),
        ('h', "U+0068"),
        ('i', "U+0069"),
        ('j', "U+006A"),
        ('k', "U+006B"),
        ('l', "U+006C"),
        ('m', "U+006D"),
        ('n', "U+006E"),
        ('o', "U+006F"),
        ('p', "U+0070"),
        ('q', "U+0071"),
        ('r', "U+0072"),
        ('s', "U+0073"),
        ('t', "U+0074"),
        ('u', "U+0075"),
        ('v', "U+0076"),
        ('w', "U+0077"),
        ('x', "U+0078"),
        ('y', "U+0079"),
        ('z', "U+007A"),
    ];

    for font_name in FONTS {
        let font = load_font(font_name);
        let face = rustybuzz::Face::from_slice(font.data, 0)
            .unwrap_or_else(|| panic!("failed to create font face for {font_name}"));

        for (character, codepoint) in english_alphabet {
            assert!(
                face.glyph_index(character).is_some(),
                "font '{font_name}' is missing English character '{character}' ({codepoint})"
            );
        }
    }
}
#[test]
fn all_fonts_support_numbers() {
    let numbers = [
        ('0', "U+0030"),
        ('1', "U+0031"),
        ('2', "U+0032"),
        ('3', "U+0033"),
        ('4', "U+0034"),
        ('5', "U+0035"),
        ('6', "U+0036"),
        ('7', "U+0037"),
        ('8', "U+0038"),
        ('9', "U+0039"),
    ];

    for font_name in FONTS {
        let font = load_font(font_name);
        let face = rustybuzz::Face::from_slice(font.data, 0)
            .unwrap_or_else(|| panic!("failed to create font face for {font_name}"));

        for (character, codepoint) in numbers {
            assert!(
                face.glyph_index(character).is_some(),
                "font '{font_name}' is missing number '{character}' ({codepoint})"
            );
        }
    }
}
#[test]
fn all_fonts_support_common_characters() {
    let characters = [
        (' ', "U+0020"),
        ('!', "U+0021"),
        ('"', "U+0022"),
        ('#', "U+0023"),
        ('$', "U+0024"),
        ('%', "U+0025"),
        ('&', "U+0026"),
        ('\'', "U+0027"),
        ('(', "U+0028"),
        (')', "U+0029"),
        ('*', "U+002A"),
        ('+', "U+002B"),
        (',', "U+002C"),
        ('-', "U+002D"),
        ('.', "U+002E"),
        ('/', "U+002F"),
        (':', "U+003A"),
        (';', "U+003B"),
        ('<', "U+003C"),
        ('=', "U+003D"),
        ('>', "U+003E"),
        ('?', "U+003F"),
        ('@', "U+0040"),
        ('[', "U+005B"),
        ('\\', "U+005C"),
        (']', "U+005D"),
        ('^', "U+005E"),
        ('_', "U+005F"),
        ('`', "U+0060"),
        ('{', "U+007B"),
        ('|', "U+007C"),
        ('}', "U+007D"),
        ('~', "U+007E"),
    ];

    for font_name in FONTS {
        let font = load_font(font_name);
        let face = rustybuzz::Face::from_slice(font.data, 0)
            .unwrap_or_else(|| panic!("failed to create font face for {font_name}"));

        for (character, codepoint) in characters {
            assert!(
                face.glyph_index(character).is_some(),
                "font '{font_name}' is missing character '{character}' ({codepoint})"
            );
        }
    }
}
#[test]
fn reads_units_per_em() {
    for f in FONTS {
        let font = load_font(f);
        let units = font.units_per_em().expect("failed to read unit_per_em()");
        assert!(units == 2048)
    }
}
#[test]
#[should_panic(expected = "Font is not available")]
fn unit_per_em_return_error_for_invalid_font() {
    let font = Font::get_font("invalid_font").unwrap();
    let units = font.units_per_em();
    assert!(units.is_err())
}

// SECTION: shaper.rs
#[test]
fn shape_returns_glyphs() {
    for f in FONTS {
        let font = load_font(f);
        let text = "سلام دنیا";

        let glyphs =
            Shaper::shape_glyphs(&font, text, Direction::RTL).expect("failed to shape text");

        assert!(!glyphs.is_empty());
    }
}

#[test]
fn shaped_text_contains_original_text() {
    for f in FONTS {
        let font = load_font(f);
        let text = "سلام دنیا";

        let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
            .expect("failed to shape the text");

        assert_eq!(shaped_text.text, text);
    }
}

#[test]
fn shaped_text_has_glyphs() {
    for f in FONTS {
        let font = load_font(f);
        let text = "سلام دنیا";

        let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
            .expect("failed to shape the text");

        assert!(!shaped_text.glyphs.is_empty());
    }
}

#[test]
fn shaped_text_has_positive_width() {
    for f in FONTS {
        let font = load_font(f);
        let text = "سلام دنیا";

        let shaped_text = Shaper::shaped_text(&font, text, Direction::RTL, Pt(14.0))
            .expect("failed to shape the text");

        assert!(shaped_text.width.value() > 0.0);
    }
}
