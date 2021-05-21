//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsIXREDirProvider.idl
//


/// `interface nsIXREDirProvider : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXREDirProvider {
    vtable: *const nsIXREDirProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXREDirProvider.
unsafe impl XpCom for nsIXREDirProvider {
    const IID: nsIID = nsID(0xf6ee3c0a, 0x5119, 0x47fc,
        [0xb1, 0xa7, 0xac, 0xe9, 0xe1, 0x11, 0x1f, 0xff]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXREDirProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXREDirProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXREDirProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIXREDirProvider`.
    fn coerce_from(v: &nsIXREDirProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXREDirProviderCoerce for nsIXREDirProvider {
    #[inline]
    fn coerce_from(v: &nsIXREDirProvider) -> &Self {
        v
    }
}

impl nsIXREDirProvider {
    /// Cast this `nsIXREDirProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXREDirProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXREDirProvider {
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
impl<T: nsISupportsCoerce> nsIXREDirProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXREDirProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXREDirProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXREDirProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setUserDataDirectory (in nsIFile aFile, in boolean aLocal); */
    pub SetUserDataDirectory: unsafe extern "system" fn (this: *const nsIXREDirProvider, aFile: *const nsIFile, aLocal: bool) -> ::nserror::nsresult,

    /* AString getInstallHash (); */
    pub GetInstallHash: unsafe extern "system" fn (this: *const nsIXREDirProvider, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXREDirProvider {

    /// ```text
    /// /**
    ///    * Only intended to be used from xpcshell tests. Allows setting the local
    ///    * and normal profile data directories. Calling this after something using
    ///    * them has started up will cause problems.
    ///    */
    /// ```
    ///

    /// `void setUserDataDirectory (in nsIFile aFile, in boolean aLocal);`
    #[inline]
    pub unsafe fn SetUserDataDirectory(&self, aFile: *const nsIFile, aLocal: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUserDataDirectory)(self, aFile, aLocal)
    }


    /// ```text
    /// /**
    ///    * Gets the hash for the current installation directory.
    ///    */
    /// ```
    ///

    /// `AString getInstallHash ();`
    #[inline]
    pub unsafe fn GetInstallHash(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetInstallHash)(self, _retval)
    }


}


