// basically to replace
// let thing: u64 = info
//      .next()
//      .and_then(|l| l.inner_html().parse().ok())
//      .ok_or_else(|| anyhow!("No {}!", stringify!($s)))?;
#[macro_export]
macro_rules! next_parse_inner {
    ($i:ident, $s:ident: $t:ty) => {
        let $s: $t = $i
            .next()
            .and_then(|l| l.inner_html().trim().parse().ok())
            .ok_or_else(|| anyhow!("No {}!", stringify!($s)))?;
    };
    ($i:ident, $s:ident: $t:ty $(, $ss:ident: $ts:ty)+) => {
        next_parse_inner! { $i, $s: $t }
        next_parse_inner! { $i $(, $ss: $ts)+ }
    };
}

#[macro_export]
macro_rules! next_parse {
    ($i:ident, $s:ident: $t:ty) => {
        let $s: $t = $i
            .next()
            .and_then(|l| l.parse().ok())
            .ok_or_else(|| anyhow!("No {}!", stringify!($s)))?;
    };
    ($i:ident, $s:ident: $t:ty $(, $ss:ident: $ts:ty)+) => {
        next_parse! { $i, $s: $t }
        next_parse! { $i $(, $ss: $ts)+ }
    };
}

#[macro_export]
macro_rules! next_child {
    ($i:ident, $s:ident) => {
        let $s: &str = $i
            .next()
            .and_then(|n| n.children().nth(1).and_then(|t| {
                scraper::ElementRef::wrap(t).and_then(|t| t.text().next())
            }))
            .ok_or_else(|| anyhow!("No {}!", stringify!($s)))?;
    };
    ($i:ident, $s:ident $(, $ss:ident)+) => {
        next_child! { $i, $s }
        next_child! { $i $(, $ss)+ }
    };
}
