//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsINSSVersion.idl
//


/// `interface nsINSSVersion : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINSSVersion {
    vtable: *const nsINSSVersionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINSSVersion.
unsafe impl XpCom for nsINSSVersion {
    const IID: nsIID = nsID(0xa8a53a2b, 0x75cc, 0x4c68,
        [0xa9, 0xbb, 0x97, 0x91, 0xdb, 0xdd, 0xaa, 0x00]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINSSVersion {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINSSVersion.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINSSVersionCoerce {
    /// Cheaply cast a value of this type from a `nsINSSVersion`.
    fn coerce_from(v: &nsINSSVersion) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINSSVersionCoerce for nsINSSVersion {
    #[inline]
    fn coerce_from(v: &nsINSSVersion) -> &Self {
        v
    }
}

impl nsINSSVersion {
    /// Cast this `nsINSSVersion` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINSSVersionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINSSVersion {
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
impl<T: nsISupportsCoerce> nsINSSVersionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINSSVersion) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINSSVersion
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINSSVersionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute AString NSPR_MinVersion; */
    pub GetNSPR_MinVersion: unsafe extern "system" fn (this: *const nsINSSVersion, aNSPR_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSS_MinVersion; */
    pub GetNSS_MinVersion: unsafe extern "system" fn (this: *const nsINSSVersion, aNSS_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSUTIL_MinVersion; */
    pub GetNSSUTIL_MinVersion: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSUTIL_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSSSL_MinVersion; */
    pub GetNSSSSL_MinVersion: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSSSL_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSSMIME_MinVersion; */
    pub GetNSSSMIME_MinVersion: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSSMIME_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSPR_Version; */
    pub GetNSPR_Version: unsafe extern "system" fn (this: *const nsINSSVersion, aNSPR_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSS_Version; */
    pub GetNSS_Version: unsafe extern "system" fn (this: *const nsINSSVersion, aNSS_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSUTIL_Version; */
    pub GetNSSUTIL_Version: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSUTIL_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSSSL_Version; */
    pub GetNSSSSL_Version: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSSSL_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AString NSSSMIME_Version; */
    pub GetNSSSMIME_Version: unsafe extern "system" fn (this: *const nsINSSVersion, aNSSSMIME_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINSSVersion {


    /// `[must_use] readonly attribute AString NSPR_MinVersion;`
    #[inline]
    pub unsafe fn GetNSPR_MinVersion(&self, aNSPR_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSPR_MinVersion)(self, aNSPR_MinVersion)
    }



    /// `[must_use] readonly attribute AString NSS_MinVersion;`
    #[inline]
    pub unsafe fn GetNSS_MinVersion(&self, aNSS_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSS_MinVersion)(self, aNSS_MinVersion)
    }



    /// `[must_use] readonly attribute AString NSSUTIL_MinVersion;`
    #[inline]
    pub unsafe fn GetNSSUTIL_MinVersion(&self, aNSSUTIL_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSUTIL_MinVersion)(self, aNSSUTIL_MinVersion)
    }



    /// `[must_use] readonly attribute AString NSSSSL_MinVersion;`
    #[inline]
    pub unsafe fn GetNSSSSL_MinVersion(&self, aNSSSSL_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSSSL_MinVersion)(self, aNSSSSL_MinVersion)
    }



    /// `[must_use] readonly attribute AString NSSSMIME_MinVersion;`
    #[inline]
    pub unsafe fn GetNSSSMIME_MinVersion(&self, aNSSSMIME_MinVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSSMIME_MinVersion)(self, aNSSSMIME_MinVersion)
    }



    /// `[must_use] readonly attribute AString NSPR_Version;`
    #[inline]
    pub unsafe fn GetNSPR_Version(&self, aNSPR_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSPR_Version)(self, aNSPR_Version)
    }



    /// `[must_use] readonly attribute AString NSS_Version;`
    #[inline]
    pub unsafe fn GetNSS_Version(&self, aNSS_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSS_Version)(self, aNSS_Version)
    }



    /// `[must_use] readonly attribute AString NSSUTIL_Version;`
    #[inline]
    pub unsafe fn GetNSSUTIL_Version(&self, aNSSUTIL_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSUTIL_Version)(self, aNSSUTIL_Version)
    }



    /// `[must_use] readonly attribute AString NSSSSL_Version;`
    #[inline]
    pub unsafe fn GetNSSSSL_Version(&self, aNSSSSL_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSSSL_Version)(self, aNSSSSL_Version)
    }



    /// `[must_use] readonly attribute AString NSSSMIME_Version;`
    #[inline]
    pub unsafe fn GetNSSSMIME_Version(&self, aNSSSMIME_Version: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNSSSMIME_Version)(self, aNSSSMIME_Version)
    }


}


