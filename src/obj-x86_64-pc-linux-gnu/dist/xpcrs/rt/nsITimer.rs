//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsITimer.idl
//


/// `interface nsITimerCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITimerCallback {
    vtable: *const nsITimerCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITimerCallback.
unsafe impl XpCom for nsITimerCallback {
    const IID: nsIID = nsID(0xa796816d, 0x7d47, 0x4348,
        [0x9a, 0xb8, 0xc7, 0xae, 0xb3, 0x21, 0x6a, 0x7d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITimerCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITimerCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITimerCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsITimerCallback`.
    fn coerce_from(v: &nsITimerCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITimerCallbackCoerce for nsITimerCallback {
    #[inline]
    fn coerce_from(v: &nsITimerCallback) -> &Self {
        v
    }
}

impl nsITimerCallback {
    /// Cast this `nsITimerCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITimerCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITimerCallback {
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
impl<T: nsISupportsCoerce> nsITimerCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimerCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITimerCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITimerCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notify (in nsITimer timer); */
    pub Notify: unsafe extern "system" fn (this: *const nsITimerCallback, timer: *const nsITimer) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITimerCallback {

    /// ```text
    /// /**
    ///    * @param aTimer the timer which has expired
    ///    */
    /// ```
    ///

    /// `void notify (in nsITimer timer);`
    #[inline]
    pub unsafe fn Notify(&self, timer: *const nsITimer) -> ::nserror::nsresult {
        ((*self.vtable).Notify)(self, timer)
    }


}


/// `interface nsITimer : nsISupports`
///

/// ```text
/// /**
///  * The callback interface for timers.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITimer {
    vtable: *const nsITimerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITimer.
unsafe impl XpCom for nsITimer {
    const IID: nsIID = nsID(0x3de4b105, 0x363c, 0x482c,
        [0xa4, 0x09, 0xba, 0xac, 0x83, 0xa0, 0x1b, 0xfc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITimer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITimer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITimerCoerce {
    /// Cheaply cast a value of this type from a `nsITimer`.
    fn coerce_from(v: &nsITimer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITimerCoerce for nsITimer {
    #[inline]
    fn coerce_from(v: &nsITimer) -> &Self {
        v
    }
}

impl nsITimer {
    /// Cast this `nsITimer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITimerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITimer {
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
impl<T: nsISupportsCoerce> nsITimerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITimer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITimerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsIObserver aObserver, in unsigned long aDelayInMs, in unsigned long aType); */
    pub Init: unsafe extern "system" fn (this: *const nsITimer, aObserver: *const nsIObserver, aDelayInMs: u32, aType: u32) -> ::nserror::nsresult,

    /* void initWithCallback (in nsITimerCallback aCallback, in unsigned long aDelayInMs, in unsigned long aType); */
    pub InitWithCallback: unsafe extern "system" fn (this: *const nsITimer, aCallback: *const nsITimerCallback, aDelayInMs: u32, aType: u32) -> ::nserror::nsresult,

    /* [noscript] void InitHighResolutionWithCallback (in nsITimerCallback aCallback, [const] in TimeDuration aDelay, in unsigned long aType); */
    /// Unable to generate binding because `native type mozilla::TimeDuration unsupported`
    pub InitHighResolutionWithCallback: *const ::libc::c_void,

    /* void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsITimer) -> ::nserror::nsresult,

    /* [noscript] void initWithNamedFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in string aName); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub InitWithNamedFuncCallback: *const ::libc::c_void,

    /* [noscript] void initWithNameableFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in nsTimerNameCallbackFunc aNameCallback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub InitWithNameableFuncCallback: *const ::libc::c_void,

    /* attribute unsigned long delay; */
    pub GetDelay: unsafe extern "system" fn (this: *const nsITimer, aDelay: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long delay; */
    pub SetDelay: unsafe extern "system" fn (this: *const nsITimer, aDelay: u32) -> ::nserror::nsresult,

    /* attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsITimer, aType: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long type; */
    pub SetType: unsafe extern "system" fn (this: *const nsITimer, aType: u32) -> ::nserror::nsresult,

    /* [noscript] readonly attribute voidPtr closure; */
    pub GetClosure: unsafe extern "system" fn (this: *const nsITimer, aClosure: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsITimerCallback callback; */
    pub GetCallback: unsafe extern "system" fn (this: *const nsITimer, aCallback: *mut *const nsITimerCallback) -> ::nserror::nsresult,

    /* attribute nsIEventTarget target; */
    pub GetTarget: unsafe extern "system" fn (this: *const nsITimer, aTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,

    /* attribute nsIEventTarget target; */
    pub SetTarget: unsafe extern "system" fn (this: *const nsITimer, aTarget: *const nsIEventTarget) -> ::nserror::nsresult,

    /* [noscript] readonly attribute unsigned long allowedEarlyFiringMicroseconds; */
    pub GetAllowedEarlyFiringMicroseconds: unsafe extern "system" fn (this: *const nsITimer, aAllowedEarlyFiringMicroseconds: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITimer {
    /// ```text
    /// /**
    ///  * nsITimer instances must be initialized by calling one of the "init" methods
    ///  * documented below.  You may also re-initialize (using one of the init()
        ///  * methods) an existing instance to avoid the overhead of destroying and
    ///  * creating a timer.  It is not necessary to cancel the timer in that case.
    ///  *
    ///  * By default a timer will fire on the thread that created it.  Set the .target
    ///  * attribute to fire on a different thread.  Once you have set a timer's .target
    ///  * and called one of its init functions, any further interactions with the timer
    ///  * (calling cancel(), changing member fields, etc) should only be done by the
    ///  * target thread, or races may occur with bad results like timers firing after
    ///  * they've been canceled, and/or not firing after re-initiatization.
    ///  */
    /// /**
    ///    * Type of a timer that fires once only.
    ///    */
    /// ```
    ///

    pub const TYPE_ONE_SHOT: i64 = 0;

    /// ```text
    /// /**
    ///    * After firing, a TYPE_REPEATING_SLACK timer is stopped and not restarted
    ///    * until its callback completes.  Specified timer period will be at least
    ///    * the time between when processing for last firing the callback completes
    ///    * and when the next firing occurs.
    ///    *
    ///    * This is the preferable repeating type for most situations.
    ///    */
    /// ```
    ///

    pub const TYPE_REPEATING_SLACK: i64 = 1;

    /// ```text
    /// /**
    ///    * TYPE_REPEATING_PRECISE is just a synonym for
    ///    * TYPE_REPEATING_PRECISE_CAN_SKIP. They used to be distinct, but the old
    ///    * TYPE_REPEATING_PRECISE kind was similar to TYPE_REPEATING_PRECISE_CAN_SKIP
    ///    * while also being less useful. So the distinction was removed.
    ///    */
    /// ```
    ///

    pub const TYPE_REPEATING_PRECISE: i64 = 2;

    /// ```text
    /// /**
    ///    * A TYPE_REPEATING_PRECISE_CAN_SKIP repeating timer aims to have constant
    ///    * period between firings.  The processing time for each timer callback
    ///    * should not influence the timer period.  However this timer type
    ///    * guarantees that it will not queue up new events to fire the callback
    ///    * until the previous callback event finishes firing.  If the callback
    ///    * takes a long time, then the next callback will be scheduled immediately
    ///    * afterward, but only once.  This is the only non-slack timer available.
    ///    */
    /// ```
    ///

    pub const TYPE_REPEATING_PRECISE_CAN_SKIP: i64 = 3;

    /// ```text
    /// /**
    ///    * Same as TYPE_REPEATING_SLACK with the exception that idle events
    ///    * won't yield to timers with this type.  Use this when you want an
    ///    * idle callback to be scheduled to run even though this timer is
    ///    * about to fire.
    ///    */
    /// ```
    ///

    pub const TYPE_REPEATING_SLACK_LOW_PRIORITY: i64 = 4;

    /// ```text
    /// /**
    ///    * Same as TYPE_ONE_SHOT with the exception that idle events won't
    ///    * yield to timers with this type.  Use this when you want an idle
    ///    * callback to be scheduled to run even though this timer is about
    ///    * to fire.
    ///    */
    /// ```
    ///

    pub const TYPE_ONE_SHOT_LOW_PRIORITY: i64 = 5;

    /// ```text
    /// /**
    ///    * Initialize a timer that will fire after the said delay.
    ///    * A user must keep a reference to this timer till it is
    ///    * is no longer needed or has been cancelled.
    ///    *
    ///    * @param aObserver   the callback object that observes the
    ///    *                    ``timer-callback'' topic with the subject being
    ///    *                    the timer itself when the timer fires:
    ///    *
    ///    *                    observe(nsISupports aSubject, => nsITimer
        ///    *                            string aTopic,        => ``timer-callback''
        ///    *                            wstring data          =>  null
        ///    *
        ///    * @param aDelayInMs  delay in milliseconds for timer to fire
        ///    * @param aType       timer type per TYPE* consts defined above
        ///    */
        /// ```
        ///

        /// `void init (in nsIObserver aObserver, in unsigned long aDelayInMs, in unsigned long aType);`
        #[inline]
        pub unsafe fn Init(&self, aObserver: *const nsIObserver, aDelayInMs: u32, aType: u32) -> ::nserror::nsresult {
            ((*self.vtable).Init)(self, aObserver, aDelayInMs, aType)
        }


        /// ```text
        /// /**
        ///    * Initialize a timer to fire after the given millisecond interval.
        ///    * This version takes a callback object.
        ///    *
        ///    * @param aFunc       nsITimerCallback interface to call when timer expires
        ///    * @param aDelayInMs  The millisecond interval
        ///    * @param aType       Timer type per TYPE* consts defined above
        ///    */
        /// ```
        ///

        /// `void initWithCallback (in nsITimerCallback aCallback, in unsigned long aDelayInMs, in unsigned long aType);`
        #[inline]
        pub unsafe fn InitWithCallback(&self, aCallback: *const nsITimerCallback, aDelayInMs: u32, aType: u32) -> ::nserror::nsresult {
            ((*self.vtable).InitWithCallback)(self, aCallback, aDelayInMs, aType)
        }


        /// ```text
        /// /**
        ///    * Initialize a timer to fire after the high resolution TimeDuration.
        ///    * This version takes a callback object.
        ///    *
        ///    * @param aFunc      nsITimerCallback interface to call when timer expires
        ///    * @param aDelay     The high resolution interval
        ///    * @param aType      Timer type per TYPE* consts defined above
        ///    */
        /// ```
        ///

        /// `[noscript] void InitHighResolutionWithCallback (in nsITimerCallback aCallback, [const] in TimeDuration aDelay, in unsigned long aType);`
        const _InitHighResolutionWithCallback: () = ();

        /// ```text
        /// /**
        ///    * Cancel the timer.  This method works on all types, not just on repeating
        ///    * timers -- you might want to cancel a TYPE_ONE_SHOT timer, and even reuse
        ///    * it by re-initializing it (to avoid object destruction and creation costs
            ///    * by conserving one timer instance).
        ///    */
        /// ```
        ///

        /// `void cancel ();`
        #[inline]
        pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).Cancel)(self, )
        }


        /// ```text
        /// /**
        ///    * Like initWithFuncCallback, but also takes a name for the timer; the name
        ///    * will be used when timer profiling is enabled via the "TimerFirings" log
        ///    * module.
        ///    *
        ///    * @param aFunc      The function to invoke
        ///    * @param aClosure   An opaque pointer to pass to that function
        ///    * @param aDelay     The millisecond interval
        ///    * @param aType      Timer type per TYPE* consts defined above
        ///    * @param aName      The timer's name
        ///    */
        /// ```
        ///

        /// `[noscript] void initWithNamedFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in string aName);`
        const _InitWithNamedFuncCallback: () = ();

        /// ```text
        /// /**
        ///    * Like initWithNamedFuncCallback, but instead of a timer name it takes a
        ///    * callback that will provide a name when the timer fires.
        ///    *
        ///    * @param aFunc      The function to invoke
        ///    * @param aClosure   An opaque pointer to pass to that function
        ///    * @param aDelay     The millisecond interval
        ///    * @param aType      Timer type per TYPE* consts defined above
        ///    * @param aNameCallback  The callback function
        ///    */
        /// ```
        ///

        /// `[noscript] void initWithNameableFuncCallback (in nsTimerCallbackFunc aCallback, in voidPtr aClosure, in unsigned long aDelay, in unsigned long aType, in nsTimerNameCallbackFunc aNameCallback);`
        const _InitWithNameableFuncCallback: () = ();

        /// ```text
        /// /**
        ///    * The millisecond delay of the timeout.
        ///    *
        ///    * NOTE: Re-setting the delay on a one-shot timer that has already fired
        ///    * doesn't restart the timer. Call one of the init() methods to restart
        ///    * a one-shot timer.
        ///    */
        /// ```
        ///

        /// `attribute unsigned long delay;`
        #[inline]
        pub unsafe fn GetDelay(&self, aDelay: *mut u32) -> ::nserror::nsresult {
            ((*self.vtable).GetDelay)(self, aDelay)
        }


        /// ```text
        /// /**
        ///    * The millisecond delay of the timeout.
        ///    *
        ///    * NOTE: Re-setting the delay on a one-shot timer that has already fired
        ///    * doesn't restart the timer. Call one of the init() methods to restart
        ///    * a one-shot timer.
        ///    */
        /// ```
        ///

        /// `attribute unsigned long delay;`
        #[inline]
        pub unsafe fn SetDelay(&self, aDelay: u32) -> ::nserror::nsresult {
            ((*self.vtable).SetDelay)(self, aDelay)
        }


        /// ```text
        /// /**
        ///    * The timer type - one of the above TYPE_* constants.
        ///    */
        /// ```
        ///

        /// `attribute unsigned long type;`
        #[inline]
        pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
            ((*self.vtable).GetType)(self, aType)
        }


        /// ```text
        /// /**
        ///    * The timer type - one of the above TYPE_* constants.
        ///    */
        /// ```
        ///

        /// `attribute unsigned long type;`
        #[inline]
        pub unsafe fn SetType(&self, aType: u32) -> ::nserror::nsresult {
            ((*self.vtable).SetType)(self, aType)
        }


        /// ```text
        /// /**
        ///    * The opaque pointer pass to initWithFuncCallback.
        ///    */
        /// ```
        ///

        /// `[noscript] readonly attribute voidPtr closure;`
        #[inline]
        pub unsafe fn GetClosure(&self, aClosure: *mut *mut libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetClosure)(self, aClosure)
        }


        /// ```text
        /// /**
        ///    * The nsITimerCallback object passed to initWithCallback.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsITimerCallback callback;`
        #[inline]
        pub unsafe fn GetCallback(&self, aCallback: *mut *const nsITimerCallback) -> ::nserror::nsresult {
            ((*self.vtable).GetCallback)(self, aCallback)
        }


        /// ```text
        /// /**
        ///    * The nsIEventTarget where the callback will be dispatched. Note that this
        ///    * target may only be set before the call to one of the init methods above.
        ///    *
        ///    * By default the target is the thread that created the timer.
        ///    */
        /// ```
        ///

        /// `attribute nsIEventTarget target;`
        #[inline]
        pub unsafe fn GetTarget(&self, aTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
            ((*self.vtable).GetTarget)(self, aTarget)
        }


        /// ```text
        /// /**
        ///    * The nsIEventTarget where the callback will be dispatched. Note that this
        ///    * target may only be set before the call to one of the init methods above.
        ///    *
        ///    * By default the target is the thread that created the timer.
        ///    */
        /// ```
        ///

        /// `attribute nsIEventTarget target;`
        #[inline]
        pub unsafe fn SetTarget(&self, aTarget: *const nsIEventTarget) -> ::nserror::nsresult {
            ((*self.vtable).SetTarget)(self, aTarget)
        }


        /// ```text
        /// /**
        ///    * The number of microseconds this nsITimer implementation can possibly
        ///    * fire early.
        ///    */
        /// ```
        ///

        /// `[noscript] readonly attribute unsigned long allowedEarlyFiringMicroseconds;`
        #[inline]
        pub unsafe fn GetAllowedEarlyFiringMicroseconds(&self, aAllowedEarlyFiringMicroseconds: *mut u32) -> ::nserror::nsresult {
            ((*self.vtable).GetAllowedEarlyFiringMicroseconds)(self, aAllowedEarlyFiringMicroseconds)
        }


    }


