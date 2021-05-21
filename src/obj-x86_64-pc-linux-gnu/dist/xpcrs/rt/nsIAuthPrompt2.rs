//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPrompt2.idl
//


/// `interface nsIAuthPrompt2 : nsISupports`
///

/// ```text
/// /**
///  * An interface allowing to prompt for a username and password. This interface
///  * is usually acquired using getInterface on notification callbacks or similar.
///  * It can be used to prompt users for authentication information, either
///  * synchronously or asynchronously.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthPrompt2 {
    vtable: *const nsIAuthPrompt2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthPrompt2.
unsafe impl XpCom for nsIAuthPrompt2 {
    const IID: nsIID = nsID(0x651395eb, 0x8612, 0x4876,
        [0x8a, 0xc0, 0xa8, 0x8d, 0x4d, 0xce, 0x9e, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthPrompt2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthPrompt2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthPrompt2Coerce {
    /// Cheaply cast a value of this type from a `nsIAuthPrompt2`.
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthPrompt2Coerce for nsIAuthPrompt2 {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self {
        v
    }
}

impl nsIAuthPrompt2 {
    /// Cast this `nsIAuthPrompt2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthPrompt2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthPrompt2 {
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
impl<T: nsISupportsCoerce> nsIAuthPrompt2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthPrompt2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthPrompt2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean promptAuth (in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo); */
    pub PromptAuth: unsafe extern "system" fn (this: *const nsIAuthPrompt2, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsICancelable asyncPromptAuth (in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo); */
    pub AsyncPromptAuth: unsafe extern "system" fn (this: *const nsIAuthPrompt2, aChannel: *const nsIChannel, aCallback: *const nsIAuthPromptCallback, aContext: *const nsISupports, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthPrompt2 {
    /// ```text
    /// /** @name Security Levels */
    /// /**
    ///    * The password will be sent unencrypted. No security provided.
    ///    */
    /// ```
    ///

    pub const LEVEL_NONE: i64 = 0;

    /// ```text
    /// /**
    ///    * Password will be sent encrypted, but the connection is otherwise
    ///    * insecure.
    ///    */
    /// ```
    ///

    pub const LEVEL_PW_ENCRYPTED: i64 = 1;

    /// ```text
    /// /**
    ///    * The connection, both for password and data, is secure.
    ///    */
    /// ```
    ///

    pub const LEVEL_SECURE: i64 = 2;

    /// ```text
    /// /**
    ///    * Requests a username and a password. Implementations will commonly show a
    ///    * dialog with a username and password field, depending on flags also a
    ///    * domain field.
    ///    *
    ///    * @param aChannel
    ///    *        The channel that requires authentication.
    ///    * @param level
    ///    *        One of the level constants from above. See there for descriptions
    ///    *        of the levels.
    ///    * @param authInfo
    ///    *        Authentication information object. The implementation should fill in
    ///    *        this object with the information entered by the user before
    ///    *        returning.
    ///    *
    ///    * @retval true
    ///    *         Authentication can proceed using the values in the authInfo
    ///    *         object.
    ///    * @retval false
    ///    *         Authentication should be cancelled, usually because the user did
    ///    *         not provide username/password.
    ///    *
    ///    * @note   Exceptions thrown from this function will be treated like a
    ///    *         return value of false.
    ///    * @deprecated use asyncPromptAuth
    ///    */
    /// ```
    ///

    /// `boolean promptAuth (in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo);`
    #[inline]
    pub unsafe fn PromptAuth(&self, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptAuth)(self, aChannel, level, authInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Asynchronously prompt the user for a username and password.
    ///    * This has largely the same semantics as promptUsernameAndPassword(),
    ///    * but must return immediately after calling and return the entered
    ///    * data in a callback.
    ///    *
    ///    * If the user closes the dialog using a cancel button or similar,
    ///    * the callback's nsIAuthPromptCallback::onAuthCancelled method must be
    ///    * called.
    ///    * Calling nsICancelable::cancel on the returned object SHOULD close the
    ///    * dialog and MUST call nsIAuthPromptCallback::onAuthCancelled on the provided
    ///    * callback.
    ///    *
    ///    * This implementation may:
    ///    *
    ///    *  1) Coalesce identical prompts.  This means prompts that are guaranteed to
///    *     want the same auth information from the user.  A single prompt will be
///    *     shown; then the callbacks for all the coalesced prompts will be notified
///    *     with the resulting auth information.
///    *  2) Serialize prompts that are all in the same "context" (this might mean
///    *     application-wide, for a given window, or something else depending on
///    *     the user interface) so that the user is not deluged with prompts.
///    *
///    * @throw
///    *     This method may throw any exception when the prompt fails to queue e.g
///    *     because of out-of-memory error. It must not throw when the prompt
///    *     could already be potentially shown to the user. In that case information
///    *     about the failure has to come through the callback. This way we
///    *     prevent multiple dialogs shown to the user because consumer may fall
///    *     back to synchronous prompt on synchronous failure of this method.
///    */
/// ```
///

/// `nsICancelable asyncPromptAuth (in nsIChannel aChannel, in nsIAuthPromptCallback aCallback, in nsISupports aContext, in uint32_t level, in nsIAuthInformation authInfo);`
#[inline]
pub unsafe fn AsyncPromptAuth(&self, aChannel: *const nsIChannel, aCallback: *const nsIAuthPromptCallback, aContext: *const nsISupports, level: uint32_t, authInfo: *const nsIAuthInformation, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
((*self.vtable).AsyncPromptAuth)(self, aChannel, aCallback, aContext, level, authInfo, _retval)
}


}


