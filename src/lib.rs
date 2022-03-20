#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};

    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

#[macro_export]
macro_rules! hash {
    () => {{ collection!{} }};

    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::HashMap::from([$(($k, $v),)*])
    }};

    // set-like
    ($($v:expr),* $(,)?) => {{
        std::collections::HashSet::from([$($v,)*])
    }};
}

#[macro_export]
macro_rules! btree {
    () => {{ collection!{} }};
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::BTreeMap::from([$(($k, $v),)*])
    }};


    // set-like
    ($($v:expr),* $(,)?) => {{
        std::collections::BTreeSet::from([$($v,)*])
    }};
}

