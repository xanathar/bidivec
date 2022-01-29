#[macro_export]
macro_rules! run_test_on_types {
    ($test:ident on all) => {
        run_test_on_types!($test on
            structs: Struct,
            clonable: CloneStruct,
            copyable: CopyStruct,
            defaultable: DefaultStruct,
            enums: Enum,
            option: Option<i32>,
            plain_int: i32,
            string: String,
            vec: Vec<i32>
        );
    };
    ($test:ident on clonables) => {
        run_test_on_types!($test on
            clonable: CloneStruct,
            copyable: CopyStruct,
            option: Option<i32>,
            plain_int: i32,
            string: String,
            vec: Vec<i32>
        );
    };
    ($test:ident on copyables) => {
        run_test_on_types!($test on
            copyable: CopyStruct,
            option: Option<i32>,
            plain_int: i32
        );
    };
    ($test:ident on $($name:ident: $type:ty),*) => {
        mod $test {
            use super::*;
                $(#[test]
                fn $name() {
                    $test::<$type>();
                })*
        }
    }
}

pub(super) trait Testable {
    fn new(id: i32) -> Self;
    fn id(&self) -> i32;
}

pub(super) struct Struct(pub i32);
impl Testable for Struct {
    fn new(id: i32) -> Self {
        Self(id)
    }
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub(super) struct CopyStruct(pub i32);
impl Testable for CopyStruct {
    fn new(id: i32) -> Self {
        Self(id)
    }
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(Default)]
pub(super) struct DefaultStruct(pub i32);
impl Testable for DefaultStruct {
    fn new(id: i32) -> Self {
        Self(id)
    }
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(Clone)]
pub(super) struct CloneStruct(pub i32);
impl Testable for CloneStruct {
    fn new(id: i32) -> Self {
        Self(id)
    }
    fn id(&self) -> i32 {
        self.0
    }
}

pub(super) enum Enum {
    Zero,
    Other(i32),
}
impl Testable for Enum {
    fn new(id: i32) -> Self {
        if id == 0 {
            Enum::Zero
        } else {
            Enum::Other(id)
        }
    }
    fn id(&self) -> i32 {
        match self {
            Enum::Zero => 0,
            Enum::Other(id) => *id,
        }
    }
}

impl Testable for Option<i32> {
    fn new(id: i32) -> Self {
        if id == 0 {
            None
        } else {
            Some(id)
        }
    }
    fn id(&self) -> i32 {
        self.unwrap_or_default()
    }
}

impl Testable for i32 {
    fn new(id: i32) -> Self {
        id
    }
    fn id(&self) -> i32 {
        *self
    }
}

impl Testable for String {
    fn new(id: i32) -> Self {
        id.to_string()
    }
    fn id(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}

impl Testable for Vec<i32> {
    fn new(id: i32) -> Self {
        vec![id]
    }
    fn id(&self) -> i32 {
        self[0]
    }
}
