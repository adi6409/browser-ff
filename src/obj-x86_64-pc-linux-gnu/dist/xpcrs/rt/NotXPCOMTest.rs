//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/tests/NotXPCOMTest.idl
//


/// `interface nsIScriptableOK : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableOK {
    vtable: *const nsIScriptableOKVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableOK.
unsafe impl XpCom for nsIScriptableOK {
    const IID: nsIID = nsID(0x93142a4f, 0xe4cf, 0x424a,
        [0xb8, 0x33, 0xe6, 0x38, 0xf8, 0x7d, 0x26, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableOK {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableOK.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableOKCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableOK`.
    fn coerce_from(v: &nsIScriptableOK) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableOKCoerce for nsIScriptableOK {
    #[inline]
    fn coerce_from(v: &nsIScriptableOK) -> &Self {
        v
    }
}

impl nsIScriptableOK {
    /// Cast this `nsIScriptableOK` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableOKCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableOK {
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
impl<T: nsISupportsCoerce> nsIScriptableOKCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableOK) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableOK
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableOKVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void method1 (); */
    pub Method1: unsafe extern "system" fn (this: *const nsIScriptableOK) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableOK {


    /// `void method1 ();`
    #[inline]
    pub unsafe fn Method1(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Method1)(self, )
    }


}


/// `interface nsIScriptableWithNotXPCOM : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableWithNotXPCOM {
    vtable: *const nsIScriptableWithNotXPCOMVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableWithNotXPCOM.
unsafe impl XpCom for nsIScriptableWithNotXPCOM {
    const IID: nsIID = nsID(0x237d01a3, 0x771e, 0x4c6e,
        [0xad, 0xf9, 0xc9, 0x7f, 0x9a, 0xab, 0x29, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableWithNotXPCOM {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableWithNotXPCOM.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableWithNotXPCOMCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableWithNotXPCOM`.
    fn coerce_from(v: &nsIScriptableWithNotXPCOM) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableWithNotXPCOMCoerce for nsIScriptableWithNotXPCOM {
    #[inline]
    fn coerce_from(v: &nsIScriptableWithNotXPCOM) -> &Self {
        v
    }
}

impl nsIScriptableWithNotXPCOM {
    /// Cast this `nsIScriptableWithNotXPCOM` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableWithNotXPCOMCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableWithNotXPCOM {
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
impl<T: nsISupportsCoerce> nsIScriptableWithNotXPCOMCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableWithNotXPCOM) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableWithNotXPCOM
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableWithNotXPCOMVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [notxpcom] void method2 (); */
    pub Method2: unsafe extern "system" fn (this: *const nsIScriptableWithNotXPCOM) -> libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableWithNotXPCOM {


    /// `[notxpcom] void method2 ();`
    #[inline]
    pub unsafe fn Method2(&self, ) -> libc::c_void {
        ((*self.vtable).Method2)(self, )
    }


}


