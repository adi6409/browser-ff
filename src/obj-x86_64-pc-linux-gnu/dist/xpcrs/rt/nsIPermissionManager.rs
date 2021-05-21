//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermissionManager.idl
//


/// `interface nsIPermissionManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPermissionManager {
    vtable: *const nsIPermissionManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPermissionManager.
unsafe impl XpCom for nsIPermissionManager {
    const IID: nsIID = nsID(0x4dcb3851, 0xeba2, 0x4e42,
        [0xb2, 0x36, 0x82, 0xd2, 0x59, 0x6f, 0xca, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPermissionManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPermissionManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPermissionManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIPermissionManager`.
    fn coerce_from(v: &nsIPermissionManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPermissionManagerCoerce for nsIPermissionManager {
    #[inline]
    fn coerce_from(v: &nsIPermissionManager) -> &Self {
        v
    }
}

impl nsIPermissionManager {
    /// Cast this `nsIPermissionManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPermissionManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPermissionManager {
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
impl<T: nsISupportsCoerce> nsIPermissionManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPermissionManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPermissionManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPermissionManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Array<nsIPermission> getAllForPrincipal (in nsIPrincipal principal); */
    pub GetAllForPrincipal: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult,

    /* Array<nsIPermission> getAllWithTypePrefix (in ACString prefix); */
    pub GetAllWithTypePrefix: unsafe extern "system" fn (this: *const nsIPermissionManager, prefix: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult,

    /* Array<nsIPermission> getAllByTypeSince (in ACString type, in int64_t since); */
    pub GetAllByTypeSince: unsafe extern "system" fn (this: *const nsIPermissionManager, type_: *const ::nsstring::nsACString, since: int64_t, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult,

    /* void addFromPrincipal (in nsIPrincipal principal, in ACString type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
    pub AddFromPrincipal: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> ::nserror::nsresult,

    /* void removeFromPrincipal (in nsIPrincipal principal, in ACString type); */
    pub RemoveFromPrincipal: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void removePermission (in nsIPermission perm); */
    pub RemovePermission: unsafe extern "system" fn (this: *const nsIPermissionManager, perm: *const nsIPermission) -> ::nserror::nsresult,

    /* void removeAll (); */
    pub RemoveAll: unsafe extern "system" fn (this: *const nsIPermissionManager) -> ::nserror::nsresult,

    /* void removeAllSince (in int64_t since); */
    pub RemoveAllSince: unsafe extern "system" fn (this: *const nsIPermissionManager, since: int64_t) -> ::nserror::nsresult,

    /* void removeByType (in ACString type); */
    pub RemoveByType: unsafe extern "system" fn (this: *const nsIPermissionManager, type_: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void removeByTypeSince (in ACString type, in int64_t since); */
    pub RemoveByTypeSince: unsafe extern "system" fn (this: *const nsIPermissionManager, type_: *const ::nsstring::nsACString, since: int64_t) -> ::nserror::nsresult,

    /* uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
    pub TestPermissionFromPrincipal: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
    pub TestExactPermissionFromPrincipal: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* uint32_t testExactPermanentPermission (in nsIPrincipal principal, in ACString type); */
    pub TestExactPermanentPermission: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* nsIPermission getPermissionObject (in nsIPrincipal principal, in ACString type, in boolean exactHost); */
    pub GetPermissionObject: unsafe extern "system" fn (this: *const nsIPermissionManager, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, exactHost: bool, _retval: *mut*const nsIPermission) -> ::nserror::nsresult,

    /* readonly attribute Array<nsIPermission> all; */
    pub GetAll: unsafe extern "system" fn (this: *const nsIPermissionManager, aAll: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult,

    /* void removePermissionsWithAttributes (in AString patternAsJSON); */
    pub RemovePermissionsWithAttributes: unsafe extern "system" fn (this: *const nsIPermissionManager, patternAsJSON: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal); */
    pub BroadcastPermissionsForPrincipalToAllContentProcesses: unsafe extern "system" fn (this: *const nsIPermissionManager, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPermissionManager {
    /// ```text
    /// /**
    ///    * Predefined return values for the testPermission method and for
    ///    * the permission param of the add method
    ///    * NOTE: UNKNOWN_ACTION (0) is reserved to represent the
    ///    * default permission when no entry is found for a host, and
    ///    * should not be used by consumers to indicate otherwise.
    ///    */
    /// ```
    ///

    pub const UNKNOWN_ACTION: i64 = 0;


    pub const ALLOW_ACTION: i64 = 1;


    pub const DENY_ACTION: i64 = 2;


    pub const PROMPT_ACTION: i64 = 3;

    /// ```text
    /// /**
    ///    * Predefined expiration types for permissions.  Permissions can be permanent
    ///    * (never expire), expire at the end of the session, or expire at a specified
    ///    * time. Permissions that expire at the end of a session may also have a
    ///    * specified expiration time.
    ///    *
    ///    * EXPIRE_POLICY is a special expiration status. It is set when the permission
    ///    * is set by reading an enterprise policy. These permissions cannot be overridden.
    ///    */
    /// ```
    ///

    pub const EXPIRE_NEVER: i64 = 0;


    pub const EXPIRE_SESSION: i64 = 1;


    pub const EXPIRE_TIME: i64 = 2;


    pub const EXPIRE_POLICY: i64 = 3;

    /// ```text
    /// /**
    ///    * Get all custom permissions for a given nsIPrincipal. This will return an
    ///    * enumerator of all permissions which are not set to default and which
    ///    * belong to the matching principal of the given nsIPrincipal.
    ///    *
    ///    * @param principal  the URI to get all permissions for
    ///    */
    /// ```
    ///

    /// `Array<nsIPermission> getAllForPrincipal (in nsIPrincipal principal);`
    #[inline]
    pub unsafe fn GetAllForPrincipal(&self, principal: *const nsIPrincipal, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllForPrincipal)(self, principal, _retval)
    }


    /// ```text
    /// /**
    ///    * Get all custom permissions of a specific type, specified with a prefix
    ///    * string.  This will return an array of all permissions which are not set to
    ///    * default.  Also the passed type argument is either equal to or a prefix of
    ///    * the type of the returned permissions.
    ///    *
    ///    * @param prefix  the type prefix string
    ///    */
    /// ```
    ///

    /// `Array<nsIPermission> getAllWithTypePrefix (in ACString prefix);`
    #[inline]
    pub unsafe fn GetAllWithTypePrefix(&self, prefix: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllWithTypePrefix)(self, prefix, _retval)
    }


    /// ```text
    /// /**
    ///    * Get all custom permissions of a specific type and that were modified after
    ///    * the specified date. This will return an array of all permissions which are
    ///    * not set to default.
    ///    *
    ///    * @param type    a case-sensitive ASCII string, identifying the permission.
    ///    * @param since   a unix timestamp representing the number of milliseconds from
    ///    *                Jan 1, 1970 00:00:00 UTC.
    ///    */
    /// ```
    ///

    /// `Array<nsIPermission> getAllByTypeSince (in ACString type, in int64_t since);`
    #[inline]
    pub unsafe fn GetAllByTypeSince(&self, type_: *const ::nsstring::nsACString, since: int64_t, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllByTypeSince)(self, type_, since, _retval)
    }


    /// ```text
    /// /**
    ///    * Add permission information for a given principal.
    ///    * It is internally calling the other add() method using the nsIURI from the
    ///    * principal.
    ///    * Passing a system principal will be a no-op because they will always be
    ///    * granted permissions.
    ///    */
    /// ```
    ///

    /// `void addFromPrincipal (in nsIPrincipal principal, in ACString type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime);`
    #[inline]
    pub unsafe fn AddFromPrincipal(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, permission: uint32_t, expireType: uint32_t, expireTime: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).AddFromPrincipal)(self, principal, type_, permission, expireType, expireTime)
    }


    /// ```text
    /// /**
    ///    * Remove permission information for a given principal.
    ///    * This is internally calling remove() with the host from the principal's URI.
    ///    * Passing system principal will be a no-op because we never add them to the
    ///    * database.
    ///    */
    /// ```
    ///

    /// `void removeFromPrincipal (in nsIPrincipal principal, in ACString type);`
    #[inline]
    pub unsafe fn RemoveFromPrincipal(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveFromPrincipal)(self, principal, type_)
    }


    /// ```text
    /// /**
    ///    * Remove the given permission from the permission manager.
    ///    *
    ///    * @param perm   a permission obtained from the permission manager.
    ///    */
    /// ```
    ///

    /// `void removePermission (in nsIPermission perm);`
    #[inline]
    pub unsafe fn RemovePermission(&self, perm: *const nsIPermission) -> ::nserror::nsresult {
        ((*self.vtable).RemovePermission)(self, perm)
    }


    /// ```text
    /// /**
    ///    * Clear permission information for all websites.
    ///    */
    /// ```
    ///

    /// `void removeAll ();`
    #[inline]
    pub unsafe fn RemoveAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAll)(self, )
    }


    /// ```text
    /// /**
    ///    * Clear all permission information added since the specified time.
    ///    */
    /// ```
    ///

    /// `void removeAllSince (in int64_t since);`
    #[inline]
    pub unsafe fn RemoveAllSince(&self, since: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllSince)(self, since)
    }


