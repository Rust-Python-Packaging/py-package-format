enum PythonTag {}

enum ABITag {
    None,
}

struct Platform {
    OS: String,
    Architecture: String,
}

struct WheelPackageName {
    distribution: String,
    version: String,
    build_tag: String,
    python_tag: String,
    abi_tag: ABITag,
    platform_tag: Platform,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
