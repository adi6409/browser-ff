//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/console/nsIConsoleAPIStorage.idl
//


/// `interface nsIConsoleAPIStorage : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConsoleAPIStorage {
    vtable: *const nsIConsoleAPIStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConsoleAPIStorage.
unsafe impl XpCom for nsIConsoleAPIStorage {
    const IID: nsIID = nsID(0x9e32a7b6, 0xc4d1, 0x4d9a,
        [0x87, 0xb9, 0x1e, 0xf6, 0xb7, 0x5c, 0x27, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConsoleAPIStorage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConsoleAPIStorage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConsoleAPIStorageCoerce {
    /// Cheaply cast a value of this type from a `nsIConsoleAPIStorage`.
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConsoleAPIStorageCoerce for nsIConsoleAPIStorage {
    #[inline]
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self {
        v
    }
}

impl nsIConsoleAPIStorage {
    /// Cast this `nsIConsoleAPIStorage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConsoleAPIStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConsoleAPIStorage {
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
impl<T: nsISupportsCoerce> nsIConsoleAPIStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleAPIStorage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConsoleAPIStorage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConsoleAPIStorageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* jsval getEvents ([optional] in AString aId); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetEvents: *const ::libc::c_void,

    /* void recordEvent (in AString aId, in AString aOuterId, in jsval aEvent); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RecordEvent: *const ::libc::c_void,

    /* void clearEvents ([optional] in AString aId); */
    pub ClearEvents: unsafe extern "system" fn (this: *const nsIConsoleAPIStorage, aId: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConsoleAPIStorage {

    /// ```text
    /// /**
    ///    * Get the events array by inner window ID or all events from all windows.
    ///    *
    ///    * @param string [aId]
    ///    *        Optional, the inner window ID for which you want to get the array of
    ///    *        cached events.
    ///    * @returns array
    ///    *          The array of cached events for the given window. If no |aId| is
    ///    *          given this function returns all of the cached events, from any
    ///    *          window.
    ///    */
    /// ```
    ///

    /// `jsval getEvents ([optional] in AString aId);`
    const _GetEvents: () = ();

    /// ```text
    /// /**
    ///    * Record an event associated with the given window ID.
    ///    *
    ///    * @param string aId
    ///    *        The ID of the inner window for which the event occurred or "jsm" for
    ///    *        messages logged from JavaScript modules..
    ///    * @param string aOuterId
    ///    *        This ID is used as 3rd parameters for the console-api-log-event
    ///    *        notification.
    ///    * @param object aEvent
    ///    *        A JavaScript object you want to store.
    ///    */
    /// ```
    ///

    /// `void recordEvent (in AString aId, in AString aOuterId, in jsval aEvent);`
    const _RecordEvent: () = ();

    /// ```text
    /// /**
    ///    * Clear storage data for the given window.
    ///    *
    ///    * @param string [aId]
    ///    *        Optional, the inner window ID for which you want to clear the
    ///    *        messages. If this is not specified all of the cached messages are
    ///    *        cleared, from all window objects.
    ///    */
    /// ```
    ///

    /// `void clearEvents ([optional] in AString aId);`
    #[inline]
    pub unsafe fn ClearEvents(&self, aId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ClearEvents)(self, aId)
    }


}


