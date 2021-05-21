//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermission.idl
//


/// `interface nsIPermission : nsISupports`
///

/// ```text
/// /**
///  * This interface defines a "permission" object,
///  * used to specify allowed/blocked objects from
///  * user-specified sites (cookies, images etc).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPermission {
    vtable: *const nsIPermissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPermission.
unsafe impl XpCom for nsIPermission {
    const IID: nsIID = nsID(0xbb409a51, 0x2371, 0x4fea,
        [0x9d, 0xc9, 0xb7, 0x28, 0x6a, 0x45, 0x8b, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPermission {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPermission.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPermissionCoerce {
    /// Cheaply cast a value of this type from a `nsIPermission`.
    fn coerce_from(v: &nsIPermission) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPermissionCoerce for nsIPermission {
    #[inline]
    fn coerce_from(v: &nsIPermission) -> &Self {
        v
    }
}

impl nsIPermission {
    /// Cast this `nsIPermission` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPermissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPermission {
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
impl<T: nsISupportsCoerce> nsIPermissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPermission) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPermission
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPermissionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIPermission, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute ACString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIPermission, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t capability; */
    pub GetCapability: unsafe extern "system" fn (this: *const nsIPermission, aCapability: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t expireType; */
    pub GetExpireType: unsafe extern "system" fn (this: *const nsIPermission, aExpireType: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute int64_t expireTime; */
    pub GetExpireTime: unsafe extern "system" fn (this: *const nsIPermission, aExpireTime: *mut int64_t) -> ::nserror::nsresult,

    /* readonly attribute int64_t modificationTime; */
    pub GetModificationTime: unsafe extern "system" fn (this: *const nsIPermission, aModificationTime: *mut int64_t) -> ::nserror::nsresult,

    /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
    pub Matches: unsafe extern "system" fn (this: *const nsIPermission, principal: *const nsIPrincipal, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] boolean matchesPrincipalForPermission (in nsIPrincipal principal, in boolean exactHost); */
    pub MatchesPrincipalForPermission: unsafe extern "system" fn (this: *const nsIPermission, principal: *const nsIPrincipal, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
    pub MatchesURI: unsafe extern "system" fn (this: *const nsIPermission, uri: *const nsIURI, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPermission {

    /// ```text
    /// /**
    ///      * The principal for which this permission applies.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///      * a case-sensitive ASCII string, indicating the type of permission
    ///      * (e.g., "cookie", "image", etc).
    ///      * This string is specified by the consumer when adding a permission
    ///      * via nsIPermissionManager.
    ///      * @see nsIPermissionManager
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///      * The permission (see nsIPermissionManager.idl for allowed values)
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t capability;`
    #[inline]
    pub unsafe fn GetCapability(&self, aCapability: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCapability)(self, aCapability)
    }


    /// ```text
    /// /**
    ///      * The expiration type of the permission (session, time-based or none).
    ///      * Constants are EXPIRE_*, defined in nsIPermissionManager.
    ///      * @see nsIPermissionManager
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t expireType;`
    #[inline]
    pub unsafe fn GetExpireType(&self, aExpireType: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpireType)(self, aExpireType)
    }


    /// ```text
    /// /**
    ///      * The expiration time of the permission (milliseconds since Jan 1 1970
        ///      * 0:00:00).
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t expireTime;`
    #[inline]
    pub unsafe fn GetExpireTime(&self, aExpireTime: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpireTime)(self, aExpireTime)
    }


    /// ```text
    /// /**
    ///      * The last modification time of the permission (milliseconds since Jan 1 1970
        ///      * 0:00:00).
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t modificationTime;`
    #[inline]
    pub unsafe fn GetModificationTime(&self, aModificationTime: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetModificationTime)(self, aModificationTime)
    }


    /// ```text
    /// /**
    ///      * Test whether a principal would be affected by this permission.
    ///      *
    ///      * @param principal  the principal to test
    ///      * @param exactHost  If true, only the specific host will be matched,
    ///      *                   @see nsIPermissionManager::testExactPermission.
    ///      *                   If false, subdomains will also be searched,
    ///      *                   @see nsIPermissionManager::testPermission.
    ///      */
    /// ```
    ///

    /// `boolean matches (in nsIPrincipal principal, in boolean exactHost);`
    #[inline]
    pub unsafe fn Matches(&self, principal: *const nsIPrincipal, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Matches)(self, principal, exactHost, _retval)
    }


    /// ```text
    /// /**
    ///      * Similar to matches() but the principal's URI should be just an origin
    ///      * (no path, no queryString, etc).
    ///      */
    /// ```
    ///

    /// `[noscript] boolean matchesPrincipalForPermission (in nsIPrincipal principal, in boolean exactHost);`
    #[inline]
    pub unsafe fn MatchesPrincipalForPermission(&self, principal: *const nsIPrincipal, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).MatchesPrincipalForPermission)(self, principal, exactHost, _retval)
    }


    /// ```text
    /// /**
    ///      * Test whether a URI would be affected by this permission.
    ///      * NOTE: This performs matches with default origin attribute values.
    ///      *
    ///      * @param uri        the uri to test
    ///      * @param exactHost  If true, only the specific host will be matched,
    ///      *                   @see nsIPermissionManager::testExactPermission.
    ///      *                   If false, subdomains will also be searched,
    ///      *                   @see nsIPermissionManager::testPermission.
    ///      */
    /// ```
    ///

    /// `boolean matchesURI (in nsIURI uri, in boolean exactHost);`
    #[inline]
    pub unsafe fn MatchesURI(&self, uri: *const nsIURI, exactHost: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).MatchesURI)(self, uri, exactHost, _retval)
    }


}


