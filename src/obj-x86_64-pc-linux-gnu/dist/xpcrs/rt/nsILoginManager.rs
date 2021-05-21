//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManager.idl
//


/// `interface nsILoginManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginManager {
    vtable: *const nsILoginManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginManager.
unsafe impl XpCom for nsILoginManager {
    const IID: nsIID = nsID(0x38c7f6af, 0x7df9, 0x49c7,
        [0xb5, 0x58, 0x27, 0x76, 0xb2, 0x4e, 0x6c, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginManagerCoerce {
    /// Cheaply cast a value of this type from a `nsILoginManager`.
    fn coerce_from(v: &nsILoginManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginManagerCoerce for nsILoginManager {
    #[inline]
    fn coerce_from(v: &nsILoginManager) -> &Self {
        v
    }
}

impl nsILoginManager {
    /// Cast this `nsILoginManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginManager {
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
impl<T: nsISupportsCoerce> nsILoginManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Promise initializationPromise; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetInitializationPromise: *const ::libc::c_void,

    /* nsILoginInfo addLogin (in nsILoginInfo aLogin); */
    pub AddLogin: unsafe extern "system" fn (this: *const nsILoginManager, aLogin: *const nsILoginInfo, _retval: *mut*const nsILoginInfo) -> ::nserror::nsresult,

    /* Promise addLogins (in jsval aLogins); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddLogins: *const ::libc::c_void,

    /* void removeLogin (in nsILoginInfo aLogin); */
    pub RemoveLogin: unsafe extern "system" fn (this: *const nsILoginManager, aLogin: *const nsILoginInfo) -> ::nserror::nsresult,

    /* void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData); */
    pub ModifyLogin: unsafe extern "system" fn (this: *const nsILoginManager, oldLogin: *const nsILoginInfo, newLoginData: *const nsISupports) -> ::nserror::nsresult,

    /* void recordPasswordUse (in nsILoginInfo aLogin, in boolean aPrivateContextWithoutExplicitConsent, in AString aLoginType, in boolean aFilled); */
    pub RecordPasswordUse: unsafe extern "system" fn (this: *const nsILoginManager, aLogin: *const nsILoginInfo, aPrivateContextWithoutExplicitConsent: bool, aLoginType: *const ::nsstring::nsAString, aFilled: bool) -> ::nserror::nsresult,

    /* void removeAllUserFacingLogins (); */
    pub RemoveAllUserFacingLogins: unsafe extern "system" fn (this: *const nsILoginManager) -> ::nserror::nsresult,

    /* void removeAllLogins (); */
    pub RemoveAllLogins: unsafe extern "system" fn (this: *const nsILoginManager) -> ::nserror::nsresult,

    /* Array<nsILoginInfo> getAllLogins (); */
    pub GetAllLogins: unsafe extern "system" fn (this: *const nsILoginManager, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult,

    /* Promise getAllLoginsAsync (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetAllLoginsAsync: *const ::libc::c_void,

    /* Array<AString> getAllDisabledHosts (); */
    pub GetAllDisabledHosts: unsafe extern "system" fn (this: *const nsILoginManager, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* boolean getLoginSavingEnabled (in AString aHost); */
    pub GetLoginSavingEnabled: unsafe extern "system" fn (this: *const nsILoginManager, aHost: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void setLoginSavingEnabled (in AString aHost, in boolean isEnabled); */
    pub SetLoginSavingEnabled: unsafe extern "system" fn (this: *const nsILoginManager, aHost: *const ::nsstring::nsAString, isEnabled: bool) -> ::nserror::nsresult,

    /* Array<nsILoginInfo> findLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
    pub FindLogins: unsafe extern "system" fn (this: *const nsILoginManager, aOrigin: *const ::nsstring::nsAString, aActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult,

    /* unsigned long countLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm); */
    pub CountLogins: unsafe extern "system" fn (this: *const nsILoginManager, aOrigin: *const ::nsstring::nsAString, aActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult,

    /* Promise searchLoginsAsync (in jsval matchData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SearchLoginsAsync: *const ::libc::c_void,

    /* Array<nsILoginInfo> searchLogins (in nsIPropertyBag matchData); */
    pub SearchLogins: unsafe extern "system" fn (this: *const nsILoginManager, matchData: *const nsIPropertyBag, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult,

    /* Promise getSyncID (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetSyncID: *const ::libc::c_void,

    /* Promise setSyncID (in AString syncID); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SetSyncID: *const ::libc::c_void,

    /* Promise getLastSync (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetLastSync: *const ::libc::c_void,

    /* Promise setLastSync (in double timestamp); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub SetLastSync: *const ::libc::c_void,

    /* readonly attribute boolean uiBusy; */
    pub GetUiBusy: unsafe extern "system" fn (this: *const nsILoginManager, aUiBusy: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isLoggedIn; */
    pub GetIsLoggedIn: unsafe extern "system" fn (this: *const nsILoginManager, aIsLoggedIn: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginManager {

    /// ```text
    /// /**
    ///    * This promise is resolved when initialization is complete, and is rejected
    ///    * in case initialization failed.  This includes the initial loading of the
    ///    * login data as well as any migration from previous versions.
    ///    *
    ///    * Calling any method of nsILoginManager before this promise is resolved
    ///    * might trigger the synchronous initialization fallback.
    ///    */
    /// ```
    ///

    /// `readonly attribute Promise initializationPromise;`
    const _GetInitializationPromise: () = ();

    /// ```text
    /// /**
    ///    * Store a new login in the login manager.
    ///    *
    ///    * @param aLogin
    ///    *        The login to be added.
    ///    * @return a clone of the login info with the guid set (even if it was not provided)
    ///    *
    ///    * Default values for the login's nsILoginMetaInfo properties will be
    ///    * created. However, if the caller specifies non-default values, they will
    ///    * be used instead.
    ///    */
    /// ```
    ///

    /// `nsILoginInfo addLogin (in nsILoginInfo aLogin);`
    #[inline]
    pub unsafe fn AddLogin(&self, aLogin: *const nsILoginInfo, _retval: *mut*const nsILoginInfo) -> ::nserror::nsresult {
        ((*self.vtable).AddLogin)(self, aLogin, _retval)
    }


    /// ```text
    /// /**
    ///    * Like addLogin, but asynchronous and for many logins.
    ///    *
    ///    * @param aLogins
    ///    *        A JS Array of nsILoginInfos to add.
    ///    * @return A promise which resolves with a JS Array of cloned logins with
    ///    *         the guids set.
    ///    *
    ///    * Default values for each login's nsILoginMetaInfo properties will be
    ///    * created. However, if the caller specifies non-default values, they will
    ///    * be used instead.
    ///    */
    /// ```
    ///

    /// `Promise addLogins (in jsval aLogins);`
    const _AddLogins: () = ();

    /// ```text
    /// /**
    ///    * Remove a login from the login manager.
    ///    *
    ///    * @param aLogin
    ///    *        The login to be removed.
    ///    *
    ///    * The specified login must exactly match a stored login. However, the
    ///    * values of any nsILoginMetaInfo properties are ignored.
    ///    */
    /// ```
    ///

    /// `void removeLogin (in nsILoginInfo aLogin);`
    #[inline]
    pub unsafe fn RemoveLogin(&self, aLogin: *const nsILoginInfo) -> ::nserror::nsresult {
        ((*self.vtable).RemoveLogin)(self, aLogin)
    }


    /// ```text
    /// /**
    ///    * Modify an existing login in the login manager.
    ///    *
    ///    * @param oldLogin
    ///    *        The login to be modified.
    ///    * @param newLoginData
    ///    *        The new login values (either a nsILoginInfo or nsIProperyBag)
    ///    *
    ///    * If newLoginData is a nsILoginInfo, all of the old login's nsILoginInfo
    ///    * properties are changed to the values from newLoginData (but the old
        ///    * login's nsILoginMetaInfo properties are unmodified).
    ///    *
    ///    * If newLoginData is a nsIPropertyBag, only the specified properties
    ///    * will be changed. The nsILoginMetaInfo properties of oldLogin can be
    ///    * changed in this manner.
    ///    *
    ///    * If the propertybag contains an item named "timesUsedIncrement", the
    ///    * login's timesUsed property will be incremented by the item's value.
    ///    */
    /// ```
    ///

    /// `void modifyLogin (in nsILoginInfo oldLogin, in nsISupports newLoginData);`
    #[inline]
    pub unsafe fn ModifyLogin(&self, oldLogin: *const nsILoginInfo, newLoginData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).ModifyLogin)(self, oldLogin, newLoginData)
    }


    /// ```text
    /// /**
    ///    * Record that the password of a saved login was used (e.g. submitted or copied).
    ///    *
    ///    * @param {nsILoginInfo} aLogin
    ///    *        The login record of the password that was used.
    ///    * @param {boolean} aPrivateContextWithoutExplicitConsent
    ///    *        If the use was in private browsing AND without the user explicitly choosing to save/update.
    ///    *        Login use metadata will not be updated in this case but it will stil be counted for telemetry.
    ///    * @param {AString} aLoginType
    ///    *        One of "form_login", "form_password", "auth_login", or "prompt_login".
    ///    *        See saved_login_used in Events.yaml.
    ///    *        Don't assume that an auth. login is never used in a form and vice-versa. This argument
    ///    *        indicates the context of how it was used.
    ///    * @param {boolean} aFilled
    ///    *        Whether the login was filled, rather than being typed manually.
    ///    *
    ///    * If only the username was used, this method shouldn't be called as we don't
    ///    * want to double-count the use if both the username and password are copied.
    ///    * Copying of the username normally precedes the copying of the password anyways.
    ///    */
    /// ```
    ///

    /// `void recordPasswordUse (in nsILoginInfo aLogin, in boolean aPrivateContextWithoutExplicitConsent, in AString aLoginType, in boolean aFilled);`
    #[inline]
    pub unsafe fn RecordPasswordUse(&self, aLogin: *const nsILoginInfo, aPrivateContextWithoutExplicitConsent: bool, aLoginType: *const ::nsstring::nsAString, aFilled: bool) -> ::nserror::nsresult {
        ((*self.vtable).RecordPasswordUse)(self, aLogin, aPrivateContextWithoutExplicitConsent, aLoginType, aFilled)
    }


    /// ```text
    /// /**
    ///    * Remove all stored user facing logins.
    ///    *
    ///    * This will remove all the logins that a user can access through about:logins.
    ///    * This will not remove the FxA Sync key which is stored with the rest of a user's logins
    ///    * but is not accessible through about:logins
    ///    *
    ///    * The browser sanitization feature allows the user to clear any stored
    ///    * passwords. This interface allows that to be done without getting each
    ///    * login first.
    ///    *
    ///    */
    /// ```
    ///

    /// `void removeAllUserFacingLogins ();`
    #[inline]
    pub unsafe fn RemoveAllUserFacingLogins(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllUserFacingLogins)(self, )
    }


    /// ```text
    /// /**
    ///    * Completely remove all logins, including the user's FxA Sync key.
    ///    *
    ///    */
    /// ```
    ///

    /// `void removeAllLogins ();`
    #[inline]
    pub unsafe fn RemoveAllLogins(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAllLogins)(self, )
    }


    /// ```text
    /// /**
    ///    * Fetch all logins in the login manager. An array is always returned;
    ///    * if there are no logins the array is empty.
    ///    *
    ///    * @deprecated Use `getAllLoginsAsync` instead.
    ///    *
    ///    * @return An array of nsILoginInfo objects.
    ///    */
    /// ```
    ///

    /// `Array<nsILoginInfo> getAllLogins ();`
    #[inline]
    pub unsafe fn GetAllLogins(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllLogins)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Like getAllLogins, but asynchronous. This method is faster when large
    ///    * amounts of logins are present since it will handle decryption in one batch.
    ///    *
    ///    * @return A promise which resolves with a JS Array of nsILoginInfo objects.
    ///    */
    /// ```
    ///

    /// `Promise getAllLoginsAsync ();`
    const _GetAllLoginsAsync: () = ();

    /// ```text
    /// /**
    ///    * Obtain a list of all origins for which password saving is disabled.
    ///    *
    ///    * @return An array of origin strings. For example: ["https://www.site.com"].
    ///    */
    /// ```
    ///

    /// `Array<AString> getAllDisabledHosts ();`
    #[inline]
    pub unsafe fn GetAllDisabledHosts(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllDisabledHosts)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Check to see if saving logins has been disabled for an origin.
    ///    *
    ///    * @param aHost
    ///    *        The origin to check. For example: "http://foo.com".
    ///    */
    /// ```
    ///

    /// `boolean getLoginSavingEnabled (in AString aHost);`
    #[inline]
    pub unsafe fn GetLoginSavingEnabled(&self, aHost: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLoginSavingEnabled)(self, aHost, _retval)
    }


    /// ```text
    /// /**
    ///    * Disable (or enable) storing logins for the specified origin. When
    ///    * disabled, the login manager will not prompt to store logins for
    ///    * that origin. Existing logins are not affected.
    ///    *
    ///    * @param aHost
    ///    *        The origin to set. For example: "http://foo.com".
    ///    * @param isEnabled
    ///    *        Specify if saving logins should be enabled (true) or
    ///    *        disabled (false)
    ///    */
    /// ```
    ///

    /// `void setLoginSavingEnabled (in AString aHost, in boolean isEnabled);`
    #[inline]
    pub unsafe fn SetLoginSavingEnabled(&self, aHost: *const ::nsstring::nsAString, isEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetLoginSavingEnabled)(self, aHost, isEnabled)
    }


    /// ```text
    /// /**
    ///    * Search for logins matching the specified criteria. Called when looking
    ///    * for logins that might be applicable to a form or authentication request.
    ///    *
    ///    * @deprecated Use `searchLoginsAsync` instead.
    ///    *
    ///    * @param aOrigin
    ///    *        The origin to restrict searches to. For example: "http://www.site.com".
    ///    *        To find logins for a given nsIURI, you would typically pass in
    ///    *        its prePath (excluding userPass).
    ///    * @param aActionOrigin
    ///    *        For form logins, this argument should be the origin to which the
    ///    *        form will be submitted, not the whole URL.
    ///    *        For HTTP auth. logins, specify null.
    ///    *        An empty string ("") will match any value (except null).
    ///    * @param aHttpRealm
    ///    *        For HTTP auth. logins, this argument should be the HTTP Realm
    ///    *        for which the login applies. This is obtained from the
    ///    *        WWW-Authenticate header. See RFC2617. For form logins,
    ///    *        specify null.
    ///    *        An empty string ("") will match any value (except null).
    ///    * @return An array of nsILoginInfo objects.
    ///    */
    /// ```
    ///

    /// `Array<nsILoginInfo> findLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm);`
    #[inline]
    pub unsafe fn FindLogins(&self, aOrigin: *const ::nsstring::nsAString, aActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult {
        ((*self.vtable).FindLogins)(self, aOrigin, aActionOrigin, aHttpRealm, _retval)
    }


    /// ```text
    /// /**
    ///    * Search for logins matching the specified criteria, as with
    ///    * findLogins(). This interface only returns the number of matching
    ///    * logins (and not the logins themselves), which allows a caller to
    ///    * check for logins without causing the user to be prompted for a master
    ///    * password to decrypt the logins.
    ///    *
    ///    * @param aOrigin
    ///    *        The origin to restrict searches to. Specify an empty string
    ///    *        to match all origins. A null value will not match any logins, and
    ///    *        will thus always return a count of 0.
    ///    * @param aActionOrigin
    ///    *        The origin to which a form login will be submitted. To match any
    ///    *        form login, specify an empty string. To not match any form
    ///    *        login, specify null.
    ///    * @param aHttpRealm
    ///    *        The HTTP Realm for which the login applies. To match logins for
    ///    *        any realm, specify an empty string. To not match logins for any
    ///    *        realm, specify null.
    ///    */
    /// ```
    ///

    /// `unsigned long countLogins (in AString aOrigin, in AString aActionOrigin, in AString aHttpRealm);`
    #[inline]
    pub unsafe fn CountLogins(&self, aOrigin: *const ::nsstring::nsAString, aActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).CountLogins)(self, aOrigin, aActionOrigin, aHttpRealm, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchonously search for logins in the login manager. The Promise always
    ///    * resolves to an array; if there are no logins the array is empty.
    ///    *
    ///    * @param {object} matchData
    ///    *        The data used to search as a JS object. This does not follow the same
    ///    *        requirements as findLogins for those fields. Wildcard matches are
    ///    *        simply not specified. If a `guid` is specified then no other properties
    ///    *        are used (outside of GeckoView).
    ///    * @return A promise resolving to an array of nsILoginInfo objects.
    ///    */
    /// ```
    ///

    /// `Promise searchLoginsAsync (in jsval matchData);`
    const _SearchLoginsAsync: () = ();

    /// ```text
    /// /**
    ///    * Search for logins in the login manager. An array is always returned;
    ///    * if there are no logins the array is empty.
    ///    * @deprecated New code should use `searchLoginsAsync`.
    ///    *             Only autocomplete, prompt, and test code still use this.
    ///    *
    ///    * @param matchData
    ///    *        The data used to search. This does not follow the same
    ///    *        requirements as findLogins for those fields. Wildcard matches are
    ///    *        simply not specified. If a `guid` is specified then no other properties
    ///    *        are used (outside of GeckoView).
    ///    * @return An array of nsILoginInfo objects.
    ///    */
    /// ```
    ///

    /// `Array<nsILoginInfo> searchLogins (in nsIPropertyBag matchData);`
    #[inline]
    pub unsafe fn SearchLogins(&self, matchData: *const nsIPropertyBag, _retval: *mut thin_vec::ThinVec<RefPtr<nsILoginInfo>>) -> ::nserror::nsresult {
        ((*self.vtable).SearchLogins)(self, matchData, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the "sync id" used by Sync to know whether the store is current with
    ///    * respect to the sync servers.
    ///    *
    ///    * Returns null if the data doesn't exist or if the data can't be
    ///    * decrypted (including if the master-password prompt is cancelled). This is
    ///    * OK for Sync as it can't even begin syncing if the master-password is
    ///    * locked as the sync encrytion keys are stored in this login manager.
    ///    */
    /// ```
    ///

    /// `Promise getSyncID ();`
    const _GetSyncID: () = ();

    /// ```text
    /// /**
    ///    * Sets the "sync id" used by Sync to know whether the store is current with
    ///    * respect to the sync servers. May be set to null.
    ///    *
    ///    * Throws if the data can't be encrypted (including if the master-password
        ///    * prompt is cancelled)
    ///    */
    /// ```
    ///

    /// `Promise setSyncID (in AString syncID);`
    const _SetSyncID: () = ();

    /// ```text
    /// /**
    ///    * Returns the timestamp of the last sync as a double (in seconds since Epoch
        ///    * rounded to two decimal places), or 0.0 if the data doesn't exist.
    ///    */
    /// ```
    ///

    /// `Promise getLastSync ();`
    const _GetLastSync: () = ();

    /// ```text
    /// /**
    ///    * Sets the timestamp of the last sync.
    ///    */
    /// ```
    ///

    /// `Promise setLastSync (in double timestamp);`
    const _SetLastSync: () = ();

    /// ```text
    /// /**
    ///   * True when a master password prompt is being displayed.
    ///   */
    /// ```
    ///

    /// `readonly attribute boolean uiBusy;`
    #[inline]
    pub unsafe fn GetUiBusy(&self, aUiBusy: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUiBusy)(self, aUiBusy)
    }


    /// ```text
    /// /**
    ///   * True when the master password has already been entered, and so a caller
    ///   * can ask for decrypted logins without triggering a prompt.
    ///   */
    /// ```
    ///

    /// `readonly attribute boolean isLoggedIn;`
    #[inline]
    pub unsafe fn GetIsLoggedIn(&self, aIsLoggedIn: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLoggedIn)(self, aIsLoggedIn)
    }


}


