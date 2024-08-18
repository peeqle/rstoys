#[macro_export]
macro_rules! format {
    ($ref: tt, $($s: tt),*) => {
        $(
            $ref.replacen("{}",$s, 1);
        )*
    }
}
