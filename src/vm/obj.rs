use super::Vm;

#[derive(Debug)]
#[repr(C)]
pub struct Obj {
    pub kind: ObjKind,
    pub next: *mut Obj,
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

#[inline]
pub fn free_object(obj: *mut Obj) {
    unsafe {
        let _ = Box::from_raw(
            match (*obj).kind {
                ObjKind::String => obj as *mut ObjString,
            }
        );
    }
}

pub fn allocate_string(vm: &mut Vm, string: String) -> *mut Obj{
    let obj_string = Box::into_raw(Box::new(ObjString {
        obj: Obj { kind: ObjKind::String, next: vm.objects }, string
    })) as *mut Obj;
    vm.objects = obj_string;
    obj_string
}

#[inline]
pub unsafe fn as_rust_string(obj: *const Obj) -> *const String {
    &(*(obj as *mut ObjString)).string
}

#[macro_export]
macro_rules! as_rs_string {
    ($obj:expr) => {
        &*crate::vm::obj::as_rust_string($obj)
    };
}
