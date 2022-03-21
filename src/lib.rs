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
    ($(map)? of $kt:ty => $vt:ty) => {{
        std::collections::HashMap::<$kt, $vt>::new()
    }};
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::HashMap::from([$(($k, $v),)*])
    }};
    ($(map)? of $kt:ty => $vt:ty { $($k:expr => $v:expr),* $(,)? }) => {{
        std::collections::HashMap::<$kt, $vt>::from([$(($k, $v),)*])
    }};

    // set-like
    ($(set)? of $vt:ty) => {{
        std::collections::HashSet::<$vt>::new()
    }};
    ($($v:expr),* $(,)?) => {{
        std::collections::HashSet::from([$($v,)*])
    }};
    ($(set)? of $vt:ty { $($v:expr),* $(,)? }) => {{
        std::collections::HashSet::<$vt>::from([$($v,)*])
    }};
}

#[macro_export]
macro_rules! btree {
    () => {{ collection!{} }};

    // map-like
    ($(map)? of $kt:ty => $vt:ty) => {{
        std::collections::BTreeMap::<$kt, $vt>::new()
    }};
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::BTreeMap::from([$(($k, $v),)*])
    }};
    ($(map)? of $kt:ty => $vt:ty { $($k:expr => $v:expr),* $(,)? }) => {{
        std::collections::BTreeMap::<$kt, $vt>::from([$($k, $v)*])
    }};

    // set-like
    ($(set)? of $vt:ty) => {{
        std::collections::BTreeSet::<$vt>::new()
    }};
    ($($v:expr),* $(,)?) => {{
        std::collections::BTreeSet::from([$($v,)*])
    }};
    ($(set)? of $vt:ty { $($v:expr),* $(,)? }) => {{
        std::collections::BTreeSet::<$vt>::from([$($v,)*])
    }};
}
