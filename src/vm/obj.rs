pub struct Obj {
    pub kind: ObjKind,
}

pub enum ObjKind {
    String,
}

pub struct ObjString {
    pub obj: Obj,
    pub string: String,
}

pub fn allocate_string(string: String) -> *const Obj{
    Box::into_raw(Box::new(ObjString {
        obj: Obj { kind: ObjKind::String },
        string
    })) as *const Obj
}

pub unsafe fn as_rust_string(obj: *const Obj) -> *const String {
    &(*(obj as *const ObjString)).string
}

#[macro_export]
macro_rules! as_rs_string {
    ($obj:expr) => {
        &*crate::vm::obj::as_rust_string($obj)
    };
}
