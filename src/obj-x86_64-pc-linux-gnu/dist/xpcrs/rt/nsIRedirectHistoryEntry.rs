//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectHistoryEntry.idl
//


/// `interface nsIRedirectHistoryEntry : nsISupports`
///

/// ```text
/// /**
///  * This nsIRedirectHistoryEntry defines an interface for specifying channel
///  * redirect information
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRedirectHistoryEntry {
    vtable: *const nsIRedirectHistoryEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRedirectHistoryEntry.
unsafe impl XpCom for nsIRedirectHistoryEntry {
    const IID: nsIID = nsID(0x133b2905, 0x0eba, 0x411c,
        [0xa8, 0xbb, 0xf5, 0x97, 0x87, 0x14, 0x2a, 0xa2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRedirectHistoryEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRedirectHistoryEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRedirectHistoryEntryCoerce {
    /// Cheaply cast a value of this type from a `nsIRedirectHistoryEntry`.
    fn coerce_from(v: &nsIRedirectHistoryEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRedirectHistoryEntryCoerce for nsIRedirectHistoryEntry {
    #[inline]
    fn coerce_from(v: &nsIRedirectHistoryEntry) -> &Self {
        v
    }
}

impl nsIRedirectHistoryEntry {
    /// Cast this `nsIRedirectHistoryEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRedirectHistoryEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRedirectHistoryEntry {
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
impl<T: nsISupportsCoerce> nsIRedirectHistoryEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRedirectHistoryEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRedirectHistoryEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRedirectHistoryEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIRedirectHistoryEntry, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIURI referrerURI; */
    pub GetReferrerURI: unsafe extern "system" fn (this: *const nsIRedirectHistoryEntry, aReferrerURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute ACString remoteAddress; */
    pub GetRemoteAddress: unsafe extern "system" fn (this: *const nsIRedirectHistoryEntry, aRemoteAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRedirectHistoryEntry {

    /// ```text
    /// /**
    ///    * The principal of this redirect entry
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * The referring URI of this redirect entry.  This may be null.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI referrerURI;`
    #[inline]
    pub unsafe fn GetReferrerURI(&self, aReferrerURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerURI)(self, aReferrerURI)
    }


    /// ```text
    /// /**
    ///    * The remote address of this redirect entry.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString remoteAddress;`
    #[inline]
    pub unsafe fn GetRemoteAddress(&self, aRemoteAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteAddress)(self, aRemoteAddress)
    }


}


