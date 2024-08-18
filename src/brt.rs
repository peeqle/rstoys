pub(crate) fn brt_std(rgb: &[u8]) -> f32 {
    (0.2126 * rgb[0] as f32) + (0.7152 * rgb[1] as f32) + (0.0722 * rgb[2] as f32)
}
pub(crate) fn brt_lumB(rgb: &[u8]) -> f32 {
    (0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32)
}
pub(crate) fn brt_lumA(rgb: &[u8]) -> f32 {
    f32::sqrt(
        0.299 * ((rgb[0] * rgb[0]) as f32)
            + 0.587 * ((rgb[1] * rgb[1]) as f32)
            + 0.114 * ((rgb[2] * rgb[2]) as f32),
    )
}
