//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULAppInfo.idl
//


/// `interface nsIXULAppInfo : nsIPlatformInfo`
///

/// ```text
/// /**
///  * A scriptable interface to the nsXULAppAPI structure. See nsXULAppAPI.h for
///  * a detailed description of each attribute.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXULAppInfo {
    vtable: *const nsIXULAppInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXULAppInfo.
unsafe impl XpCom for nsIXULAppInfo {
    const IID: nsIID = nsID(0xddea4f31, 0x3c5e, 0x4769,
        [0xac, 0x68, 0x21, 0xab, 0x4b, 0x3d, 0x78, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXULAppInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXULAppInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXULAppInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIXULAppInfo`.
    fn coerce_from(v: &nsIXULAppInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXULAppInfoCoerce for nsIXULAppInfo {
    #[inline]
    fn coerce_from(v: &nsIXULAppInfo) -> &Self {
        v
    }
}

impl nsIXULAppInfo {
    /// Cast this `nsIXULAppInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXULAppInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXULAppInfo {
    type Target = nsIPlatformInfo;
    #[inline]
    fn deref(&self) -> &nsIPlatformInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPlatformInfoCoerce> nsIXULAppInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXULAppInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXULAppInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXULAppInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPlatformInfoVTable,

    /* readonly attribute ACString vendor; */
    pub GetVendor: unsafe extern "system" fn (this: *const nsIXULAppInfo, aVendor: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIXULAppInfo, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString ID; */
    pub GetID: unsafe extern "system" fn (this: *const nsIXULAppInfo, aID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString version; */
    pub GetVersion: unsafe extern "system" fn (this: *const nsIXULAppInfo, aVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString appBuildID; */
    pub GetAppBuildID: unsafe extern "system" fn (this: *const nsIXULAppInfo, aAppBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString UAName; */
    pub GetUAName: unsafe extern "system" fn (this: *const nsIXULAppInfo, aUAName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString sourceURL; */
    pub GetSourceURL: unsafe extern "system" fn (this: *const nsIXULAppInfo, aSourceURL: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString updateURL; */
    pub GetUpdateURL: unsafe extern "system" fn (this: *const nsIXULAppInfo, aUpdateURL: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXULAppInfo {

    /// ```text
    /// /**
    ///    * @see XREAppData.vendor
    ///    * @returns an empty string if XREAppData.vendor is not set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString vendor;`
    #[inline]
    pub unsafe fn GetVendor(&self, aVendor: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetVendor)(self, aVendor)
    }


    /// ```text
    /// /**
    ///    * @see XREAppData.name
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * @see XREAppData.ID
    ///    * @returns an empty string if XREAppData.ID is not set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString ID;`
    #[inline]
    pub unsafe fn GetID(&self, aID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetID)(self, aID)
    }


    /// ```text
    /// /**
    ///    * The version of the XUL application. It is different than the
    ///    * version of the XULRunner platform. Be careful about which one you want.
    ///    *
    ///    * @see XREAppData.version
    ///    * @returns an empty string if XREAppData.version is not set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString version;`
    #[inline]
    pub unsafe fn GetVersion(&self, aVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetVersion)(self, aVersion)
    }


    /// ```text
    /// /**
    ///    * The build ID/date of the application. For xulrunner applications,
    ///    * this will be different than the build ID of the platform. Be careful
    ///    * about which one you want.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString appBuildID;`
    #[inline]
    pub unsafe fn GetAppBuildID(&self, aAppBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppBuildID)(self, aAppBuildID)
    }


    /// ```text
    /// /**
    ///    * @see XREAppData.UAName
    ///    * @returns an empty string if XREAppData.UAName is not set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString UAName;`
    #[inline]
    pub unsafe fn GetUAName(&self, aUAName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUAName)(self, aUAName)
    }


    /// ```text
    /// /**
    ///    * @see XREAppData.sourceURL
    ///    * @returns an empty string if XREAppData.sourceURL is not set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString sourceURL;`
    #[inline]
    pub unsafe fn GetSourceURL(&self, aSourceURL: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceURL)(self, aSourceURL)
    }


    /// ```text
    /// /**
    ///    * @see XREAppData.updateURL
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString updateURL;`
    #[inline]
    pub unsafe fn GetUpdateURL(&self, aUpdateURL: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateURL)(self, aUpdateURL)
    }


}


