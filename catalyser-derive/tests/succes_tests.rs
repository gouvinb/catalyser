mod something {
    use catalyser_derive::overloads;
    use std::{
        fmt,
        time::Duration,
    };

    #[overloads]
    pub fn add(a: i32, #[default(10)] b: i32) -> i32 {
        a + b
    }

    #[overloads]
    pub fn describe<T: fmt::Debug>(v: T, #[default("value")] label: &str) -> String {
        format!("{label}: {v:?}")
    }

    #[overloads]
    pub fn ports(#[default(80)] http: u16, #[default(http + 363)] https: u16) -> (u16, u16) {
        (http, https)
    }

    #[overloads]
    pub fn as_micros_str(#[default(Duration::from_secs(0))] duration: Duration) -> String {
        duration.as_micros().to_string()
    }

    #[overloads]
    pub fn nothing() -> &'static str {
        "ok"
    }

    #[overloads]
    pub fn opt(#[default(None)] x: Option<u8>, #[default(Some(1))] y: Option<u8>) -> (Option<u8>, Option<u8>) {
        (x, y)
    }
}

use something::*;
use std::time::Duration;

#[test]
fn function() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add!(1, 2), 3);
    assert_eq!(add!(1, b = 2), 3);
    assert_eq!(add!(1), 11);
    assert_eq!(add!(a = 1), 11);
}

#[test]
fn function_with_generic() {
    assert_eq!(describe(5, "n"), "n: 5");
    assert_eq!(describe!(5, "n"), "n: 5");
    assert_eq!(describe!(5, label = "n"), "n: 5");
    assert_eq!(describe!(v = 5, label = "n"), "n: 5");
    assert_eq!(describe!(5), "value: 5");
    assert_eq!(describe!(v = 5), "value: 5");
}

#[test]
fn function_with_combined_defaults() {
    assert_eq!(ports(80, 443), (80, 443));
    assert_eq!(ports!(80), (80, 443));
    assert_eq!(ports!(http = 80), (80, 443));
    assert_eq!(ports!(http = 80, https = 443), (80, 443));
}

#[test]
fn function_with_struct_default_param() {
    assert_eq!(as_micros_str(Duration::from_secs(1)), "1000000");
    assert_eq!(as_micros_str!(Duration::from_secs(1)), "1000000");
    assert_eq!(as_micros_str!(), "0");
    assert_eq!(as_micros_str!(duration = Duration::from_secs(1)), "1000000");
}

#[test]
fn function_without_param() {
    assert_eq!(nothing(), "ok");
    assert_eq!(nothing!(), "ok");
}

#[test]
fn function_with_combined_defaults_and_struct_default_param() {
    assert_eq!(opt(None, None), (None, None));
    assert_eq!(opt!(None, None), (None, None));
    assert_eq!(opt!(), (None, Some(1)));
    assert_eq!(opt!(None), (None, Some(1)));
    assert_eq!(opt!(None, y = None), (None, None));
    assert_eq!(opt!(x = None), (None, Some(1)));
    assert_eq!(opt!(x = None, y = None), (None, None));
}
