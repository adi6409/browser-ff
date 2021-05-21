//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIUserIdleService.idl
//


/// `interface nsIUserIdleService : nsISupports`
///

/// ```text
/// /**
///  * This interface lets you monitor how long the user has been 'idle',
///  * i.e. not used their mouse or keyboard. You can get the idle time directly,
///  * but in most cases you will want to register an observer for a predefined
///  * interval. The observer will get an 'idle' notification when the user is idle
///  * for that interval (or longer), and receive an 'active' notification when the
///  * user starts using their computer again.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUserIdleService {
    vtable: *const nsIUserIdleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUserIdleService.
unsafe impl XpCom for nsIUserIdleService {
    const IID: nsIID = nsID(0xcc52f19a, 0x63ae, 0x4a1c,
        [0x9c, 0xc3, 0xe7, 0x9e, 0xac, 0xe0, 0xb4, 0x71]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUserIdleService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUserIdleService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUserIdleServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIUserIdleService`.
    fn coerce_from(v: &nsIUserIdleService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUserIdleServiceCoerce for nsIUserIdleService {
    #[inline]
    fn coerce_from(v: &nsIUserIdleService) -> &Self {
        v
    }
}

impl nsIUserIdleService {
    /// Cast this `nsIUserIdleService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUserIdleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUserIdleService {
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
impl<T: nsISupportsCoerce> nsIUserIdleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUserIdleService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUserIdleService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUserIdleServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long idleTime; */
    pub GetIdleTime: unsafe extern "system" fn (this: *const nsIUserIdleService, aIdleTime: *mut u32) -> ::nserror::nsresult,

    /* void addIdleObserver (in nsIObserver observer, in unsigned long time); */
    pub AddIdleObserver: unsafe extern "system" fn (this: *const nsIUserIdleService, observer: *const nsIObserver, time: u32) -> ::nserror::nsresult,

    /* void removeIdleObserver (in nsIObserver observer, in unsigned long time); */
    pub RemoveIdleObserver: unsafe extern "system" fn (this: *const nsIUserIdleService, observer: *const nsIObserver, time: u32) -> ::nserror::nsresult,

    /* attribute boolean disabled; */
    pub GetDisabled: unsafe extern "system" fn (this: *const nsIUserIdleService, aDisabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean disabled; */
    pub SetDisabled: unsafe extern "system" fn (this: *const nsIUserIdleService, aDisabled: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUserIdleService {

    /// ```text
    /// /**
    ///      * The amount of time in milliseconds that has passed
    ///      * since the last user activity.
    ///      *
    ///      * If we do not have a valid idle time to report, 0 is returned
    ///      * (this can happen if the user never interacted with the browser
        ///      * at all, and if we are also unable to poll for idle time manually).
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long idleTime;`
    #[inline]
    pub unsafe fn GetIdleTime(&self, aIdleTime: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetIdleTime)(self, aIdleTime)
    }


    /// ```text
    /// /**
    ///      * Add an observer to be notified when the user idles for some period of
    ///      * time, and when they get back from that.
    ///      *
    ///      * @param observer the observer to be notified
    ///      * @param time the amount of time in seconds the user should be idle before
    ///      *             the observer should be notified.
    ///      *
    ///      * @note
    ///      * The subject of the notification the observer will get is always the
    ///      * nsIUserIdleService itself.
    ///      * When the user goes idle, the observer topic is "idle" and when he gets
    ///      * back, the observer topic is "active".
    ///      * The data param for the notification contains the current user idle time.
    ///      *
    ///      * @note
    ///      * You can add the same observer twice.
    ///      * @note
    ///      * Most implementations need to poll the OS for idle info themselves,
    ///      * meaning your notifications could arrive with a delay up to the length
    ///      * of the polling interval in that implementation.
    ///      * Current implementations use a delay of 5 seconds.
    ///      */
    /// ```
    ///

    /// `void addIdleObserver (in nsIObserver observer, in unsigned long time);`
    #[inline]
    pub unsafe fn AddIdleObserver(&self, observer: *const nsIObserver, time: u32) -> ::nserror::nsresult {
        ((*self.vtable).AddIdleObserver)(self, observer, time)
    }


    /// ```text
    /// /**
    ///      * Remove an observer registered with addIdleObserver.
    ///      * @param observer the observer that needs to be removed.
    ///      * @param time the amount of time they were listening for.
    ///      * @note
    ///      * Removing an observer will remove it once, for the idle time you specify.
    ///      * If you have added an observer multiple times, you will need to remove it
    ///      * just as many times.
    ///      */
    /// ```
    ///

    /// `void removeIdleObserver (in nsIObserver observer, in unsigned long time);`
    #[inline]
    pub unsafe fn RemoveIdleObserver(&self, observer: *const nsIObserver, time: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveIdleObserver)(self, observer, time)
    }


    /// ```text
    /// /**
    ///      * If true, the idle service is temporarily disabled, and all idle events
    ///      * will be ignored.
    ///      *
    ///      * This should only be used in automation.
    ///      */
    /// ```
    ///

    /// `attribute boolean disabled;`
    #[inline]
    pub unsafe fn GetDisabled(&self, aDisabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDisabled)(self, aDisabled)
    }


    /// ```text
    /// /**
    ///      * If true, the idle service is temporarily disabled, and all idle events
    ///      * will be ignored.
    ///      *
    ///      * This should only be used in automation.
    ///      */
    /// ```
    ///

    /// `attribute boolean disabled;`
    #[inline]
    pub unsafe fn SetDisabled(&self, aDisabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDisabled)(self, aDisabled)
    }


}


