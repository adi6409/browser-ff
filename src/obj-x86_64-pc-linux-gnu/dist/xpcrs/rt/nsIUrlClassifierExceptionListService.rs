//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierExceptionListService.idl
//


/// `interface nsIUrlClassifierExceptionListObserver : nsISupports`
///

/// ```text
/// /**
///  * Observer for exception list updates.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierExceptionListObserver {
    vtable: *const nsIUrlClassifierExceptionListObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierExceptionListObserver.
unsafe impl XpCom for nsIUrlClassifierExceptionListObserver {
    const IID: nsIID = nsID(0xf7c918e5, 0x94bf, 0x4b6e,
        [0x97, 0x58, 0xef, 0x7b, 0xda, 0xb6, 0xaf, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierExceptionListObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierExceptionListObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierExceptionListObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierExceptionListObserver`.
    fn coerce_from(v: &nsIUrlClassifierExceptionListObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierExceptionListObserverCoerce for nsIUrlClassifierExceptionListObserver {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierExceptionListObserver) -> &Self {
        v
    }
}

impl nsIUrlClassifierExceptionListObserver {
    /// Cast this `nsIUrlClassifierExceptionListObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierExceptionListObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierExceptionListObserver {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierExceptionListObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierExceptionListObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierExceptionListObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierExceptionListObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onExceptionListUpdate (in ACString aList); */
    pub OnExceptionListUpdate: unsafe extern "system" fn (this: *const nsIUrlClassifierExceptionListObserver, aList: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierExceptionListObserver {

    /// ```text
    /// /**
    ///    * Called by nsIUrlClassifierExceptionListService when the exception list
    ///    * for a designated feature changes and when the observer is first registered.
    ///    *
    ///    * @param aList
    ///    *        A comma-separated list of url patterns, intended to be parsed
    ///    *        by nsContentUtils::IsURIInList.
    ///    */
    /// ```
    ///

    /// `void onExceptionListUpdate (in ACString aList);`
    #[inline]
    pub unsafe fn OnExceptionListUpdate(&self, aList: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnExceptionListUpdate)(self, aList)
    }


}


/// `interface nsIUrlClassifierExceptionListService : nsISupports`
///

/// ```text
/// /**
///  * A service that monitors updates to the exception list of url-classifier
///  * feature from sources such as a local pref and remote settings updates.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierExceptionListService {
    vtable: *const nsIUrlClassifierExceptionListServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierExceptionListService.
unsafe impl XpCom for nsIUrlClassifierExceptionListService {
    const IID: nsIID = nsID(0x75c3d1a3, 0xe977, 0x4079,
        [0x9e, 0x27, 0xb3, 0xb5, 0x6b, 0xdb, 0x76, 0xea]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierExceptionListService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierExceptionListService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierExceptionListServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierExceptionListService`.
    fn coerce_from(v: &nsIUrlClassifierExceptionListService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierExceptionListServiceCoerce for nsIUrlClassifierExceptionListService {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierExceptionListService) -> &Self {
        v
    }
}

impl nsIUrlClassifierExceptionListService {
    /// Cast this `nsIUrlClassifierExceptionListService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierExceptionListServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierExceptionListService {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierExceptionListServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierExceptionListService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierExceptionListService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierExceptionListServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerAndRunExceptionListObserver (in ACString aFeature, in ACString aPrefName, in nsIUrlClassifierExceptionListObserver aObserver); */
    pub RegisterAndRunExceptionListObserver: unsafe extern "system" fn (this: *const nsIUrlClassifierExceptionListService, aFeature: *const ::nsstring::nsACString, aPrefName: *const ::nsstring::nsACString, aObserver: *const nsIUrlClassifierExceptionListObserver) -> ::nserror::nsresult,

    /* void unregisterExceptionListObserver (in ACString aFeature, in nsIUrlClassifierExceptionListObserver aObserver); */
    pub UnregisterExceptionListObserver: unsafe extern "system" fn (this: *const nsIUrlClassifierExceptionListService, aFeature: *const ::nsstring::nsACString, aObserver: *const nsIUrlClassifierExceptionListObserver) -> ::nserror::nsresult,

    /* void clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsIUrlClassifierExceptionListService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierExceptionListService {

    /// ```text
    /// /**
    ///    * Register a new observer to exception list updates. When the observer is
    ///    * registered it is called immediately once. Afterwards it will be called
    ///    * whenever the specified pref changes or when remote settings for
    ///    * url-classifier features updates.
    ///    *
    ///    * @param aFeature
    ///    *        The feature for which to observe the exception list.
    ///    *
    ///    * @param aPrefName
    ///    *        (Optional) A pref name to monitor. The pref must be of string
    ///    *        type and contain a comma-separated list of URL patterns.
    ///    *
    ///    * @param aObserver
    ///    *        An nsIUrlClassifierExceptionListObserver object or function that
    ///    *        will receive updates to the exception list as a comma-separated
    ///    *        string. Will be called immediately with the current exception
    ///    *        list value.
    ///    */
    /// ```
    ///

    /// `void registerAndRunExceptionListObserver (in ACString aFeature, in ACString aPrefName, in nsIUrlClassifierExceptionListObserver aObserver);`
    #[inline]
    pub unsafe fn RegisterAndRunExceptionListObserver(&self, aFeature: *const ::nsstring::nsACString, aPrefName: *const ::nsstring::nsACString, aObserver: *const nsIUrlClassifierExceptionListObserver) -> ::nserror::nsresult {
        ((*self.vtable).RegisterAndRunExceptionListObserver)(self, aFeature, aPrefName, aObserver)
    }


    /// ```text
    /// /**
    ///    * Unregister an observer.
    ///    *
    ///    * @param aFeature
    ///    *        The feature for which to stop observing.
    ///    *
    ///    * @param aObserver
    ///    *        The nsIUrlClassifierExceptionListObserver object to unregister.
    ///    */
    /// ```
    ///

    /// `void unregisterExceptionListObserver (in ACString aFeature, in nsIUrlClassifierExceptionListObserver aObserver);`
    #[inline]
    pub unsafe fn UnregisterExceptionListObserver(&self, aFeature: *const ::nsstring::nsACString, aObserver: *const nsIUrlClassifierExceptionListObserver) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterExceptionListObserver)(self, aFeature, aObserver)
    }


    /// ```text
    /// /**
    ///    *  Clear all data in the service.
    ///    *  This API is for testing only.
    ///    */
    /// ```
    ///

    /// `void clear ();`
    #[inline]
    pub unsafe fn Clear(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, )
    }


}


