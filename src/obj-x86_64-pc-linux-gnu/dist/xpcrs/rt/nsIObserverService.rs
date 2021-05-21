//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIObserverService.idl
//


/// `interface nsIObserverService : nsISupports`
///

/// ```text
/// /**
///  * nsIObserverService
///  *
///  * Service allows a client listener (nsIObserver) to register and unregister for
///  * notifications of specific string referenced topic. Service also provides a
///  * way to notify registered listeners and a way to enumerate registered client
///  * listeners.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIObserverService {
    vtable: *const nsIObserverServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIObserverService.
unsafe impl XpCom for nsIObserverService {
    const IID: nsIID = nsID(0xd07f5192, 0xe3d1, 0x11d2,
        [0x8a, 0xcd, 0x00, 0x10, 0x5a, 0x1b, 0x88, 0x60]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIObserverService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIObserverService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIObserverServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIObserverService`.
    fn coerce_from(v: &nsIObserverService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIObserverServiceCoerce for nsIObserverService {
    #[inline]
    fn coerce_from(v: &nsIObserverService) -> &Self {
        v
    }
}

impl nsIObserverService {
    /// Cast this `nsIObserverService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIObserverServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIObserverService {
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
impl<T: nsISupportsCoerce> nsIObserverServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObserverService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIObserverService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIObserverServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsIObserverService, anObserver: *const nsIObserver, aTopic: *const libc::c_char, ownsWeak: bool) -> ::nserror::nsresult,

    /* void removeObserver (in nsIObserver anObserver, in string aTopic); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsIObserverService, anObserver: *const nsIObserver, aTopic: *const libc::c_char) -> ::nserror::nsresult,

    /* void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData); */
    pub NotifyObservers: unsafe extern "system" fn (this: *const nsIObserverService, aSubject: *const nsISupports, aTopic: *const libc::c_char, someData: *const i16) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerateObservers (in string aTopic); */
    pub EnumerateObservers: unsafe extern "system" fn (this: *const nsIObserverService, aTopic: *const libc::c_char, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIObserverService {

    /// ```text
    /// /**
    ///      * AddObserver
    ///      *
    ///      * Registers a given listener for a notifications regarding the specified
    ///      * topic.
    ///      *
    ///      * @param anObserve : The interface pointer which will receive notifications.
    ///      * @param aTopic    : The notification topic or subject.
    ///      * @param ownsWeak  : If set to false, the nsIObserverService will hold a
    ///      *                    strong reference to |anObserver|.  If set to true and
    ///      *                    |anObserver| supports the nsIWeakReference interface,
    ///      *                    a weak reference will be held.  Otherwise an error will be
    ///      *                    returned.
    ///      */
    /// ```
    ///

    /// `void addObserver (in nsIObserver anObserver, in string aTopic, [optional] in boolean ownsWeak);`
    #[inline]
    pub unsafe fn AddObserver(&self, anObserver: *const nsIObserver, aTopic: *const libc::c_char, ownsWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, anObserver, aTopic, ownsWeak)
    }


    /// ```text
    /// /**
    ///      * removeObserver
    ///      *
    ///      * Unregisters a given listener from notifications regarding the specified
    ///      * topic.
    ///      *
    ///      * @param anObserver : The interface pointer which will stop recieving
    ///      *                     notifications.
    ///      * @param aTopic     : The notification topic or subject.
    ///      */
    /// ```
    ///

    /// `void removeObserver (in nsIObserver anObserver, in string aTopic);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, anObserver: *const nsIObserver, aTopic: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, anObserver, aTopic)
    }


    /// ```text
    /// /**
    ///      * notifyObservers
    ///      *
    ///      * Notifies all registered listeners of the given topic.
    ///      *
    ///      * @param aSubject : Notification specific interface pointer.
    ///      * @param aTopic   : The notification topic or subject.
    ///      * @param someData : Notification specific wide string.
    ///      */
    /// ```
    ///

    /// `void notifyObservers (in nsISupports aSubject, in string aTopic, [optional] in wstring someData);`
    #[inline]
    pub unsafe fn NotifyObservers(&self, aSubject: *const nsISupports, aTopic: *const libc::c_char, someData: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).NotifyObservers)(self, aSubject, aTopic, someData)
    }


    /// ```text
    /// /**
    ///      * enumerateObservers
    ///      *
    ///      * Returns an enumeration of all registered listeners.
    ///      *
    ///      * @param aTopic   : The notification topic or subject.
    ///      */
    /// ```
    ///

    /// `nsISimpleEnumerator enumerateObservers (in string aTopic);`
    #[inline]
    pub unsafe fn EnumerateObservers(&self, aTopic: *const libc::c_char, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateObservers)(self, aTopic, _retval)
    }


}


