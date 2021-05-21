//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/enterprisepolicies/nsIEnterprisePolicies.idl
//


/// `interface nsIEnterprisePolicies : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEnterprisePolicies {
    vtable: *const nsIEnterprisePoliciesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEnterprisePolicies.
unsafe impl XpCom for nsIEnterprisePolicies {
    const IID: nsIID = nsID(0x6a568972, 0xcc91, 0x4bf5,
        [0x96, 0x3e, 0x37, 0x68, 0xf3, 0x31, 0x9b, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEnterprisePolicies {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEnterprisePolicies.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEnterprisePoliciesCoerce {
    /// Cheaply cast a value of this type from a `nsIEnterprisePolicies`.
    fn coerce_from(v: &nsIEnterprisePolicies) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEnterprisePoliciesCoerce for nsIEnterprisePolicies {
    #[inline]
    fn coerce_from(v: &nsIEnterprisePolicies) -> &Self {
        v
    }
}

impl nsIEnterprisePolicies {
    /// Cast this `nsIEnterprisePolicies` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEnterprisePoliciesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEnterprisePolicies {
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
impl<T: nsISupportsCoerce> nsIEnterprisePoliciesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEnterprisePolicies) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEnterprisePolicies
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEnterprisePoliciesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute short status; */
    pub GetStatus: unsafe extern "system" fn (this: *const nsIEnterprisePolicies, aStatus: *mut i16) -> ::nserror::nsresult,

    /* bool isAllowed (in ACString feature); */
    pub IsAllowed: unsafe extern "system" fn (this: *const nsIEnterprisePolicies, feature: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* jsval getActivePolicies (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetActivePolicies: *const ::libc::c_void,

    /* jsval getSupportMenu (); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetSupportMenu: *const ::libc::c_void,

    /* jsval getExtensionPolicy (in ACString extensionID); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetExtensionPolicy: *const ::libc::c_void,

    /* jsval getExtensionSettings (in ACString extensionID); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetExtensionSettings: *const ::libc::c_void,

    /* bool mayInstallAddon (in jsval addon); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub MayInstallAddon: *const ::libc::c_void,

    /* bool allowedInstallSource (in nsIURI uri); */
    pub AllowedInstallSource: unsafe extern "system" fn (this: *const nsIEnterprisePolicies, uri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEnterprisePolicies {

    pub const UNINITIALIZED: i64 = -1;


    pub const INACTIVE: i64 = 0;


    pub const ACTIVE: i64 = 1;


    pub const FAILED: i64 = 2;


    /// `readonly attribute short status;`
    #[inline]
    pub unsafe fn GetStatus(&self, aStatus: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetStatus)(self, aStatus)
    }



    /// `bool isAllowed (in ACString feature);`
    #[inline]
    pub unsafe fn IsAllowed(&self, feature: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsAllowed)(self, feature, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the active policies that have been successfully parsed.
    ///    *
    ///    * @returns A JS object that contains the policies names and
    ///    *          their corresponding parameters.
    ///    */
    /// ```
    ///

    /// `jsval getActivePolicies ();`
    const _GetActivePolicies: () = ();

    /// ```text
    /// /**
    ///    * Get the contents of the support menu (if applicable)
    ///    *
    ///    * @returns A JS object that contains the url and label or null.
    ///    */
    /// ```
    ///

    /// `jsval getSupportMenu ();`
    const _GetSupportMenu: () = ();

    /// ```text
    /// /**
    ///    * Get the policy for a given extensionID (if available)
    ///    *
    ///    * @returns A JS object that contains the storage or null if unavailable.
    ///    */
    /// ```
    ///

    /// `jsval getExtensionPolicy (in ACString extensionID);`
    const _GetExtensionPolicy: () = ();

    /// ```text
    /// /**
    ///    * Retrieves the ExtensionSettings policy for the given extensionID.
    ///    *
    ///    * If there is no policy for the extension, it returns the global policy.
    ///    *
    ///    * If there is no global policy, it returns null.
    ///    *
    ///    * @returns A JS object that settings or null if unavailable.
    ///    */
    /// ```
    ///

    /// `jsval getExtensionSettings (in ACString extensionID);`
    const _GetExtensionSettings: () = ();

    /// ```text
    /// /**
    ///    * Uses the whitelist, blacklist and settings to determine if an extension
    ///    * may be installed.
    ///    *
    ///    * @returns A boolean - true of the extension may be installed.
    ///    */
    /// ```
    ///

    /// `bool mayInstallAddon (in jsval addon);`
    const _MayInstallAddon: () = ();

    /// ```text
    /// /**
    ///    * Uses install_sources to determine if an extension can be installed
    ///    * from the given URI.
    ///    *
    ///    * @returns A boolean - true of the extension may be installed.
    ///    */
    /// ```
    ///

    /// `bool allowedInstallSource (in nsIURI uri);`
    #[inline]
    pub unsafe fn AllowedInstallSource(&self, uri: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowedInstallSource)(self, uri, _retval)
    }


}


