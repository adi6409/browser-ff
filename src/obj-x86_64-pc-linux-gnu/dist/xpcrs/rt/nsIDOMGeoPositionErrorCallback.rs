//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionErrorCallback.idl
//


/// `interface nsIDOMGeoPositionErrorCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMGeoPositionErrorCallback {
    vtable: *const nsIDOMGeoPositionErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMGeoPositionErrorCallback.
unsafe impl XpCom for nsIDOMGeoPositionErrorCallback {
    const IID: nsIID = nsID(0x7d9b09d9, 0x4843, 0x43eb,
        [0xa7, 0xa7, 0x67, 0xf7, 0xdd, 0xa6, 0xb3, 0xc4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMGeoPositionErrorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMGeoPositionErrorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMGeoPositionErrorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMGeoPositionErrorCallback`.
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMGeoPositionErrorCallbackCoerce for nsIDOMGeoPositionErrorCallback {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self {
        v
    }
}

impl nsIDOMGeoPositionErrorCallback {
    /// Cast this `nsIDOMGeoPositionErrorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMGeoPositionErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMGeoPositionErrorCallback {
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
impl<T: nsISupportsCoerce> nsIDOMGeoPositionErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMGeoPositionErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMGeoPositionErrorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMGeoPositionErrorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleEvent (in GeolocationPositionError positionError); */
    pub HandleEvent: unsafe extern "system" fn (this: *const nsIDOMGeoPositionErrorCallback, positionError: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMGeoPositionErrorCallback {


    /// `void handleEvent (in GeolocationPositionError positionError);`
    #[inline]
    pub unsafe fn HandleEvent(&self, positionError: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).HandleEvent)(self, positionError)
    }


}


