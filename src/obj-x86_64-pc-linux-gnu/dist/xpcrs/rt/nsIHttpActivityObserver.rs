//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpActivityObserver.idl
//


/// `interface nsIHttpActivityObserver : nsISupports`
///

/// ```text
/// /**
///  * nsIHttpActivityObserver
///  *
///  * This interface provides a way for http activities to be reported
///  * to observers.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpActivityObserver {
    vtable: *const nsIHttpActivityObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpActivityObserver.
unsafe impl XpCom for nsIHttpActivityObserver {
    const IID: nsIID = nsID(0x412880c8, 0x6c36, 0x48d8,
        [0xbf, 0x8f, 0x84, 0xf9, 0x1f, 0x89, 0x25, 0x03]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpActivityObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpActivityObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpActivityObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpActivityObserver`.
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpActivityObserverCoerce for nsIHttpActivityObserver {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self {
        v
    }
}

impl nsIHttpActivityObserver {
    /// Cast this `nsIHttpActivityObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpActivityObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpActivityObserver {
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
impl<T: nsISupportsCoerce> nsIHttpActivityObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpActivityObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpActivityObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
    pub ObserveActivity: unsafe extern "system" fn (this: *const nsIHttpActivityObserver, aHttpChannel: *const nsISupports, aActivityType: uint32_t, aActivitySubtype: uint32_t, aTimestamp: PRTime, aExtraSizeData: uint64_t, aExtraStringData: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isActive; */
    pub GetIsActive: unsafe extern "system" fn (this: *const nsIHttpActivityObserver, aIsActive: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void setIsActive (in boolean aActived); */
    pub SetIsActive: unsafe extern "system" fn (this: *const nsIHttpActivityObserver, aActived: bool) -> ::nserror::nsresult,

    /* [must_use,noscript] void observeActivityWithArgs (in HttpActivityArgs aArgs, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData); */
    /// Unable to generate binding because `native type const mozilla::net::HttpActivityArgs unsupported`
    pub ObserveActivityWithArgs: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpActivityObserver {

    pub const ACTIVITY_TYPE_SOCKET_TRANSPORT: i64 = 1;


    pub const ACTIVITY_TYPE_HTTP_TRANSACTION: i64 = 2;


    pub const ACTIVITY_SUBTYPE_REQUEST_HEADER: i64 = 20481;


    pub const ACTIVITY_SUBTYPE_REQUEST_BODY_SENT: i64 = 20482;


    pub const ACTIVITY_SUBTYPE_RESPONSE_START: i64 = 20483;


    pub const ACTIVITY_SUBTYPE_RESPONSE_HEADER: i64 = 20484;


    pub const ACTIVITY_SUBTYPE_RESPONSE_COMPLETE: i64 = 20485;


    pub const ACTIVITY_SUBTYPE_TRANSACTION_CLOSE: i64 = 20486;

    /// ```text
    /// /**
    ///      * observe activity from the http transport
    ///      *
    ///      * @param aHttpChannel
    ///      *        nsISupports interface for the the http channel that
    ///      *        generated this activity
    ///      * @param aActivityType
    ///      *        The value of this aActivityType will be one of
    ///      *          ACTIVITY_TYPE_SOCKET_TRANSPORT or
    ///      *          ACTIVITY_TYPE_HTTP_TRANSACTION
    ///      * @param aActivitySubtype
    ///      *        The value of this aActivitySubtype, will be depend
    ///      *        on the value of aActivityType. When aActivityType
    ///      *        is ACTIVITY_TYPE_SOCKET_TRANSPORT
    ///      *          aActivitySubtype will be one of the
    ///      *          nsISocketTransport::STATUS_???? values defined in
    ///      *          nsISocketTransport.idl
    ///      *        OR when aActivityType
    ///      *        is ACTIVITY_TYPE_HTTP_TRANSACTION
    ///      *          aActivitySubtype will be one of the
    ///      *          nsIHttpActivityObserver::ACTIVITY_SUBTYPE_???? values
    ///      *          defined below
    ///      * @param aTimestamp
    ///      *        microseconds past the epoch of Jan 1, 1970
    ///      * @param aExtraSizeData
    ///      *        Any extra size data optionally available with
    ///      *        this activity
    ///      * @param aExtraStringData
    ///      *        Any extra string data optionally available with
    ///      *        this activity
    ///      */
    /// ```
    ///

    /// `[must_use] void observeActivity (in nsISupports aHttpChannel, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData);`
    #[inline]
    pub unsafe fn ObserveActivity(&self, aHttpChannel: *const nsISupports, aActivityType: uint32_t, aActivitySubtype: uint32_t, aTimestamp: PRTime, aExtraSizeData: uint64_t, aExtraStringData: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ObserveActivity)(self, aHttpChannel, aActivityType, aActivitySubtype, aTimestamp, aExtraSizeData, aExtraStringData)
    }


    /// ```text
    /// /**
    ///      * This attribute is true when this interface is active and should
    ///      * observe http activities. When false, observeActivity() should not
    ///      * be called. It is present for compatibility reasons and should be
    ///      * implemented only by nsHttpActivityDistributor.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean isActive;`
    #[inline]
    pub unsafe fn GetIsActive(&self, aIsActive: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsActive)(self, aIsActive)
    }


    /// ```text
    /// /**
    ///      * This function is for internal use only. Every time a http transaction
    ///      * is created in socket process, we use this function to set the value of
    ///      * |isActive|. We need this since the real value of |isActive| is
    ///      * only available in parent process.
    ///      */
    /// ```
    ///

    /// `[noscript] void setIsActive (in boolean aActived);`
    #[inline]
    pub unsafe fn SetIsActive(&self, aActived: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsActive)(self, aActived)
    }


    /// ```text
    /// /**
    ///      * This function is used when the real http channel is not available.
    ///      * We use the information in |HttpActivityArgs| to get the http channel or
    ///      * create a |NullHttpChannel|.
    ///      *
    ///      * @param aArgs
    ///      *        See the definition of |HttpActivityArgs| in PSocketProcess.ipdl.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] void observeActivityWithArgs (in HttpActivityArgs aArgs, in uint32_t aActivityType, in uint32_t aActivitySubtype, in PRTime aTimestamp, in uint64_t aExtraSizeData, in ACString aExtraStringData);`
    const _ObserveActivityWithArgs: () = ();

}


/// `interface nsIHttpActivityDistributor : nsIHttpActivityObserver`
///

/// ```text
/// /**
///  * nsIHttpActivityDistributor
///  *
///  * This interface provides a way to register and unregister observers to the
///  * http activities.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpActivityDistributor {
    vtable: *const nsIHttpActivityDistributorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpActivityDistributor.
unsafe impl XpCom for nsIHttpActivityDistributor {
    const IID: nsIID = nsID(0x7c512cb8, 0x582a, 0x4625,
        [0xb5, 0xb6, 0x86, 0x39, 0x75, 0x52, 0x71, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpActivityDistributor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpActivityDistributor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpActivityDistributorCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpActivityDistributor`.
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpActivityDistributorCoerce for nsIHttpActivityDistributor {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self {
        v
    }
}

impl nsIHttpActivityDistributor {
    /// Cast this `nsIHttpActivityDistributor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpActivityDistributorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpActivityDistributor {
    type Target = nsIHttpActivityObserver;
    #[inline]
    fn deref(&self) -> &nsIHttpActivityObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHttpActivityObserverCoerce> nsIHttpActivityDistributorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpActivityDistributor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpActivityDistributor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpActivityDistributorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHttpActivityObserverVTable,

    /* void addObserver (in nsIHttpActivityObserver aObserver); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsIHttpActivityDistributor, aObserver: *const nsIHttpActivityObserver) -> ::nserror::nsresult,

    /* void removeObserver (in nsIHttpActivityObserver aObserver); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsIHttpActivityDistributor, aObserver: *const nsIHttpActivityObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpActivityDistributor {


    /// `void addObserver (in nsIHttpActivityObserver aObserver);`
    #[inline]
    pub unsafe fn AddObserver(&self, aObserver: *const nsIHttpActivityObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, aObserver)
    }



    /// `void removeObserver (in nsIHttpActivityObserver aObserver);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, aObserver: *const nsIHttpActivityObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, aObserver)
    }


}


