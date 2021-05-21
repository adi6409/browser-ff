//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIPartitioningExceptionListService.idl
//


/// `interface nsIPartitioningExceptionListObserver : nsISupports`
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
pub struct nsIPartitioningExceptionListObserver {
    vtable: *const nsIPartitioningExceptionListObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPartitioningExceptionListObserver.
unsafe impl XpCom for nsIPartitioningExceptionListObserver {
    const IID: nsIID = nsID(0xd8db1086, 0x7b59, 0x44d3,
        [0x9f, 0x88, 0xf3, 0x1a, 0x7e, 0x64, 0x26, 0x37]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPartitioningExceptionListObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPartitioningExceptionListObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPartitioningExceptionListObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIPartitioningExceptionListObserver`.
    fn coerce_from(v: &nsIPartitioningExceptionListObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPartitioningExceptionListObserverCoerce for nsIPartitioningExceptionListObserver {
    #[inline]
    fn coerce_from(v: &nsIPartitioningExceptionListObserver) -> &Self {
        v
    }
}

impl nsIPartitioningExceptionListObserver {
    /// Cast this `nsIPartitioningExceptionListObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPartitioningExceptionListObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPartitioningExceptionListObserver {
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
impl<T: nsISupportsCoerce> nsIPartitioningExceptionListObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPartitioningExceptionListObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPartitioningExceptionListObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPartitioningExceptionListObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onExceptionListUpdate (in ACString aList); */
    pub OnExceptionListUpdate: unsafe extern "system" fn (this: *const nsIPartitioningExceptionListObserver, aList: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPartitioningExceptionListObserver {

    /// ```text
    /// /**
    ///    * Called by nsIPartitioningExceptionListService when the exception list
    ///    * changes and when the observer is first registered.
    ///    *
    ///    * @param aList
    ///    *        A semicolon-separated list of comma-separated url pairs.
    ///    */
    /// ```
    ///

    /// `void onExceptionListUpdate (in ACString aList);`
    #[inline]
    pub unsafe fn OnExceptionListUpdate(&self, aList: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnExceptionListUpdate)(self, aList)
    }


}


/// `interface nsIPartitioningExceptionListService : nsISupports`
///

/// ```text
/// /**
///  * A service that monitors updates to the exception list of partitioning
///  * from sources such as a local pref and remote settings updates.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPartitioningExceptionListService {
    vtable: *const nsIPartitioningExceptionListServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPartitioningExceptionListService.
unsafe impl XpCom for nsIPartitioningExceptionListService {
    const IID: nsIID = nsID(0xcf83a9af, 0xdd3f, 0x43a2,
        [0x88, 0xbb, 0x48, 0x9a, 0x22, 0xbc, 0xa1, 0x24]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPartitioningExceptionListService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPartitioningExceptionListService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPartitioningExceptionListServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPartitioningExceptionListService`.
    fn coerce_from(v: &nsIPartitioningExceptionListService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPartitioningExceptionListServiceCoerce for nsIPartitioningExceptionListService {
    #[inline]
    fn coerce_from(v: &nsIPartitioningExceptionListService) -> &Self {
        v
    }
}

impl nsIPartitioningExceptionListService {
    /// Cast this `nsIPartitioningExceptionListService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPartitioningExceptionListServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPartitioningExceptionListService {
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
impl<T: nsISupportsCoerce> nsIPartitioningExceptionListServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPartitioningExceptionListService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPartitioningExceptionListService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPartitioningExceptionListServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerAndRunExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
    pub RegisterAndRunExceptionListObserver: unsafe extern "system" fn (this: *const nsIPartitioningExceptionListService, aObserver: *const nsIPartitioningExceptionListObserver) -> ::nserror::nsresult,

    /* void unregisterExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
    pub UnregisterExceptionListObserver: unsafe extern "system" fn (this: *const nsIPartitioningExceptionListService, aObserver: *const nsIPartitioningExceptionListObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPartitioningExceptionListService {

    /// ```text
    /// /**
    ///    * Register a new observer to exception list updates. When the observer is
    ///    * registered it is called immediately once. Afterwards it will be called
    ///    * whenever the specified pref changes or when remote settings for
    ///    * partitioning updates.
    ///    *
    ///    * @param aObserver
    ///    *        An nsIPartitioningExceptionListObserver object or function that
    ///    *        will receive updates to the exception list as a comma-separated
    ///    *        string. Will be called immediately with the current exception
    ///    *        list value.
    ///    */
    /// ```
    ///

    /// `void registerAndRunExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver);`
    #[inline]
    pub unsafe fn RegisterAndRunExceptionListObserver(&self, aObserver: *const nsIPartitioningExceptionListObserver) -> ::nserror::nsresult {
        ((*self.vtable).RegisterAndRunExceptionListObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Unregister an observer.
    ///    *
    ///    * @param aObserver
    ///    *        The nsIPartitioningExceptionListObserver object to unregister.
    ///    */
    /// ```
    ///

    /// `void unregisterExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver);`
    #[inline]
    pub unsafe fn UnregisterExceptionListObserver(&self, aObserver: *const nsIPartitioningExceptionListObserver) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterExceptionListObserver)(self, aObserver)
    }


}


