//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIOSReauthenticator.idl
//


/// `interface nsIOSReauthenticator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOSReauthenticator {
    vtable: *const nsIOSReauthenticatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOSReauthenticator.
unsafe impl XpCom for nsIOSReauthenticator {
    const IID: nsIID = nsID(0x4fe082ae, 0x6ff0, 0x4b41,
        [0xb2, 0x4f, 0xea, 0xa6, 0x64, 0xf6, 0xe4, 0x6a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOSReauthenticator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOSReauthenticator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOSReauthenticatorCoerce {
    /// Cheaply cast a value of this type from a `nsIOSReauthenticator`.
    fn coerce_from(v: &nsIOSReauthenticator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOSReauthenticatorCoerce for nsIOSReauthenticator {
    #[inline]
    fn coerce_from(v: &nsIOSReauthenticator) -> &Self {
        v
    }
}

impl nsIOSReauthenticator {
    /// Cast this `nsIOSReauthenticator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOSReauthenticatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOSReauthenticator {
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
impl<T: nsISupportsCoerce> nsIOSReauthenticatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOSReauthenticator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOSReauthenticator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOSReauthenticatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext,must_use] Promise asyncReauthenticateUser (in AString prompt, in AString caption, in mozIDOMWindow parentWindow); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub AsyncReauthenticateUser: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOSReauthenticator {

    /// ```text
    /// /**
    ///    * This interface provides an abstract way to request that the user
    ///    * reauthenticate themselves to the operating system. It may be useful in
    ///    * conjunction with nsIOSKeyStore, whereby consumers of these APIs may
    ///    * consider some secrets too sensitive to access without first
    ///    * reauthenticating the user.
    ///    *
    ///    * Usage:
    ///    *
    ///    * // obtain the singleton nsIOSReauthenticator instance
    ///    * const reauthenticator = Cc["@mozilla.org/security/osreauthenticator;1"]
    ///    *                           .getService(Ci.nsIOSReauthenticator);
    ///    * if (await reauthenticator.asyncReauthenticate()) {
        ///    *   // do something only authenticated users are allowed to do...
        ///    * } else {
        ///    *   // show a "sorry, this isn't allowed" error
        ///    * }
    ///    */
    /// /**
    ///    * Asynchronously cause the operating system to request that the user
    ///    * reauthenticate. This is typically in the form of a dialog box asking the
    ///    * user for their login password. The actual behaviour of this depends on the
    ///    * OS.
    ///    *
    ///    * @param prompt A short string that may be incorporated in the dialog
    ///    * @param caption A short string that may be shown as the dialog caption (usually Product Name)
    ///    * @param parentWindow Used to associate the OS dialog with the calling window.
    ///    * @return Promise resolving to true if the user successfully authenticated
    ///    *         and false otherwise.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext,must_use] Promise asyncReauthenticateUser (in AString prompt, in AString caption, in mozIDOMWindow parentWindow);`
    const _AsyncReauthenticateUser: () = ();

}


