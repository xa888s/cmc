#[macro_export]
macro_rules! next_parse {
    ($i:ident, $($s:ident: $t:ty),+) => {
        $(let $s: $t = $i
            .next()
            .ok_or(crate::error::CrackmeError::NotFound(stringify!($s)))?
            .parse()
            .map_err(|_| crate::error::CrackmeError::DetailParse(stringify!($s)))?;)+
    };
}
