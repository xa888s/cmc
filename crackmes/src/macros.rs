#[macro_export]
macro_rules! next_parse {
    ($i:ident, $s:ident: $t:ty) => {
        let $s: $t = $i
            .next()
            .ok_or(crate::error::CrackMeError::NotFound(stringify!($s)))?
            .parse()
            .map_err(|_| crate::error::CrackMeError::DetailParse(stringify!($s)))?;
    };

    ($i:ident, $s:ident: $t:ty $(, $ss:ident: $ts:ty)+) => {
        next_parse! { $i, $s: $t }
        next_parse! { $i $(, $ss: $ts)+ }
    };
}