    /// ```text
    /// /**
    ///    * Clear all permissions of the passed type.
    ///    */
    /// ```
    ///

    /// `void removeByType (in ACString type);`
    #[inline]
    pub unsafe fn RemoveByType(&self, type_: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveByType)(self, type_)
    }


    /// ```text
    /// /**
    ///    * Clear all permissions of the passed type added since the specified time.
    ///    * @param type    a case-sensitive ASCII string, identifying the permission.
    ///    * @param since   a unix timestamp representing the number of milliseconds from
    ///    *                Jan 1, 1970 00:00:00 UTC.
    ///    */
    /// ```
    ///

    /// `void removeByTypeSince (in ACString type, in int64_t since);`
    #[inline]
    pub unsafe fn RemoveByTypeSince(&self, type_: *const ::nsstring::nsACString, since: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).RemoveByTypeSince)(self, type_, since)
    }


    /// ```text
    /// /**
    ///    * Test whether the principal has the permission to perform a given action.
    ///    * System principals will always have permissions granted.
    ///    * This function will perform a pref lookup to permissions.default.<type>
    ///    * if the specific permission type is part of the whitelist for that functionality.
    ///    */
    /// ```
    ///

    /// `uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in ACString type);`
    #[inline]
    pub unsafe fn TestPermissionFromPrincipal(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).TestPermissionFromPrincipal)(self, principal, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * See testExactPermission() above.
    ///    * System principals will always have permissions granted.
    ///    * This function will perform a pref lookup to permissions.default.<type>
    ///    * if the specific permission type is part of the whitelist for that functionality.
    ///    */
    /// ```
    ///

    /// `uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in ACString type);`
    #[inline]
    pub unsafe fn TestExactPermissionFromPrincipal(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).TestExactPermissionFromPrincipal)(self, principal, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Test whether a website has permission to perform the given action
    ///    * ignoring active sessions.
    ///    * System principals will always have permissions granted.
    ///    * This function will perform a pref lookup to permissions.default.<type>
    ///    * if the specific permission type is part of the whitelist for that functionality.
    ///    *
    ///    * @param principal the principal
    ///    * @param type      a case-sensitive ASCII string, identifying the consumer
    ///    * @param return    see add(), param permission. returns UNKNOWN_ACTION when
    ///    *                  there is no stored permission for this uri and / or type.
    ///    */
    /// ```
    ///

    /// `uint32_t testExactPermanentPermission (in nsIPrincipal principal, in ACString type);`
    #[inline]
    pub unsafe fn TestExactPermanentPermission(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).TestExactPermanentPermission)(self, principal, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the permission object associated with the given principal and action.
    ///    * @param principal The principal
    ///    * @param type      A case-sensitive ASCII string identifying the consumer
    ///    * @param exactHost If true, only the specific host will be matched,
    ///    *                  @see testExactPermission. If false, subdomains will
    ///    *                  also be searched, @see testPermission.
    ///    * @returns The matching permission object, or null if no matching object
    ///    *          was found. No matching object is equivalent to UNKNOWN_ACTION.
    ///    * @note Clients in general should prefer the test* methods unless they
    ///    *       need to know the specific stored details.
    ///    * @note This method will always return null for the system principal.
    ///    */
    /// ```
    ///

    /// `nsIPermission getPermissionObject (in nsIPrincipal principal, in ACString type, in boolean exactHost);`
    #[inline]
    pub unsafe fn GetPermissionObject(&self, principal: *const nsIPrincipal, type_: *const ::nsstring::nsACString, exactHost: bool, _retval: *mut*const nsIPermission) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissionObject)(self, principal, type_, exactHost, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns all stored permissions.
    ///    * @return an array of nsIPermission objects
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<nsIPermission> all;`
    #[inline]
    pub unsafe fn GetAll(&self, aAll: *mut thin_vec::ThinVec<RefPtr<nsIPermission>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAll)(self, aAll)
    }


    /// ```text
    /// /**
    ///    * Remove all permissions that will match the origin pattern.
    ///    */
    /// ```
    ///

    /// `void removePermissionsWithAttributes (in AString patternAsJSON);`
    #[inline]
    pub unsafe fn RemovePermissionsWithAttributes(&self, patternAsJSON: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemovePermissionsWithAttributes)(self, patternAsJSON)
    }


    /// ```text
    /// /**
    ///    * Broadcasts permissions for the given principal to all content processes.
    ///    *
    ///    * DO NOT USE THIS METHOD if you can avoid it. It was added in bug XXX to
    ///    * handle the current temporary implementation of ServiceWorker debugging. It
    ///    * will be removed when service worker debugging is fixed.
    ///    *
    ///    * @param aPrincipal The principal to broadcast permissions for.
    ///    */
    /// ```
    ///

    /// `void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn BroadcastPermissionsForPrincipalToAllContentProcesses(&self, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).BroadcastPermissionsForPrincipalToAllContentProcesses)(self, aPrincipal)
    }


}


