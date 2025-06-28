use zed_extension_api as zed;

struct Yap;

impl zed::Extension for Yap {
    fn new() -> Self {
       Self
    }
}

zed::register_extension!(Yap);
