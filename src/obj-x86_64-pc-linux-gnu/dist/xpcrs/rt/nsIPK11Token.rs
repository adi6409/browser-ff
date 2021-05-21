//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11Token.idl
//


/// `interface nsIPK11Token : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPK11Token {
    vtable: *const nsIPK11TokenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPK11Token.
unsafe impl XpCom for nsIPK11Token {
    const IID: nsIID = nsID(0x51191434, 0x1dd2, 0x11b2,
        [0xa1, 0x7c, 0xe4, 0x9c, 0x4e, 0x99, 0xa4, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPK11Token {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPK11Token.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPK11TokenCoerce {
    /// Cheaply cast a value of this type from a `nsIPK11Token`.
    fn coerce_from(v: &nsIPK11Token) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPK11TokenCoerce for nsIPK11Token {
    #[inline]
    fn coerce_from(v: &nsIPK11Token) -> &Self {
        v
    }
}

impl nsIPK11Token {
    /// Cast this `nsIPK11Token` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPK11TokenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPK11Token {
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
impl<T: nsISupportsCoerce> nsIPK11TokenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPK11Token) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPK11Token
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPK11TokenVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute AUTF8String tokenName; */
    pub GetTokenName: unsafe extern "system" fn (this: *const nsIPK11Token, aTokenName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isInternalKeyToken; */
    pub GetIsInternalKeyToken: unsafe extern "system" fn (this: *const nsIPK11Token, aIsInternalKeyToken: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String tokenManID; */
    pub GetTokenManID: unsafe extern "system" fn (this: *const nsIPK11Token, aTokenManID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String tokenHWVersion; */
    pub GetTokenHWVersion: unsafe extern "system" fn (this: *const nsIPK11Token, aTokenHWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String tokenFWVersion; */
    pub GetTokenFWVersion: unsafe extern "system" fn (this: *const nsIPK11Token, aTokenFWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute AUTF8String tokenSerialNumber; */
    pub GetTokenSerialNumber: unsafe extern "system" fn (this: *const nsIPK11Token, aTokenSerialNumber: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] boolean isLoggedIn (); */
    pub IsLoggedIn: unsafe extern "system" fn (this: *const nsIPK11Token, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void login (in boolean force); */
    pub Login: unsafe extern "system" fn (this: *const nsIPK11Token, force: bool) -> ::nserror::nsresult,

    /* [must_use] void logoutSimple (); */
    pub LogoutSimple: unsafe extern "system" fn (this: *const nsIPK11Token) -> ::nserror::nsresult,

    /* [must_use] void logoutAndDropAuthenticatedResources (); */
    pub LogoutAndDropAuthenticatedResources: unsafe extern "system" fn (this: *const nsIPK11Token) -> ::nserror::nsresult,

    /* [must_use] boolean needsLogin (); */
    pub NeedsLogin: unsafe extern "system" fn (this: *const nsIPK11Token, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean needsUserInit; */
    pub GetNeedsUserInit: unsafe extern "system" fn (this: *const nsIPK11Token, aNeedsUserInit: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const nsIPK11Token) -> ::nserror::nsresult,

    /* [must_use] boolean checkPassword (in AUTF8String password); */
    pub CheckPassword: unsafe extern "system" fn (this: *const nsIPK11Token, password: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void initPassword (in AUTF8String initialPassword); */
    pub InitPassword: unsafe extern "system" fn (this: *const nsIPK11Token, initialPassword: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
    pub ChangePassword: unsafe extern "system" fn (this: *const nsIPK11Token, oldPassword: *const ::nsstring::nsACString, newPassword: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean hasPassword; */
    pub GetHasPassword: unsafe extern "system" fn (this: *const nsIPK11Token, aHasPassword: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPK11Token {


    /// `[must_use] readonly attribute AUTF8String tokenName;`
    #[inline]
    pub unsafe fn GetTokenName(&self, aTokenName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenName)(self, aTokenName)
    }



    /// `[must_use] readonly attribute boolean isInternalKeyToken;`
    #[inline]
    pub unsafe fn GetIsInternalKeyToken(&self, aIsInternalKeyToken: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInternalKeyToken)(self, aIsInternalKeyToken)
    }


    /// ```text
    /// /**
    ///    * Manufacturer ID of the token.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String tokenManID;`
    #[inline]
    pub unsafe fn GetTokenManID(&self, aTokenManID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenManID)(self, aTokenManID)
    }


    /// ```text
    /// /**
    ///    * Hardware version of the token.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String tokenHWVersion;`
    #[inline]
    pub unsafe fn GetTokenHWVersion(&self, aTokenHWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenHWVersion)(self, aTokenHWVersion)
    }


    /// ```text
    /// /**
    ///    * Firmware version of the token.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute AUTF8String tokenFWVersion;`
    #[inline]
    pub unsafe fn GetTokenFWVersion(&self, aTokenFWVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenFWVersion)(self, aTokenFWVersion)
    }



    /// `[must_use] readonly attribute AUTF8String tokenSerialNumber;`
    #[inline]
    pub unsafe fn GetTokenSerialNumber(&self, aTokenSerialNumber: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenSerialNumber)(self, aTokenSerialNumber)
    }



    /// `[must_use] boolean isLoggedIn ();`
    #[inline]
    pub unsafe fn IsLoggedIn(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLoggedIn)(self, _retval)
    }



    /// `[must_use] void login (in boolean force);`
    #[inline]
    pub unsafe fn Login(&self, force: bool) -> ::nserror::nsresult {
        ((*self.vtable).Login)(self, force)
    }



    /// `[must_use] void logoutSimple ();`
    #[inline]
    pub unsafe fn LogoutSimple(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LogoutSimple)(self, )
    }



    /// `[must_use] void logoutAndDropAuthenticatedResources ();`
    #[inline]
    pub unsafe fn LogoutAndDropAuthenticatedResources(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).LogoutAndDropAuthenticatedResources)(self, )
    }



    /// `[must_use] boolean needsLogin ();`
    #[inline]
    pub unsafe fn NeedsLogin(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).NeedsLogin)(self, _retval)
    }



    /// `[must_use] readonly attribute boolean needsUserInit;`
    #[inline]
    pub unsafe fn GetNeedsUserInit(&self, aNeedsUserInit: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNeedsUserInit)(self, aNeedsUserInit)
    }



    /// `[must_use] void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


    /// ```text
    /// /**
    ///    * Checks whether the given password is correct. Logs the token out if an
    ///    * incorrect password is given.
    ///    *
    ///    * @param password The password to check.
    ///    * @return true if the password was correct, false otherwise.
    ///    */
    /// ```
    ///

    /// `[must_use] boolean checkPassword (in AUTF8String password);`
    #[inline]
    pub unsafe fn CheckPassword(&self, password: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckPassword)(self, password, _retval)
    }



    /// `[must_use] void initPassword (in AUTF8String initialPassword);`
    #[inline]
    pub unsafe fn InitPassword(&self, initialPassword: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).InitPassword)(self, initialPassword)
    }



    /// `[must_use] void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword);`
    #[inline]
    pub unsafe fn ChangePassword(&self, oldPassword: *const ::nsstring::nsACString, newPassword: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ChangePassword)(self, oldPassword, newPassword)
    }



    /// `[must_use] readonly attribute boolean hasPassword;`
    #[inline]
    pub unsafe fn GetHasPassword(&self, aHasPassword: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasPassword)(self, aHasPassword)
    }


}


