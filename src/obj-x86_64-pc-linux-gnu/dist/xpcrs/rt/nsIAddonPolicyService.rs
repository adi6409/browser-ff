//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIAddonPolicyService.idl
//


/// `interface nsIAddonPolicyService : nsISupports`
///

/// ```text
/// /**
///  * This interface allows the security manager to query custom per-addon security
///  * policy.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAddonPolicyService {
    vtable: *const nsIAddonPolicyServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAddonPolicyService.
unsafe impl XpCom for nsIAddonPolicyService {
    const IID: nsIID = nsID(0x8a034ef9, 0x9d14, 0x4c5d,
        [0x83, 0x19, 0x06, 0xc1, 0xab, 0x57, 0x4b, 0xaa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAddonPolicyService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAddonPolicyService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAddonPolicyServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIAddonPolicyService`.
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAddonPolicyServiceCoerce for nsIAddonPolicyService {
    #[inline]
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self {
        v
    }
}

impl nsIAddonPolicyService {
    /// Cast this `nsIAddonPolicyService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAddonPolicyServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAddonPolicyService {
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
impl<T: nsISupportsCoerce> nsIAddonPolicyServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAddonPolicyService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAddonPolicyService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAddonPolicyServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString defaultCSP; */
    pub GetDefaultCSP: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aDefaultCSP: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getBaseCSP (in AString aAddonId); */
    pub GetBaseCSP: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getExtensionPageCSP (in AString aAddonId); */
    pub GetExtensionPageCSP: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
    pub GetGeneratedBackgroundPageUrl: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
    pub AddonHasPermission: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsAString, aPerm: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
    pub AddonMayLoadURI: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsAString, aURI: *const nsIURI, aExplicit: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getExtensionName (in AString aAddonId); */
    pub GetExtensionName: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
    pub ExtensionURILoadableByAnyone: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString extensionURIToAddonId (in nsIURI aURI); */
    pub ExtensionURIToAddonId: unsafe extern "system" fn (this: *const nsIAddonPolicyService, aURI: *const nsIURI, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAddonPolicyService {

    /// ```text
    /// /**
    ///    * Returns the default content security policy which applies to extension
    ///    * documents which do not specify any custom policies.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString defaultCSP;`
    #[inline]
    pub unsafe fn GetDefaultCSP(&self, aDefaultCSP: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultCSP)(self, aDefaultCSP)
    }


    /// ```text
    /// /**
    ///    * Returns the base content security policy which applies to all extension resources.
    ///    */
    /// ```
    ///

    /// `AString getBaseCSP (in AString aAddonId);`
    #[inline]
    pub unsafe fn GetBaseCSP(&self, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseCSP)(self, aAddonId, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the content security policy which applies to documents belonging
    ///    * to the extension with the given ID. This may be either a custom policy,
    ///    * if one was supplied, or the default policy if one was not.
    ///    */
    /// ```
    ///

    /// `AString getExtensionPageCSP (in AString aAddonId);`
    #[inline]
    pub unsafe fn GetExtensionPageCSP(&self, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetExtensionPageCSP)(self, aAddonId, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the generated background page as a data-URI, if any. If the addon
    ///    * does not have an auto-generated background page, an empty string is
    ///    * returned.
    ///    */
    /// ```
    ///

    /// `ACString getGeneratedBackgroundPageUrl (in ACString aAddonId);`
    #[inline]
    pub unsafe fn GetGeneratedBackgroundPageUrl(&self, aAddonId: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetGeneratedBackgroundPageUrl)(self, aAddonId, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if the addon was granted the |aPerm| API permission.
    ///    */
    /// ```
    ///

    /// `boolean addonHasPermission (in AString aAddonId, in AString aPerm);`
    #[inline]
    pub unsafe fn AddonHasPermission(&self, aAddonId: *const ::nsstring::nsAString, aPerm: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AddonHasPermission)(self, aAddonId, aPerm, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if unprivileged code associated with the given addon may load
    ///    * data from |aURI|.  If |aExplicit| is true, the <all_urls> permission and
    ///    * permissive host globs are ignored when checking for a match.
    ///    */
    /// ```
    ///

    /// `boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit);`
    #[inline]
    pub unsafe fn AddonMayLoadURI(&self, aAddonId: *const ::nsstring::nsAString, aURI: *const nsIURI, aExplicit: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AddonMayLoadURI)(self, aAddonId, aURI, aExplicit, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the name of the WebExtension with the given ID, or the ID string
    ///    * if no matching add-on can be found.
    ///    */
    /// ```
    ///

    /// `AString getExtensionName (in AString aAddonId);`
    #[inline]
    pub unsafe fn GetExtensionName(&self, aAddonId: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetExtensionName)(self, aAddonId, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if a given extension:// URI is web-accessible.
    ///    */
    /// ```
    ///

    /// `boolean extensionURILoadableByAnyone (in nsIURI aURI);`
    #[inline]
    pub unsafe fn ExtensionURILoadableByAnyone(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExtensionURILoadableByAnyone)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///    * Maps an extension URI to the ID of the addon it belongs to.
    ///    */
    /// ```
    ///

    /// `AString extensionURIToAddonId (in nsIURI aURI);`
    #[inline]
    pub unsafe fn ExtensionURIToAddonId(&self, aURI: *const nsIURI, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ExtensionURIToAddonId)(self, aURI, _retval)
    }


}


/// `interface nsIAddonContentPolicy : nsISupports`
///

/// ```text
/// /**
///  * This interface exposes functionality related to add-on content policy
///  * enforcement.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAddonContentPolicy {
    vtable: *const nsIAddonContentPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAddonContentPolicy.
unsafe impl XpCom for nsIAddonContentPolicy {
    const IID: nsIID = nsID(0x7a4fe60b, 0x9131, 0x45f5,
        [0x83, 0xf3, 0xdc, 0x63, 0xb5, 0xd7, 0x1a, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAddonContentPolicy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAddonContentPolicy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAddonContentPolicyCoerce {
    /// Cheaply cast a value of this type from a `nsIAddonContentPolicy`.
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAddonContentPolicyCoerce for nsIAddonContentPolicy {
    #[inline]
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self {
        v
    }
}

impl nsIAddonContentPolicy {
    /// Cast this `nsIAddonContentPolicy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAddonContentPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAddonContentPolicy {
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
impl<T: nsISupportsCoerce> nsIAddonContentPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAddonContentPolicy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAddonContentPolicy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAddonContentPolicyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString validateAddonCSP (in AString aPolicyString, in unsigned long aPermittedPolicy); */
    pub ValidateAddonCSP: unsafe extern "system" fn (this: *const nsIAddonContentPolicy, aPolicyString: *const ::nsstring::nsAString, aPermittedPolicy: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAddonContentPolicy {

    pub const CSP_ALLOW_ANY: i64 = 65535;


    pub const CSP_ALLOW_LOCALHOST: i64 = 1;


    pub const CSP_ALLOW_EVAL: i64 = 2;


    pub const CSP_ALLOW_REMOTE: i64 = 4;

    /// ```text
    /// /**
    ///    * Checks a custom content security policy string, to ensure that it meets
    ///    * minimum security requirements. Returns null for valid policies, or a
    ///    * string describing the error for invalid policies.
    ///    */
    /// ```
    ///

    /// `AString validateAddonCSP (in AString aPolicyString, in unsigned long aPermittedPolicy);`
    #[inline]
    pub unsafe fn ValidateAddonCSP(&self, aPolicyString: *const ::nsstring::nsAString, aPermittedPolicy: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ValidateAddonCSP)(self, aPolicyString, aPermittedPolicy, _retval)
    }


}


