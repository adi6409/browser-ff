//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebNavigationInfo.idl
//


/// `interface nsIWebNavigationInfo : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebNavigationInfo interface exposes a way to get information
///  * on the capabilities of Gecko webnavigation objects.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebNavigationInfo {
    vtable: *const nsIWebNavigationInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebNavigationInfo.
unsafe impl XpCom for nsIWebNavigationInfo {
    const IID: nsIID = nsID(0x62a93afb, 0x93a1, 0x465c,
        [0x84, 0xc8, 0x04, 0x32, 0x26, 0x42, 0x29, 0xde]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebNavigationInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebNavigationInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebNavigationInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIWebNavigationInfo`.
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebNavigationInfoCoerce for nsIWebNavigationInfo {
    #[inline]
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self {
        v
    }
}

impl nsIWebNavigationInfo {
    /// Cast this `nsIWebNavigationInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebNavigationInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebNavigationInfo {
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
impl<T: nsISupportsCoerce> nsIWebNavigationInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebNavigationInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebNavigationInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebNavigationInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav); */
    pub IsTypeSupported: unsafe extern "system" fn (this: *const nsIWebNavigationInfo, aType: *const ::nsstring::nsACString, aWebNav: *const nsIWebNavigation, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebNavigationInfo {
    /// ```text
    /// /**
    ///    * Returned by isTypeSupported to indicate lack of support for a type.
    ///    * @note this is guaranteed not to change, so that boolean tests can be done
    ///    * on the return value if isTypeSupported to detect whether a type is
    ///    * supported at all.
    ///    */
    /// ```
    ///

    pub const UNSUPPORTED: i64 = 0;

    /// ```text
    /// /**
    ///    * Returned by isTypeSupported to indicate that a type is supported as an
    ///    * image.
    ///    */
    /// ```
    ///

    pub const IMAGE: i64 = 1;

    /// ```text
    /// /**
    ///    * Returned by isTypeSupported to indicate that a type is supported via an
    ///    * NPAPI ("Netscape 4 API") plug-in.  This is not the value returned for
    ///    * "XPCOM plug-ins".
    ///    */
    /// ```
    ///

    pub const PLUGIN: i64 = 2;

    /// ```text
    /// /**
    ///    * @note Other return types may be added here in the future as they become
    ///    * relevant.
    ///    */
    /// /**
    ///    * Returned by isTypeSupported to indicate that a type is supported via some
    ///    * other means.
    ///    */
    /// ```
    ///

    pub const OTHER: i64 = 32768;

    /// ```text
    /// /**
    ///    * Query whether aType is supported.
    ///    * @param aType the MIME type in question.
    ///    * @param aWebNav the nsIWebNavigation object for which the request
    ///    *        is being made.  This is allowed to be null.  If it is non-null,
    ///    *        the return value of this method may depend on the exact state of
    ///    *        aWebNav and the values set through nsIWebBrowserSetup; otherwise
    ///    *        the method will assume that the caller is interested in information
    ///    *        about nsIWebNavigation objects in their default state.
    ///    * @return an enum value indicating whether and how aType is supported.
    ///    * @note This method may rescan plugins to ensure that they're properly
    ///    *       registered for the types they support.
    ///    */
    /// ```
    ///

    /// `unsigned long isTypeSupported (in ACString aType, in nsIWebNavigation aWebNav);`
    #[inline]
    pub unsafe fn IsTypeSupported(&self, aType: *const ::nsstring::nsACString, aWebNav: *const nsIWebNavigation, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).IsTypeSupported)(self, aType, aWebNav, _retval)
    }


}


