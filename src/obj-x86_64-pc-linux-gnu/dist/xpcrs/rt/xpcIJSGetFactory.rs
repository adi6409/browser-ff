//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpcIJSGetFactory.idl
//


/// `interface xpcIJSGetFactory : nsISupports`
///

/// ```text
/// /**
///  * Every JS module exports a single NSGetFactory symbol which is converted into this
///  * functional interface type.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct xpcIJSGetFactory {
    vtable: *const xpcIJSGetFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for xpcIJSGetFactory.
unsafe impl XpCom for xpcIJSGetFactory {
    const IID: nsIID = nsID(0x3fe0c205, 0xd75b, 0x4cac,
        [0x93, 0x47, 0xd2, 0xb8, 0x55, 0x05, 0x01, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for xpcIJSGetFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from xpcIJSGetFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait xpcIJSGetFactoryCoerce {
    /// Cheaply cast a value of this type from a `xpcIJSGetFactory`.
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl xpcIJSGetFactoryCoerce for xpcIJSGetFactory {
    #[inline]
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self {
        v
    }
}

impl xpcIJSGetFactory {
    /// Cast this `xpcIJSGetFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: xpcIJSGetFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for xpcIJSGetFactory {
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
impl<T: nsISupportsCoerce> xpcIJSGetFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &xpcIJSGetFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every xpcIJSGetFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct xpcIJSGetFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIFactory get (in nsCIDRef aCID); */
    pub Get: unsafe extern "system" fn (this: *const xpcIJSGetFactory, aCID: *const nsCID, _retval: *mut*const nsIFactory) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl xpcIJSGetFactory {


    /// `nsIFactory get (in nsCIDRef aCID);`
    #[inline]
    pub unsafe fn Get(&self, aCID: *const nsCID, _retval: *mut*const nsIFactory) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, aCID, _retval)
    }


}


