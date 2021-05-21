//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerPrompter.idl
//


/// `interface nsILoginManagerPrompter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginManagerPrompter {
    vtable: *const nsILoginManagerPrompterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginManagerPrompter.
unsafe impl XpCom for nsILoginManagerPrompter {
    const IID: nsIID = nsID(0xc47ff942, 0x9678, 0x44a5,
        [0xbc, 0x9b, 0x05, 0xe0, 0xd6, 0x76, 0xc7, 0x9c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginManagerPrompter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginManagerPrompter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginManagerPrompterCoerce {
    /// Cheaply cast a value of this type from a `nsILoginManagerPrompter`.
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginManagerPrompterCoerce for nsILoginManagerPrompter {
    #[inline]
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self {
        v
    }
}

impl nsILoginManagerPrompter {
    /// Cast this `nsILoginManagerPrompter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginManagerPrompterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginManagerPrompter {
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
impl<T: nsISupportsCoerce> nsILoginManagerPrompterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerPrompter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginManagerPrompter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginManagerPrompterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void promptToSavePassword (in Element aBrowser, in nsILoginInfo aLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub PromptToSavePassword: *const ::libc::c_void,

    /* void promptToChangePassword (in Element aBrowser, in nsILoginInfo aOldLogin, in nsILoginInfo aNewLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoSavedLoginGuid, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub PromptToChangePassword: *const ::libc::c_void,

    /* void promptToChangePasswordWithUsernames (in Element aBrowser, in Array<nsILoginInfo> logins, in nsILoginInfo aNewLogin); */
    pub PromptToChangePasswordWithUsernames: unsafe extern "system" fn (this: *const nsILoginManagerPrompter, aBrowser: *const libc::c_void, logins: *const thin_vec::ThinVec<RefPtr<nsILoginInfo>>, aNewLogin: *const nsILoginInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginManagerPrompter {

    /// ```text
    /// /**
    ///    * Ask the user if they want to save a login (Yes, Never, Not Now)
    ///    *
    ///    * @param aBrowser
    ///    *        The browser of the webpage request that triggered the prompt.
    ///    * @param aLogin
    ///    *        The login to be saved.
    ///    * @param dismissed
    ///    *        A boolean value indicating whether the save logins doorhanger should
    ///    *        be dismissed automatically when shown.
    ///    * @param notifySaved
    ///    *        A boolean value indicating whether the notification should indicate that
    ///    *        a login has been saved
    ///    * @param autoFilledLoginGuid
    ///    *        A string guid value for the login which was autofilled into the form
    ///    * @param possibleValues
    ///    *        Contains values from anything that we think, but are not sure, might be
    ///    *        a username or password.  Has two properties, 'usernames' and 'passwords'.
    ///    */
    /// ```
    ///

    /// `void promptToSavePassword (in Element aBrowser, in nsILoginInfo aLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues);`
    const _PromptToSavePassword: () = ();

    /// ```text
    /// /**
    ///    * Ask the user if they want to change a login's password or username.
    ///    * If the user consents, modifyLogin() will be called.
    ///    *
    ///    * @param aBrowser
    ///    *        The browser of the webpage request that triggered the prompt.
    ///    * @param aOldLogin
    ///    *        The existing login (with the old password).
    ///    * @param aNewLogin
    ///    *        The new login.
    ///    * @param dismissed
    ///    *        A boolean value indicating whether the save logins doorhanger should
    ///    *        be dismissed automatically when shown.
    ///    * @param autoSavedLoginGuid
    ///    *        A string guid value for the old login to be removed if the changes
    ///    *        match it to a different login
    ///    * @param autoFilledLoginGuid
    ///    *        A string guid value for the login which was autofilled into the form
    ///    * @param possibleValues
    ///    *        Contains values from anything that we think, but are not sure, might be
    ///    *        a username or password.  Has two properties, 'usernames' and 'passwords'.
    ///    */
    /// ```
    ///

    /// `void promptToChangePassword (in Element aBrowser, in nsILoginInfo aOldLogin, in nsILoginInfo aNewLogin, [optional] in boolean dismissed, [optional] in boolean notifySaved, [optional] in AString autoSavedLoginGuid, [optional] in AString autoFilledLoginGuid, [optional] in jsval possibleValues);`
    const _PromptToChangePassword: () = ();

    /// ```text
    /// /**
    ///    * Ask the user if they want to change the password for one of
    ///    * multiple logins, when the caller can't determine exactly which
    ///    * login should be changed. If the user consents, modifyLogin() will
    ///    * be called.
    ///    *
    ///    * @param aBrowser
    ///    *        The browser of the webpage request that triggered the prompt.
    ///    * @param logins
    ///    *        An array of existing logins.
    ///    * @param aNewLogin
    ///    *        The new login.
    ///    *
    ///    * Note: Because the caller does not know the username of the login
    ///    *       to be changed, aNewLogin.username and aNewLogin.usernameField
    ///    *       will be set (using the user's selection) before modifyLogin()
    ///    *       is called.
    ///    */
    /// ```
    ///

    /// `void promptToChangePasswordWithUsernames (in Element aBrowser, in Array<nsILoginInfo> logins, in nsILoginInfo aNewLogin);`
    #[inline]
    pub unsafe fn PromptToChangePasswordWithUsernames(&self, aBrowser: *const libc::c_void, logins: *const thin_vec::ThinVec<RefPtr<nsILoginInfo>>, aNewLogin: *const nsILoginInfo) -> ::nserror::nsresult {
        ((*self.vtable).PromptToChangePasswordWithUsernames)(self, aBrowser, logins, aNewLogin)
    }


}


