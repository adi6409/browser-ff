//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11Module.idl
//


/// `interface nsIPKCS11Module : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPKCS11Module {
    vtable: *const nsIPKCS11ModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPKCS11Module.
unsafe impl XpCom for nsIPKCS11Module {
    const IID: nsIID = nsID(0x8a44bdf9, 0xd1a5, 0x4734,
        [0xbd, 0x5a, 0x34, 0xed, 0x7f, 0xe5, 0x64, 0xc2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPKCS11Module {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPKCS11Module.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPKCS11ModuleCoerce {
    /// Cheaply cast a value of this type from a `nsIPKCS11Module`.
    fn coerce_from(v: &nsIPKCS11Module) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPKCS11ModuleCoerce for nsIPKCS11Module {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Module) -> &Self {
        v
    }
}

impl nsIPKCS11Module {
    /// Cast this `nsIPKCS11Module` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPKCS11ModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPKCS11Module {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIPKCS11ModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11Module) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPKCS11Module
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPKCS11ModuleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPKCS11Module, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String libName; */
    pub GetLibName: unsafe extern "system" fn (this: *const nsIPKCS11Module, aLibName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] nsISimpleEnumerator listSlots (); */
    pub ListSlots: unsafe extern "system" fn (this: *const nsIPKCS11Module, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPKCS11Module {


    /// `[must_use] readonly attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `[must_use] readonly attribute AUTF8String libName;`
    #[inline]
    pub unsafe fn GetLibName(&self, aLibName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLibName)(self, aLibName)
    }



    /// `[must_use] nsISimpleEnumerator listSlots ();`
    #[inline]
    pub unsafe fn ListSlots(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).ListSlots)(self, _retval)
    }


}


