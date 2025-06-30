use zed_extension_api as zed;

struct YapExtension;

impl zed::Extension for YapExtension {
    fn new() -> Self {
       Self
    }
}

zed::register_extension!(YapExtension);
