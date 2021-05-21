//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIGleanMetrics.idl
//


/// `interface nsIGleanBoolean : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanBoolean {
    vtable: *const nsIGleanBooleanVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanBoolean.
unsafe impl XpCom for nsIGleanBoolean {
    const IID: nsIID = nsID(0xd3180fe0, 0x19fa, 0x11eb,
        [0x8b, 0x6f, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanBoolean {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanBoolean.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanBooleanCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanBoolean`.
    fn coerce_from(v: &nsIGleanBoolean) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanBooleanCoerce for nsIGleanBoolean {
    #[inline]
    fn coerce_from(v: &nsIGleanBoolean) -> &Self {
        v
    }
}

impl nsIGleanBoolean {
    /// Cast this `nsIGleanBoolean` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanBooleanCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanBoolean {
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
impl<T: nsISupportsCoerce> nsIGleanBooleanCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanBoolean) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanBoolean
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanBooleanVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void set (in bool value); */
    pub Set: unsafe extern "system" fn (this: *const nsIGleanBoolean, value: bool) -> ::nserror::nsresult,

    /* jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanBoolean {

    /// ```text
    /// /**
    ///    * Set to the specified boolean value.
    ///    *
    ///    * @param value the value to set.
    ///    */
    /// ```
    ///

    /// `void set (in bool value);`
    #[inline]
    pub unsafe fn Set(&self, value: bool) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, value)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as a boolean.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanDatetime : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanDatetime {
    vtable: *const nsIGleanDatetimeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanDatetime.
unsafe impl XpCom for nsIGleanDatetime {
    const IID: nsIID = nsID(0xaa15fd20, 0x1e8a, 0x11eb,
        [0x9b, 0xec, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanDatetime {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanDatetime.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanDatetimeCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanDatetime`.
    fn coerce_from(v: &nsIGleanDatetime) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanDatetimeCoerce for nsIGleanDatetime {
    #[inline]
    fn coerce_from(v: &nsIGleanDatetime) -> &Self {
        v
    }
}

impl nsIGleanDatetime {
    /// Cast this `nsIGleanDatetime` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanDatetimeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanDatetime {
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
impl<T: nsISupportsCoerce> nsIGleanDatetimeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanDatetime) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanDatetime
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanDatetimeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [optional_argc] void set ([optional] in PRTime aValue); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub Set: *const ::libc::c_void,

    /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanDatetime {

    /// ```text
    /// /**
    ///    * Set the datetime to the provided value, or the local now.
    ///    *
    ///    * @param aValue The time value in milliseconds since epoch. Defaults to local now.
    ///    */
    /// ```
    ///

    /// `[optional_argc] void set ([optional] in PRTime aValue);`
    const _Set: () = ();

    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as an integer.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanCounter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanCounter {
    vtable: *const nsIGleanCounterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanCounter.
unsafe impl XpCom for nsIGleanCounter {
    const IID: nsIID = nsID(0x05b89d2a, 0xd57c, 0x11ea,
        [0x82, 0xda, 0x3f, 0x63, 0x39, 0x9a, 0x6f, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanCounter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanCounter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanCounterCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanCounter`.
    fn coerce_from(v: &nsIGleanCounter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanCounterCoerce for nsIGleanCounter {
    #[inline]
    fn coerce_from(v: &nsIGleanCounter) -> &Self {
        v
    }
}

impl nsIGleanCounter {
    /// Cast this `nsIGleanCounter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanCounterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanCounter {
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
impl<T: nsISupportsCoerce> nsIGleanCounterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanCounter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanCounter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanCounterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void add (in uint32_t amount); */
    pub Add: unsafe extern "system" fn (this: *const nsIGleanCounter, amount: uint32_t) -> ::nserror::nsresult,

    /* jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanCounter {


    /// `void add (in uint32_t amount);`
    #[inline]
    pub unsafe fn Add(&self, amount: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Add)(self, amount)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as an integer.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanTimingDistribution : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanTimingDistribution {
    vtable: *const nsIGleanTimingDistributionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanTimingDistribution.
unsafe impl XpCom for nsIGleanTimingDistribution {
    const IID: nsIID = nsID(0x92e14730, 0x9b5f, 0x45a1,
        [0xb0, 0x18, 0xf5, 0x88, 0xd0, 0xb9, 0x64, 0xd8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanTimingDistribution {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanTimingDistribution.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanTimingDistributionCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanTimingDistribution`.
    fn coerce_from(v: &nsIGleanTimingDistribution) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanTimingDistributionCoerce for nsIGleanTimingDistribution {
    #[inline]
    fn coerce_from(v: &nsIGleanTimingDistribution) -> &Self {
        v
    }
}

impl nsIGleanTimingDistribution {
    /// Cast this `nsIGleanTimingDistribution` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanTimingDistributionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanTimingDistribution {
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
impl<T: nsISupportsCoerce> nsIGleanTimingDistributionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanTimingDistribution) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanTimingDistribution
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanTimingDistributionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval start (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub Start: *const ::libc::c_void,

    /* void stopAndAccumulate (in uint64_t aId); */
    pub StopAndAccumulate: unsafe extern "system" fn (this: *const nsIGleanTimingDistribution, aId: uint64_t) -> ::nserror::nsresult,

    /* void cancel (in uint64_t aId); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIGleanTimingDistribution, aId: uint64_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanTimingDistribution {

    /// ```text
    /// /**
    ///    * Starts tracking time for the provided metric.
    ///    *
    ///    * @returns A unique timer id for the new timer
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval start ();`
    const _Start: () = ();

    /// ```text
    /// /**
    ///    * Stops tracking time for the provided metric and timer id.
    ///    *
    ///    * Adds a count to the corresponding bucket in the timing distribution.
    ///    * This will record an error if no `start` was called for this TimerId or
    ///    * if this TimerId was used to call `cancel`.
    ///    *
    ///    * @param aId The TimerId associated with this timing. This allows for
    ///    *            concurrent timing of events associated with different ids.
    ///    */
    /// ```
    ///

    /// `void stopAndAccumulate (in uint64_t aId);`
    #[inline]
    pub unsafe fn StopAndAccumulate(&self, aId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).StopAndAccumulate)(self, aId)
    }


    /// ```text
    /// /**
    ///    * Aborts a previous `start` call. No error is recorded if no `start` was
    ///    * called. (But then where did you get that id from?)
    ///    *
    ///    * @param aId The TimerID whose `start` you wish to abort.
    ///    */
    /// ```
    ///

    /// `void cancel (in uint64_t aId);`
    #[inline]
    pub unsafe fn Cancel(&self, aId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, aId)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as a DistributionData.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or Nothing() if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanMemoryDistribution : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanMemoryDistribution {
    vtable: *const nsIGleanMemoryDistributionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanMemoryDistribution.
unsafe impl XpCom for nsIGleanMemoryDistribution {
    const IID: nsIID = nsID(0xeea5ed46, 0x16ba, 0x46cd,
        [0xbb, 0x1f, 0x50, 0x45, 0x81, 0x98, 0x7f, 0xe1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanMemoryDistribution {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanMemoryDistribution.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanMemoryDistributionCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanMemoryDistribution`.
    fn coerce_from(v: &nsIGleanMemoryDistribution) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanMemoryDistributionCoerce for nsIGleanMemoryDistribution {
    #[inline]
    fn coerce_from(v: &nsIGleanMemoryDistribution) -> &Self {
        v
    }
}

impl nsIGleanMemoryDistribution {
    /// Cast this `nsIGleanMemoryDistribution` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanMemoryDistributionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanMemoryDistribution {
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
impl<T: nsISupportsCoerce> nsIGleanMemoryDistributionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanMemoryDistribution) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanMemoryDistribution
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanMemoryDistributionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void accumulate (in uint64_t aSample); */
    pub Accumulate: unsafe extern "system" fn (this: *const nsIGleanMemoryDistribution, aSample: uint64_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanMemoryDistribution {


    /// `void accumulate (in uint64_t aSample);`
    #[inline]
    pub unsafe fn Accumulate(&self, aSample: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).Accumulate)(self, aSample)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as a DistributionData.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or Nothing() if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in ACString aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanPing : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanPing {
    vtable: *const nsIGleanPingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanPing.
unsafe impl XpCom for nsIGleanPing {
    const IID: nsIID = nsID(0x5223a48b, 0x687d, 0x47ff,
        [0xa6, 0x29, 0xfd, 0x4a, 0x72, 0xd1, 0xec, 0xfa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanPing {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanPing.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanPingCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanPing`.
    fn coerce_from(v: &nsIGleanPing) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanPingCoerce for nsIGleanPing {
    #[inline]
    fn coerce_from(v: &nsIGleanPing) -> &Self {
        v
    }
}

impl nsIGleanPing {
    /// Cast this `nsIGleanPing` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanPingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanPing {
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
impl<T: nsISupportsCoerce> nsIGleanPingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanPing) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanPing
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanPingVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void submit ([optional] in ACString aReason); */
    pub Submit: unsafe extern "system" fn (this: *const nsIGleanPing, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanPing {

    /// ```text
    /// /**
    ///    * Collect and submit the ping for eventual upload.
    ///    *
    ///    * This will collect all stored data to be included in the ping.
    ///    * Data with lifetime `ping` will then be reset.
    ///    *
    ///    * If the ping is configured with `send_if_empty = false`
    ///    * and the ping currently contains no content,
    ///    * it will not be queued for upload.
    ///    * If the ping is configured with `send_if_empty = true`
    ///    * it will be queued for upload even if empty.
    ///    *
    ///    * Pings always contain the `ping_info` and `client_info` sections.
    ///    * See [ping sections](https://mozilla.github.io/glean/book/user/pings/index.html#ping-sections)
    ///    * for details.
    ///    *
    ///    * @param aReason - Optional. The reason the ping is being submitted.
    ///    *                  Must match one of the configured `reason_codes`.
    ///    */
    /// ```
    ///

    /// `void submit ([optional] in ACString aReason);`
    #[inline]
    pub unsafe fn Submit(&self, aReason: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Submit)(self, aReason)
    }


}


/// `interface nsIGleanString : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanString {
    vtable: *const nsIGleanStringVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanString.
unsafe impl XpCom for nsIGleanString {
    const IID: nsIID = nsID(0xd84a3555, 0x46f1, 0x48c1,
        [0x91, 0x22, 0xe8, 0xe8, 0x8b, 0x06, 0x9d, 0x2b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanString {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanString.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanStringCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanString`.
    fn coerce_from(v: &nsIGleanString) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanStringCoerce for nsIGleanString {
    #[inline]
    fn coerce_from(v: &nsIGleanString) -> &Self {
        v
    }
}

impl nsIGleanString {
    /// Cast this `nsIGleanString` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanStringCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanString {
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
impl<T: nsISupportsCoerce> nsIGleanStringCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanString) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanString
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanStringVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void set (in AUTF8String value); */
    pub Set: unsafe extern "system" fn (this: *const nsIGleanString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanString {


    /// `void set (in AUTF8String value);`
    #[inline]
    pub unsafe fn Set(&self, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, value)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as a string.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanStringList : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanStringList {
    vtable: *const nsIGleanStringListVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanStringList.
unsafe impl XpCom for nsIGleanStringList {
    const IID: nsIID = nsID(0x46751205, 0x2ac7, 0x47dc,
        [0x91, 0xd2, 0xef, 0x4a, 0x95, 0xef, 0x2a, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanStringList {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanStringList.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanStringListCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanStringList`.
    fn coerce_from(v: &nsIGleanStringList) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanStringListCoerce for nsIGleanStringList {
    #[inline]
    fn coerce_from(v: &nsIGleanStringList) -> &Self {
        v
    }
}

impl nsIGleanStringList {
    /// Cast this `nsIGleanStringList` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanStringListCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanStringList {
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
impl<T: nsISupportsCoerce> nsIGleanStringListCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanStringList) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanStringList
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanStringListVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void add (in AUTF8String value); */
    pub Add: unsafe extern "system" fn (this: *const nsIGleanStringList, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void set (in Array<AUTF8String> value); */
    pub Set: unsafe extern "system" fn (this: *const nsIGleanStringList, value: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanStringList {

    /// ```text
    /// /**
    ///    * Adds a new string to the list.
    ///    *
    ///    * Truncates the value and logs an error if it is longer than 50 bytes.
    ///    *
    ///    * @param value The string to add.
    ///    */
    /// ```
    ///

    /// `void add (in AUTF8String value);`
    #[inline]
    pub unsafe fn Add(&self, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Add)(self, value)
    }


    /// ```text
    /// /**
    ///    * Sets to a specific list of strings.
    ///    *
    ///    * Truncates the list and logs an error if longer than 20 items.
    ///    * Truncates any item longer than 50 bytes and logs an error.
    ///    *
    ///    * @param value The list of strings to set.
    ///    */
    /// ```
    ///

    /// `void set (in Array<AUTF8String> value);`
    #[inline]
    pub unsafe fn Set(&self, value: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, value)
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanTimespan : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanTimespan {
    vtable: *const nsIGleanTimespanVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanTimespan.
unsafe impl XpCom for nsIGleanTimespan {
    const IID: nsIID = nsID(0x2586530c, 0x030f, 0x11eb,
        [0x93, 0xcb, 0xcb, 0xf3, 0x0d, 0x25, 0x22, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanTimespan {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanTimespan.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanTimespanCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanTimespan`.
    fn coerce_from(v: &nsIGleanTimespan) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanTimespanCoerce for nsIGleanTimespan {
    #[inline]
    fn coerce_from(v: &nsIGleanTimespan) -> &Self {
        v
    }
}

impl nsIGleanTimespan {
    /// Cast this `nsIGleanTimespan` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanTimespanCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanTimespan {
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
impl<T: nsISupportsCoerce> nsIGleanTimespanCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanTimespan) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanTimespan
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanTimespanVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void start (); */
    pub Start: unsafe extern "system" fn (this: *const nsIGleanTimespan) -> ::nserror::nsresult,

    /* void stop (); */
    pub Stop: unsafe extern "system" fn (this: *const nsIGleanTimespan) -> ::nserror::nsresult,

    /* jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanTimespan {

    /// ```text
    /// /**
    ///    * Start tracking time for the provided metric.
    ///    *
    ///    * This records an error if it’s already tracking time (i.e. start was already
        ///    * called with no corresponding [stop]): in that case the original
    ///    * start time will be preserved.
    ///    */
    /// ```
    ///

    /// `void start ();`
    #[inline]
    pub unsafe fn Start(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Start)(self, )
    }


    /// ```text
    /// /**
    ///    * Stop tracking time for the provided metric.
    ///    *
    ///    * Sets the metric to the elapsed time, but does not overwrite an already
    ///    * existing value.
    ///    * This will record an error if no [start] was called or there is an already
    ///    * existing value.
    ///    */
    /// ```
    ///

    /// `void stop ();`
    #[inline]
    pub unsafe fn Stop(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, )
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as an integer.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanUuid : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanUuid {
    vtable: *const nsIGleanUuidVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanUuid.
unsafe impl XpCom for nsIGleanUuid {
    const IID: nsIID = nsID(0x395700e7, 0x06f6, 0x46be,
        [0xad, 0xcc, 0xea, 0x58, 0x97, 0x7f, 0xda, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanUuid {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanUuid.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanUuidCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanUuid`.
    fn coerce_from(v: &nsIGleanUuid) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanUuidCoerce for nsIGleanUuid {
    #[inline]
    fn coerce_from(v: &nsIGleanUuid) -> &Self {
        v
    }
}

impl nsIGleanUuid {
    /// Cast this `nsIGleanUuid` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanUuidCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanUuid {
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
impl<T: nsISupportsCoerce> nsIGleanUuidCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanUuid) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanUuid
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanUuidVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void set (in AUTF8String aValue); */
    pub Set: unsafe extern "system" fn (this: *const nsIGleanUuid, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void generateAndSet (); */
    pub GenerateAndSet: unsafe extern "system" fn (this: *const nsIGleanUuid) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanUuid {

    /// ```text
    /// /**
    ///    * Set to the specified value.
    ///    *
    ///    * @param aValue The UUID to set the metric to.
    ///    */
    /// ```
    ///

    /// `void set (in AUTF8String aValue);`
    #[inline]
    pub unsafe fn Set(&self, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, aValue)
    }


    /// ```text
    /// /**
    ///    * Generate a new random UUID and set the metric to it.
    ///    */
    /// ```
    ///

    /// `void generateAndSet ();`
    #[inline]
    pub unsafe fn GenerateAndSet(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).GenerateAndSet)(self, )
    }


    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Gets the currently stored value as an integer.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric, or undefined if there is no value.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


/// `interface nsIGleanEvent : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGleanEvent {
    vtable: *const nsIGleanEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGleanEvent.
unsafe impl XpCom for nsIGleanEvent {
    const IID: nsIID = nsID(0x1b01424a, 0x1f55, 0x11eb,
        [0x92, 0xa5, 0x07, 0x54, 0xf6, 0xc3, 0xf2, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGleanEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGleanEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGleanEventCoerce {
    /// Cheaply cast a value of this type from a `nsIGleanEvent`.
    fn coerce_from(v: &nsIGleanEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGleanEventCoerce for nsIGleanEvent {
    #[inline]
    fn coerce_from(v: &nsIGleanEvent) -> &Self {
        v
    }
}

impl nsIGleanEvent {
    /// Cast this `nsIGleanEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGleanEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGleanEvent {
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
impl<T: nsISupportsCoerce> nsIGleanEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGleanEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGleanEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGleanEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void record ([optional] in jsval aExtra); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Record: *const ::libc::c_void,

    /* [implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub TestGetValue: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGleanEvent {


    /// `[implicit_jscontext] void record ([optional] in jsval aExtra);`
    const _Record: () = ();

    /// ```text
    /// /**
    ///    * **Test-only API**
    ///    *
    ///    * Get a list of currently stored events for this event metric.
    ///    *
    ///    * This function will attempt to await the last parent-process task (if any)
    ///    * writing to the the metric's storage engine before returning a value.
    ///    * This function will not wait for data from child processes.
    ///    *
    ///    * This doesn't clear the stored value.
    ///    * Parent process only. Panics in child processes.
    ///    *
    ///    * @param aPingName The (optional) name of the ping to retrieve the metric
    ///    *        for. Defaults to the first value in `send_in_pings`.
    ///    *
    ///    * @return value of the stored metric.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval testGetValue ([optional] in AUTF8String aPingName);`
    const _TestGetValue: () = ();

}


