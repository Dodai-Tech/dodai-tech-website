//! Property: any series of sane numbers draws as an SVG `points` string without NaN or inf, and
//! has one point per value (or none for fewer than two).
#![no_main]
use dodaitech_website::charts::{poly, pts};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|d: (Vec<i16>, u16, u16, u16)| {
    let (v, w, h, max) = d;
    let v: Vec<f32> = v.iter().map(|&x| f32::from(x)).collect();
    let p = pts(&v, f32::from(w), f32::from(h), f32::from(max.max(1)));
    assert_eq!(p.len(), if v.len() < 2 { 0 } else { v.len() });
    let text = poly(&p);
    assert!(!text.contains("NaN") && !text.contains("inf"), "{text}");
});
