use super::{Vm, chunk::Chunk};

#[derive(Debug)]
#[repr(C)]
pub struct Obj {
    pub kind: ObjKind,
    pub next: *mut Obj,
}

#[derive(Debug)]
pub enum ObjKind {
    String,
    Fn,
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjString {
    pub obj: Obj,
    pub string: String,
}

#[repr(C)]
pub struct ObjFn {
    pub obj: Obj,
    pub chunk: Chunk,
    pub name: *mut Obj, // ObjString name
}

#[inline]
pub fn free_object(obj: *mut Obj) {
    unsafe {
        match (*obj).kind {
            ObjKind::String => { let _ = Box::from_raw(obj as *mut ObjString); },
            ObjKind::Fn => { 
                // todo: make sure ObjFn name is being freed by GC
                let _ = Box::from_raw(obj as *mut ObjFn); 
            },
        }
    }
}

pub fn allocate_string(vm: &mut Vm, string: String) -> *mut Obj{
    let obj_string = Box::into_raw(Box::new(ObjString {
        obj: Obj { kind: ObjKind::String, next: std::ptr::null_mut() }, string
    })) as *mut Obj;
    add_obj_to_linked_list(vm, obj_string)
}

pub fn store_function(vm: &mut Vm, function: ObjFn) -> *mut Obj{
    let function = Box::into_raw(Box::new(function)) as *mut Obj;
    add_obj_to_linked_list(vm, function)
}

fn add_obj_to_linked_list(vm: &mut Vm, obj: *mut Obj) -> *mut Obj {
    unsafe {
        (*obj).next = vm.objects;
    }
    vm.objects = obj;
    obj
}

#[inline]
pub unsafe fn as_rust_string(obj: *mut Obj) -> *mut String {
    &mut (*(obj as *mut ObjString)).string
}

#[macro_export]
macro_rules! as_rs_string {
    ($obj:expr) => {
        &*crate::vm::obj::as_rust_string($obj)
    };
}
