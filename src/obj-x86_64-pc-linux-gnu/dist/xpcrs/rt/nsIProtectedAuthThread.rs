//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIProtectedAuthThread.idl
//


/// `interface nsIProtectedAuthThread : nsISupports`
///

/// ```text
/// /**
///  *  Used to communicate with the thread for logging on to a token with
///  *  CKF_PROTECTED_AUTHENTICATION_PATH set.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtectedAuthThread {
    vtable: *const nsIProtectedAuthThreadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtectedAuthThread.
unsafe impl XpCom for nsIProtectedAuthThread {
    const IID: nsIID = nsID(0x4bb27cb7, 0x8984, 0x4cee,
        [0x8c, 0xe7, 0x9b, 0x01, 0x4c, 0x3d, 0x09, 0x1b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtectedAuthThread {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtectedAuthThread.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtectedAuthThreadCoerce {
    /// Cheaply cast a value of this type from a `nsIProtectedAuthThread`.
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtectedAuthThreadCoerce for nsIProtectedAuthThread {
    #[inline]
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self {
        v
    }
}

impl nsIProtectedAuthThread {
    /// Cast this `nsIProtectedAuthThread` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtectedAuthThreadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtectedAuthThread {
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
impl<T: nsISupportsCoerce> nsIProtectedAuthThreadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtectedAuthThread) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtectedAuthThread
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtectedAuthThreadVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void login (in nsIObserver observer); */
    pub Login: unsafe extern "system" fn (this: *const nsIProtectedAuthThread, observer: *const nsIObserver) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIPKCS11Slot slot; */
    pub GetSlot: unsafe extern "system" fn (this: *const nsIProtectedAuthThread, aSlot: *mut *const nsIPKCS11Slot) -> ::nserror::nsresult,

    /* [must_use] AString getTokenName (); */
    pub GetTokenName: unsafe extern "system" fn (this: *const nsIProtectedAuthThread, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtectedAuthThread {

    /// ```text
    /// /**
    ///    * login - run the thread
    ///    *   A user interface implementing this interface needs to
    ///    *   call this method as soon as the message to the user is
    ///    *   displayed. This will trigger login operation. No user
    ///    *   cancellation is possible during login operation.
    ///    *
    ///    *   When the login is done, the observe method of @observer will
    ///    *   be called on the UI thread with a topic of "login-finished"
    ///    *   and null data and subject.
    ///    */
    /// ```
    ///

    /// `[must_use] void login (in nsIObserver observer);`
    #[inline]
    pub unsafe fn Login(&self, observer: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).Login)(self, observer)
    }


    /// ```text
    /// /**
    ///    * The PKCS11 slot
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIPKCS11Slot slot;`
    #[inline]
    pub unsafe fn GetSlot(&self, aSlot: *mut *const nsIPKCS11Slot) -> ::nserror::nsresult {
        ((*self.vtable).GetSlot)(self, aSlot)
    }


    /// ```text
    /// /**
    ///    * Gets token to be logged in name.
    ///    */
    /// ```
    ///

    /// `[must_use] AString getTokenName ();`
    #[inline]
    pub unsafe fn GetTokenName(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTokenName)(self, _retval)
    }


}


