//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIContentSecurityManager.idl
//


/// `interface nsIContentSecurityManager : nsISupports`
///

/// ```text
/// /**
///  * nsIContentSecurityManager
///  * Describes an XPCOM component used to perform security checks.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentSecurityManager {
    vtable: *const nsIContentSecurityManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentSecurityManager.
unsafe impl XpCom for nsIContentSecurityManager {
    const IID: nsIID = nsID(0x3a9a1818, 0x2ae8, 0x4ec5,
        [0xa3, 0x40, 0x8b, 0x29, 0xd3, 0x1f, 0xca, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentSecurityManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentSecurityManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentSecurityManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIContentSecurityManager`.
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentSecurityManagerCoerce for nsIContentSecurityManager {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self {
        v
    }
}

impl nsIContentSecurityManager {
    /// Cast this `nsIContentSecurityManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentSecurityManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentSecurityManager {
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
impl<T: nsISupportsCoerce> nsIContentSecurityManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentSecurityManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentSecurityManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentSecurityManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener); */
    pub PerformSecurityCheck: unsafe extern "system" fn (this: *const nsIContentSecurityManager, aChannel: *const nsIChannel, aStreamListener: *const nsIStreamListener, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentSecurityManager {

    /// ```text
    /// /**
    ///    * Checks whether a channel is allowed to access the given URI and
    ///    * whether the channel should be openend or should be blocked consulting
    ///    * internal security checks like Same Origin Policy, Content Security
    ///    * Policy, Mixed Content Blocker, etc.
    ///    *
    ///    * If security checks within performSecurityCheck fail, the function
    ///    * throws an exception.
    ///    *
    ///    * @param aChannel
    ///    *     The channel about to be openend
    ///    * @param aStreamListener
    ///    *     The Streamlistener of the channel potentially wrapped
    ///    *     into CORSListenerProxy.
    ///    * @return
    ///    *     The StreamListener of the channel wrapped into CORSListenerProxy.
    ///    *
    ///    * @throws NS_ERROR_DOM_BAD_URI
    ///    *     If accessing the URI is not allowed (e.g. prohibted by SOP)
    ///    * @throws NS_ERROR_CONTENT_BLOCKED
    ///    *     If any of the security policies (CSP, Mixed content) is violated
    ///    */
    /// ```
    ///

    /// `nsIStreamListener performSecurityCheck (in nsIChannel aChannel, in nsIStreamListener aStreamListener);`
    #[inline]
    pub unsafe fn PerformSecurityCheck(&self, aChannel: *const nsIChannel, aStreamListener: *const nsIStreamListener, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).PerformSecurityCheck)(self, aChannel, aStreamListener, _retval)
    }


}


