use std::{borrow::Cow, ops::Deref, fmt::{self, Display, Formatter}};

use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[macro_export]
macro_rules! create {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(Cow<'static, str>);

        impl $name {
            pub const fn new_owned(value: String) -> Self {
                Self(Cow::Owned(value))
            }

            pub const fn new_static(value: &'static str) -> Self {
                Self(Cow::Borrowed(value))
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.0.deref()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new_owned(value)
            }
        }

        impl From<&'static str> for $name {
            fn from(value: &'static str) -> Self {
                Self::new_static(value)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de> {
                Ok(Self::new_owned(String::deserialize(deserializer)?))
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer {
                self.0.serialize(serializer)
            }
        }
    };
}

create!(BucketId);
create!(FileId);
create!(ApplicationKey);
create!(ApplicationKeyId);