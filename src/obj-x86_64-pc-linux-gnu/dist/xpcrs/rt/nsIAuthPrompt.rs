//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthPrompt.idl
//


/// `interface nsIAuthPrompt : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthPrompt {
    vtable: *const nsIAuthPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthPrompt.
unsafe impl XpCom for nsIAuthPrompt {
    const IID: nsIID = nsID(0x358089f9, 0xee4b, 0x4711,
        [0x82, 0xfd, 0xbc, 0xd0, 0x7f, 0xc6, 0x20, 0x61]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthPrompt {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthPrompt.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthPromptCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthPrompt`.
    fn coerce_from(v: &nsIAuthPrompt) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthPromptCoerce for nsIAuthPrompt {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt) -> &Self {
        v
    }
}

impl nsIAuthPrompt {
    /// Cast this `nsIAuthPrompt` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthPrompt {
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
impl<T: nsISupportsCoerce> nsIAuthPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthPrompt) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthPrompt
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthPromptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result); */
    pub Prompt: unsafe extern "system" fn (this: *const nsIAuthPrompt, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, defaultText: *const i16, result: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd); */
    pub PromptUsernameAndPassword: unsafe extern "system" fn (this: *const nsIAuthPrompt, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, user: *mut *const i16, pwd: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd); */
    pub PromptPassword: unsafe extern "system" fn (this: *const nsIAuthPrompt, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, pwd: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthPrompt {

    pub const SAVE_PASSWORD_NEVER: i64 = 0;


    pub const SAVE_PASSWORD_FOR_SESSION: i64 = 1;


    pub const SAVE_PASSWORD_PERMANENTLY: i64 = 2;

    /// ```text
    /// /**
    ///      * Puts up a text input dialog with OK and Cancel buttons.
    ///      * Note: prompt uses separate args for the "in" and "out" values of the
    ///      *       input field, whereas the other functions use a single inout arg.
    ///      * @param  dialogText    The title for the dialog.
    ///      * @param  text          The text to display in the dialog.
    ///      * @param  passwordRealm The "realm" the password belongs to: e.g.
    ///      *                       ldap://localhost/dc=test
    ///      * @param  savePassword  One of the SAVE_PASSWORD_* options above.
    ///      * @param  defaultText   The default text to display in the text input box.
    ///      * @param  result        The value entered by the user if OK was
    ///      *                       selected.
    ///      * @return true for OK, false for Cancel
    ///      */
    /// ```
    ///

    /// `boolean prompt (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, in wstring defaultText, out wstring result);`
    #[inline]
    pub unsafe fn Prompt(&self, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, defaultText: *const i16, result: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Prompt)(self, dialogTitle, text, passwordRealm, savePassword, defaultText, result, _retval)
    }


    /// ```text
    /// /**
    ///      * Puts up a username/password dialog with OK and Cancel buttons.
    ///      * Puts up a password dialog with OK and Cancel buttons.
    ///      * @param  dialogText    The title for the dialog.
    ///      * @param  text          The text to display in the dialog.
    ///      * @param  passwordRealm The "realm" the password belongs to: e.g.
    ///      *                       ldap://localhost/dc=test
    ///      * @param  savePassword  One of the SAVE_PASSWORD_* options above.
    ///      * @param  user          The username entered in the dialog.
    ///      * @param  pwd           The password entered by the user if OK was
    ///      *                       selected.
    ///      * @return true for OK, false for Cancel
    ///      */
    /// ```
    ///

    /// `boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring user, inout wstring pwd);`
    #[inline]
    pub unsafe fn PromptUsernameAndPassword(&self, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, user: *mut *const i16, pwd: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptUsernameAndPassword)(self, dialogTitle, text, passwordRealm, savePassword, user, pwd, _retval)
    }


    /// ```text
    /// /**
    ///      * Puts up a password dialog with OK and Cancel buttons.
    ///      * @param  dialogText    The title for the dialog.
    ///      * @param  text          The text to display in the dialog.
    ///      * @param  passwordRealm The "realm" the password belongs to: e.g.
    ///      *                       ldap://localhost/dc=test. If a username is
    ///      *                       specified (http://user@site.com) it will be used
    ///      *                       when matching existing logins or saving new ones.
    ///      *                       If no username is specified, only password-only
    ///      *                       logins will be matched or saved.
    ///      *                       Note: if a username is specified, the username
    ///      *                       should be escaped.
    ///      * @param  savePassword  One of the SAVE_PASSWORD_* options above.
    ///      * @param  pwd           The password entered by the user if OK was
    ///      *                       selected.
    ///      * @return true for OK, false for Cancel
    ///      */
    /// ```
    ///

    /// `boolean promptPassword (in wstring dialogTitle, in wstring text, in wstring passwordRealm, in uint32_t savePassword, inout wstring pwd);`
    #[inline]
    pub unsafe fn PromptPassword(&self, dialogTitle: *const i16, text: *const i16, passwordRealm: *const i16, savePassword: uint32_t, pwd: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptPassword)(self, dialogTitle, text, passwordRealm, savePassword, pwd, _retval)
    }


}


