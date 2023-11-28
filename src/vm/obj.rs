use super::Vm;

#[derive(Debug)]
#[repr(C)]
pub struct Obj {
    pub kind: ObjKind,
    pub next: *const Obj,
}

#[derive(Debug)]
pub enum ObjKind {
    String,
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub string: String,
}

pub fn allocate_string(vm: &mut Vm, string: String) -> *const Obj{
    let obj_string = Box::into_raw(Box::new(ObjString {
        obj: Obj { kind: ObjKind::String, next: vm.objects }, string
    })) as *const Obj;
    vm.objects = obj_string;
    obj_string
}

#[inline]
pub unsafe fn as_rust_string(obj: *const Obj) -> *const String {
    &(*(obj as *const ObjString)).string
}

#[macro_export]
macro_rules! as_rs_string {
    ($obj:expr) => {
        &*crate::vm::obj::as_rust_string($obj)
    };
}
