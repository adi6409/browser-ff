//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/wifi/nsIWifiListener.idl
//


/// `interface nsIWifiListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWifiListener {
    vtable: *const nsIWifiListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWifiListener.
unsafe impl XpCom for nsIWifiListener {
    const IID: nsIID = nsID(0xbcd4bede, 0xf4a5, 0x4a62,
        [0x90, 0x71, 0xd7, 0xa6, 0x01, 0x74, 0xe3, 0x76]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWifiListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWifiListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWifiListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWifiListener`.
    fn coerce_from(v: &nsIWifiListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWifiListenerCoerce for nsIWifiListener {
    #[inline]
    fn coerce_from(v: &nsIWifiListener) -> &Self {
        v
    }
}

impl nsIWifiListener {
    /// Cast this `nsIWifiListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWifiListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWifiListener {
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
impl<T: nsISupportsCoerce> nsIWifiListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWifiListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWifiListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWifiListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onChange (in Array<nsIWifiAccessPoint> accessPoints); */
    pub OnChange: unsafe extern "system" fn (this: *const nsIWifiListener, accessPoints: *const thin_vec::ThinVec<RefPtr<nsIWifiAccessPoint>>) -> ::nserror::nsresult,

    /* void onError (in nsresult error); */
    pub OnError: unsafe extern "system" fn (this: *const nsIWifiListener, error: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWifiListener {


    /// `void onChange (in Array<nsIWifiAccessPoint> accessPoints);`
    #[inline]
    pub unsafe fn OnChange(&self, accessPoints: *const thin_vec::ThinVec<RefPtr<nsIWifiAccessPoint>>) -> ::nserror::nsresult {
        ((*self.vtable).OnChange)(self, accessPoints)
    }



    /// `void onError (in nsresult error);`
    #[inline]
    pub unsafe fn OnError(&self, error: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnError)(self, error)
    }


}


