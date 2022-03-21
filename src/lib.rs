#[macro_export]
macro_rules! collection {
    ($collection_type:ty) => {{
        <$collection_type>::new()
    }};

    // map-like
    // TODO: Fix
    // ($collection_type:ty { $($key:expr => $value:expr),* $(,)? }) => {{
    //     let temp: $collection_type = core::convert::From::from([$(($key, $value),)*]);
    //     temp
    // }};
    ($($key:expr => $value:expr),* $(,)?) => {{
        core::convert::From::from([$(($key, $value),)*])
    }};

    // set-like
    // TODO: Fix
    // ($collection_type:ty { $($value:expr),* $(,)? }) => {{
    //     let temp: $collection_type = core::convert::From::from([$(($value),)*]);
    //     temp
    // }};
    ($($value:expr),* $(,)?) => {{
        core::convert::From::from([$($value,)*])
    }};
}

#[macro_export]
macro_rules! hash {
    () => {{ collection!{} }};

    // map-like
    ($(map)? of $key_type:ty => $value_type:ty) => {{
        std::collections::HashMap::<$key_type, $value_type>::new()
    }};
    ($($key:expr => $value:expr),* $(,)?) => {{
        std::collections::HashMap::from([$(($key, $value),)*])
    }};
    ($(map)? of $key_type:ty => $value_type:ty { $($key:expr => $value:expr),* $(,)? }) => {{
        std::collections::HashMap::<$key_type, $value_type>::from([$(($key, $value),)*])
    }};

    // set-like
    ($(set)? of $value_type:ty) => {{
        std::collections::HashSet::<$value_type>::new()
    }};
    ($($value:expr),* $(,)?) => {{
        std::collections::HashSet::from([$($value,)*])
    }};
    ($(set)? of $value_type:ty { $($value:expr),* $(,)? }) => {{
        std::collections::HashSet::<$value_type>::from([$($value,)*])
    }};
}

#[macro_export]
macro_rules! btree {
    () => {{ collection!{} }};

    // map-like
    ($(map)? of $key_type:ty => $value_type:ty) => {{
        std::collections::BTreeMap::<$key_type, $value_type>::new()
    }};
    ($($key:expr => $value:expr),* $(,)?) => {{
        std::collections::BTreeMap::from([$(($key, $value),)*])
    }};
    ($(map)? of $key_type:ty => $value_type:ty { $($key:expr => $value:expr),* $(,)? }) => {{
        std::collections::BTreeMap::<$key_type, $value_type>::from([$($key, $value)*])
    }};

    // set-like
    ($(set)? of $value_type:ty) => {{
        std::collections::BTreeSet::<$value_type>::new()
    }};
    ($($value:expr),* $(,)?) => {{
        std::collections::BTreeSet::from([$($value,)*])
    }};
    ($(set)? of $value_type:ty { $($value:expr),* $(,)? }) => {{
        std::collections::BTreeSet::<$value_type>::from([$($value,)*])
    }};
}
