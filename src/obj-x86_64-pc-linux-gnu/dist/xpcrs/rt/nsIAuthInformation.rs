//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthInformation.idl
//


/// `interface nsIAuthInformation : nsISupports`
///

/// ```text
/// /**
///  * A object that hold authentication information. The caller of
///  * nsIAuthPrompt2::promptUsernameAndPassword or
///  * nsIAuthPrompt2::promptPasswordAsync provides an object implementing this
///  * interface; the prompt implementation can then read the values here to prefill
///  * the dialog. After the user entered the authentication information, it should
///  * set the attributes of this object to indicate to the caller what was entered
///  * by the user.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthInformation {
    vtable: *const nsIAuthInformationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthInformation.
unsafe impl XpCom for nsIAuthInformation {
    const IID: nsIID = nsID(0x0d73639c, 0x2a92, 0x4518,
        [0x9f, 0x92, 0x28, 0xf7, 0x1f, 0xea, 0x5f, 0x20]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthInformation {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthInformation.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthInformationCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthInformation`.
    fn coerce_from(v: &nsIAuthInformation) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthInformationCoerce for nsIAuthInformation {
    #[inline]
    fn coerce_from(v: &nsIAuthInformation) -> &Self {
        v
    }
}

impl nsIAuthInformation {
    /// Cast this `nsIAuthInformation` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthInformationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthInformation {
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
impl<T: nsISupportsCoerce> nsIAuthInformationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthInformation) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthInformation
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthInformationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long flags; */
    pub GetFlags: unsafe extern "system" fn (this: *const nsIAuthInformation, aFlags: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString realm; */
    pub GetRealm: unsafe extern "system" fn (this: *const nsIAuthInformation, aRealm: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String authenticationScheme; */
    pub GetAuthenticationScheme: unsafe extern "system" fn (this: *const nsIAuthInformation, aAuthenticationScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AString username; */
    pub GetUsername: unsafe extern "system" fn (this: *const nsIAuthInformation, aUsername: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString username; */
    pub SetUsername: unsafe extern "system" fn (this: *const nsIAuthInformation, aUsername: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString password; */
    pub GetPassword: unsafe extern "system" fn (this: *const nsIAuthInformation, aPassword: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString password; */
    pub SetPassword: unsafe extern "system" fn (this: *const nsIAuthInformation, aPassword: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString domain; */
    pub GetDomain: unsafe extern "system" fn (this: *const nsIAuthInformation, aDomain: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString domain; */
    pub SetDomain: unsafe extern "system" fn (this: *const nsIAuthInformation, aDomain: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthInformation {
    /// ```text
    /// /** @name Flags */
    /// /**
    ///    * This dialog belongs to a network host.
    ///    */
    /// ```
    ///

    pub const AUTH_HOST: i64 = 1;

    /// ```text
    /// /**
    ///    * This dialog belongs to a proxy.
    ///    */
    /// ```
    ///

    pub const AUTH_PROXY: i64 = 2;

    /// ```text
    /// /**
    ///    * This dialog needs domain information. The user interface should show a
    ///    * domain field, prefilled with the domain attribute's value.
    ///    */
    /// ```
    ///

    pub const NEED_DOMAIN: i64 = 4;

    /// ```text
    /// /**
    ///    * This dialog only asks for password information. Authentication prompts
    ///    * SHOULD NOT show a username field. Attempts to change the username field
    ///    * will have no effect. nsIAuthPrompt2 implementations should, however, show
    ///    * its initial value to the user in some form. For example, a paragraph in
    ///    * the dialog might say "Please enter your password for user jsmith at
    ///    * server intranet".
    ///    *
    ///    * This flag is mutually exclusive with #NEED_DOMAIN.
    ///    */
    /// ```
    ///

    pub const ONLY_PASSWORD: i64 = 8;

    /// ```text
    /// /**
    ///    * We have already tried to log in for this channel
    ///    * (with auth values from a previous promptAuth call),
    ///    * but it failed, so we now ask the user to provide a new, correct login.
    ///    *
    ///    * @see also RFC 2616, Section 10.4.2
    ///    */
    /// ```
    ///

    pub const PREVIOUS_FAILED: i64 = 16;

    /// ```text
    /// /**
    ///    * A cross-origin sub-resource requests an authentication.
    ///    * The message presented to users must reflect that.
    ///    */
    /// ```
    ///

    pub const CROSS_ORIGIN_SUB_RESOURCE: i64 = 32;

    /// ```text
    /// /**
    ///    * Flags describing this dialog. A bitwise OR of the flag values
    ///    * above.
    ///    *
    ///    * It is possible that neither #AUTH_HOST nor #AUTH_PROXY are set.
    ///    *
    ///    * Auth prompts should ignore flags they don't understand; especially, they
    ///    * should not throw an exception because of an unsupported flag.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long flags;`
    #[inline]
    pub unsafe fn GetFlags(&self, aFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFlags)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * The server-supplied realm of the authentication as defined in RFC 2617.
    ///    * Can be the empty string if the protocol does not support realms.
    ///    * Otherwise, this is a human-readable string like "Secret files".
    ///    */
    /// ```
    ///

    /// `readonly attribute AString realm;`
    #[inline]
    pub unsafe fn GetRealm(&self, aRealm: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRealm)(self, aRealm)
    }


    /// ```text
    /// /**
    ///    * The authentication scheme used for this request, if applicable. If the
    ///    * protocol for this authentication does not support schemes, this will be
    ///    * the empty string. Otherwise, this will be a string such as "basic" or
    ///    * "digest". This string will always be in lowercase.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String authenticationScheme;`
    #[inline]
    pub unsafe fn GetAuthenticationScheme(&self, aAuthenticationScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthenticationScheme)(self, aAuthenticationScheme)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * On return, this parameter should contain the username entered by
    ///    * the user.
    ///    * This field can only be changed if the #ONLY_PASSWORD flag is not set.
    ///    */
    /// ```
    ///

    /// `attribute AString username;`
    #[inline]
    pub unsafe fn GetUsername(&self, aUsername: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsername)(self, aUsername)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * On return, this parameter should contain the username entered by
    ///    * the user.
    ///    * This field can only be changed if the #ONLY_PASSWORD flag is not set.
    ///    */
    /// ```
    ///

    /// `attribute AString username;`
    #[inline]
    pub unsafe fn SetUsername(&self, aUsername: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetUsername)(self, aUsername)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * The password should not be shown in clear.
    ///    * On return, this parameter should contain the password entered by
    ///    * the user.
    ///    */
    /// ```
    ///

    /// `attribute AString password;`
    #[inline]
    pub unsafe fn GetPassword(&self, aPassword: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * The password should not be shown in clear.
    ///    * On return, this parameter should contain the password entered by
    ///    * the user.
    ///    */
    /// ```
    ///

    /// `attribute AString password;`
    #[inline]
    pub unsafe fn SetPassword(&self, aPassword: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * On return, this parameter should contain the domain entered by
    ///    * the user.
    ///    * This attribute is only used if flags include #NEED_DOMAIN.
    ///    */
    /// ```
    ///

    /// `attribute AString domain;`
    #[inline]
    pub unsafe fn GetDomain(&self, aDomain: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDomain)(self, aDomain)
    }


    /// ```text
    /// /**
    ///    * The initial value should be used to prefill the dialog or be shown
    ///    * in some other way to the user.
    ///    * On return, this parameter should contain the domain entered by
    ///    * the user.
    ///    * This attribute is only used if flags include #NEED_DOMAIN.
    ///    */
    /// ```
    ///

    /// `attribute AString domain;`
    #[inline]
    pub unsafe fn SetDomain(&self, aDomain: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDomain)(self, aDomain)
    }


}


