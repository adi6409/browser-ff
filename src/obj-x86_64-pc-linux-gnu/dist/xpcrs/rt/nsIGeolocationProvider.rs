//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGeolocationProvider.idl
//


/// `interface nsIGeolocationUpdate : nsISupports`
///

/// ```text
/// /**
///
///  * Interface provides a way for a geolocation provider to
///  * notify the system that a new location is available.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGeolocationUpdate {
    vtable: *const nsIGeolocationUpdateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGeolocationUpdate.
unsafe impl XpCom for nsIGeolocationUpdate {
    const IID: nsIID = nsID(0x643dc5e9, 0xb911, 0x4b2c,
        [0x8d, 0x44, 0x60, 0x31, 0x62, 0x69, 0x6b, 0xaf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGeolocationUpdate {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGeolocationUpdate.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGeolocationUpdateCoerce {
    /// Cheaply cast a value of this type from a `nsIGeolocationUpdate`.
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGeolocationUpdateCoerce for nsIGeolocationUpdate {
    #[inline]
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self {
        v
    }
}

impl nsIGeolocationUpdate {
    /// Cast this `nsIGeolocationUpdate` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGeolocationUpdateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGeolocationUpdate {
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
impl<T: nsISupportsCoerce> nsIGeolocationUpdateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeolocationUpdate) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGeolocationUpdate
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGeolocationUpdateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void update (in nsIDOMGeoPosition position); */
    pub Update: unsafe extern "system" fn (this: *const nsIGeolocationUpdate, position: *const nsIDOMGeoPosition) -> ::nserror::nsresult,

    /* [can_run_script] void notifyError (in unsigned short error); */
    pub NotifyError: unsafe extern "system" fn (this: *const nsIGeolocationUpdate, error: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGeolocationUpdate {

    /// ```text
    /// /**
    ///    * Notify the geolocation service that a new geolocation
    ///    * has been discovered.
    ///    * This must be called on the main thread
    ///    */
    /// ```
    ///

    /// `void update (in nsIDOMGeoPosition position);`
    #[inline]
    pub unsafe fn Update(&self, position: *const nsIDOMGeoPosition) -> ::nserror::nsresult {
        ((*self.vtable).Update)(self, position)
    }


    /// ```text
    /// /**
    ///    * Notify the geolocation service of an error.
    ///    * This must be called on the main thread.
    ///    * The parameter refers to one of the constants in the
    ///    * nsIDOMGeoPositionError interface.
    ///    * Use this to report spurious errors coming from the
    ///    * provider; for errors occurring inside the methods in
    ///    * the nsIGeolocationProvider interface, just use the return
    ///    * value.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void notifyError (in unsigned short error);`
    #[inline]
    pub unsafe fn NotifyError(&self, error: u16) -> ::nserror::nsresult {
        ((*self.vtable).NotifyError)(self, error)
    }


}


/// `interface nsIGeolocationProvider : nsISupports`
///

/// ```text
/// /**
///  * Interface provides location information to the nsGeolocator
///  * via the nsIDOMGeolocationCallback interface.  After
///  * startup is called, any geo location change should call
///  * callback.update().
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGeolocationProvider {
    vtable: *const nsIGeolocationProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGeolocationProvider.
unsafe impl XpCom for nsIGeolocationProvider {
    const IID: nsIID = nsID(0xac4a133b, 0x9f92, 0x4f7c,
        [0xb3, 0x69, 0xd4, 0x0c, 0xb6, 0xb1, 0x76, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGeolocationProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGeolocationProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGeolocationProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIGeolocationProvider`.
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGeolocationProviderCoerce for nsIGeolocationProvider {
    #[inline]
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self {
        v
    }
}

impl nsIGeolocationProvider {
    /// Cast this `nsIGeolocationProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGeolocationProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGeolocationProvider {
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
impl<T: nsISupportsCoerce> nsIGeolocationProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeolocationProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGeolocationProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGeolocationProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void startup (); */
    pub Startup: unsafe extern "system" fn (this: *const nsIGeolocationProvider) -> ::nserror::nsresult,

    /* void watch (in nsIGeolocationUpdate callback); */
    pub Watch: unsafe extern "system" fn (this: *const nsIGeolocationProvider, callback: *const nsIGeolocationUpdate) -> ::nserror::nsresult,

    /* void shutdown (); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsIGeolocationProvider) -> ::nserror::nsresult,

    /* void setHighAccuracy (in boolean enable); */
    pub SetHighAccuracy: unsafe extern "system" fn (this: *const nsIGeolocationProvider, enable: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGeolocationProvider {

    /// ```text
    /// /**
    ///    * Start up the provider.  This is called before any other
    ///    * method.  may be called multiple times.
    ///    */
    /// ```
    ///

    /// `void startup ();`
    #[inline]
    pub unsafe fn Startup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Startup)(self, )
    }


    /// ```text
    /// /**
    ///    * watch
    ///    * When a location change is observed, notify the callback.
    ///    */
    /// ```
    ///

    /// `void watch (in nsIGeolocationUpdate callback);`
    #[inline]
    pub unsafe fn Watch(&self, callback: *const nsIGeolocationUpdate) -> ::nserror::nsresult {
        ((*self.vtable).Watch)(self, callback)
    }


    /// ```text
    /// /**
    ///    * shutdown
    ///    * Shuts down the location device.
    ///    */
    /// ```
    ///

    /// `void shutdown ();`
    #[inline]
    pub unsafe fn Shutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Shutdown)(self, )
    }


    /// ```text
    /// /**
    ///    * hint to provide to use any amount of power to provide a better result
    ///    */
    /// ```
    ///

    /// `void setHighAccuracy (in boolean enable);`
    #[inline]
    pub unsafe fn SetHighAccuracy(&self, enable: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHighAccuracy)(self, enable)
    }


}


