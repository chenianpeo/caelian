use crate::packages_list::packages_list_obtain::packages_list_obtain;

#[unsafe(no_mangle)]
pub extern "C" fn ffi_packages_list_obtain() {
    packages_list_obtain();
}