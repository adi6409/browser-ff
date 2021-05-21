//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/timermanager/nsIUpdateTimerManager.idl
//


/// `interface nsIUpdateTimerManager : nsISupports`
///

/// ```text
/// /**
///  * An interface describing a global application service that allows long
///  * duration (e.g. 1-7 or more days, weeks or months) timers to be registered
///  * and then fired.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUpdateTimerManager {
    vtable: *const nsIUpdateTimerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUpdateTimerManager.
unsafe impl XpCom for nsIUpdateTimerManager {
    const IID: nsIID = nsID(0x0765c92c, 0x6145, 0x4253,
        [0x9d, 0xb4, 0x59, 0x4d, 0x80, 0x23, 0x08, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUpdateTimerManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUpdateTimerManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUpdateTimerManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIUpdateTimerManager`.
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUpdateTimerManagerCoerce for nsIUpdateTimerManager {
    #[inline]
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self {
        v
    }
}

impl nsIUpdateTimerManager {
    /// Cast this `nsIUpdateTimerManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUpdateTimerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUpdateTimerManager {
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
impl<T: nsISupportsCoerce> nsIUpdateTimerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUpdateTimerManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUpdateTimerManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUpdateTimerManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval, [optional] in boolean skipFirst); */
    pub RegisterTimer: unsafe extern "system" fn (this: *const nsIUpdateTimerManager, id: *const ::nsstring::nsAString, callback: *const nsITimerCallback, interval: u32, skipFirst: bool) -> ::nserror::nsresult,

    /* void unregisterTimer (in AString id); */
    pub UnregisterTimer: unsafe extern "system" fn (this: *const nsIUpdateTimerManager, id: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUpdateTimerManager {

    /// ```text
    /// /**
    ///    * Register an interval with the timer manager. The timer manager
    ///    * periodically checks to see if the interval has expired and if it has
    ///    * calls the specified callback. This is persistent across application
    ///    * restarts and can handle intervals of long durations. The callback will be
    ///    * called soon after the first registration unless you ask to skip it.
    ///    * @param   id
    ///    *          An id that identifies the interval, used for persistence
    ///    * @param   callback
    ///    *          A nsITimerCallback object that is notified when the interval
    ///    *          expires
    ///    * @param   interval
    ///    *          The length of time, in seconds, of the interval
    ///    * @param   skipFirst
    ///    *          Whether to skip the initial callback on first registration.
    ///    *
    ///    * Note: to avoid having to instantiate a component to call registerTimer
    ///    * the component can intead register an update-timer category with comma
    ///    * separated values as a single string:
    ///    *
    ///    * contractID,method,id,preference,interval
    ///    *
    ///    * via a manifest entry. The values are as follows:
    ///    *   contractID : the contract ID for the component.
    ///    *   method     : the method used to instantiate the interface. This should be
    ///    *                either getService or createInstance depending on your
    ///    *                component.
    ///    *   id         : the id that identifies the interval, used for persistence.
    ///    *   preference : the preference to for timer interval. This value can be
    ///    *                optional by specifying an empty string for the value.
    ///    *   interval   : the default interval in seconds for the timer.
    ///    */
    /// ```
    ///

    /// `void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval, [optional] in boolean skipFirst);`
    #[inline]
    pub unsafe fn RegisterTimer(&self, id: *const ::nsstring::nsAString, callback: *const nsITimerCallback, interval: u32, skipFirst: bool) -> ::nserror::nsresult {
        ((*self.vtable).RegisterTimer)(self, id, callback, interval, skipFirst)
    }


    /// ```text
    /// /**
    ///    * Unregister an existing interval from the timer manager.
    ///    *
    ///    * @param   id
    ///    *          An id that identifies the interval.
    ///    */
    /// ```
    ///

    /// `void unregisterTimer (in AString id);`
    #[inline]
    pub unsafe fn UnregisterTimer(&self, id: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterTimer)(self, id)
    }


}


