//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheListener.idl
//


/// `interface nsICacheListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICacheListener {
    vtable: *const nsICacheListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICacheListener.
unsafe impl XpCom for nsICacheListener {
    const IID: nsIID = nsID(0x8eadf2ed, 0x8cac, 0x4961,
        [0x80, 0x25, 0x6d, 0xa6, 0xd5, 0x82, 0x7e, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICacheListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICacheListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICacheListenerCoerce {
    /// Cheaply cast a value of this type from a `nsICacheListener`.
    fn coerce_from(v: &nsICacheListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICacheListenerCoerce for nsICacheListener {
    #[inline]
    fn coerce_from(v: &nsICacheListener) -> &Self {
        v
    }
}

impl nsICacheListener {
    /// Cast this `nsICacheListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICacheListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICacheListener {
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
impl<T: nsISupportsCoerce> nsICacheListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICacheListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICacheListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICacheListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status); */
    pub OnCacheEntryAvailable: unsafe extern "system" fn (this: *const nsICacheListener, descriptor: *const nsICacheEntryDescriptor, accessGranted: nsCacheAccessMode, status: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void onCacheEntryDoomed (in nsresult status); */
    pub OnCacheEntryDoomed: unsafe extern "system" fn (this: *const nsICacheListener, status: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICacheListener {

    /// ```text
    /// /**
    ///      * Called when the requested access (or appropriate subset) is
    ///      * acquired.  The status parameter equals NS_OK on success.
    ///      * See nsICacheService.idl for accessGranted values.
    ///      */
    /// ```
    ///

    /// `void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status);`
    #[inline]
    pub unsafe fn OnCacheEntryAvailable(&self, descriptor: *const nsICacheEntryDescriptor, accessGranted: nsCacheAccessMode, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryAvailable)(self, descriptor, accessGranted, status)
    }


    /// ```text
    /// /**
    ///      * Called when nsCacheSession::DoomEntry() is completed. The status
    ///      * parameter is NS_OK when the entry was doomed, or NS_ERROR_NOT_AVAILABLE
    ///      * when there is no such entry.
    ///      */
    /// ```
    ///

    /// `void onCacheEntryDoomed (in nsresult status);`
    #[inline]
    pub unsafe fn OnCacheEntryDoomed(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnCacheEntryDoomed)(self, status)
    }


}


