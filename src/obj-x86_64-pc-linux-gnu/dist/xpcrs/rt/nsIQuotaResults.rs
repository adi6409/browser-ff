//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaResults.idl
//


/// `interface nsIQuotaUsageResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaUsageResult {
    vtable: *const nsIQuotaUsageResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaUsageResult.
unsafe impl XpCom for nsIQuotaUsageResult {
    const IID: nsIID = nsID(0xd8c9328b, 0x9aa8, 0x4f5d,
        [0x90, 0xe6, 0x48, 0x2d, 0xe4, 0xa6, 0xd5, 0xb8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaUsageResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaUsageResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaUsageResultCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaUsageResult`.
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaUsageResultCoerce for nsIQuotaUsageResult {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self {
        v
    }
}

impl nsIQuotaUsageResult {
    /// Cast this `nsIQuotaUsageResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaUsageResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaUsageResult {
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
impl<T: nsISupportsCoerce> nsIQuotaUsageResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaUsageResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaUsageResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString origin; */
    pub GetOrigin: unsafe extern "system" fn (this: *const nsIQuotaUsageResult, aOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean persisted; */
    pub GetPersisted: unsafe extern "system" fn (this: *const nsIQuotaUsageResult, aPersisted: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long usage; */
    pub GetUsage: unsafe extern "system" fn (this: *const nsIQuotaUsageResult, aUsage: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long lastAccessed; */
    pub GetLastAccessed: unsafe extern "system" fn (this: *const nsIQuotaUsageResult, aLastAccessed: *mut u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaUsageResult {


    /// `readonly attribute ACString origin;`
    #[inline]
    pub unsafe fn GetOrigin(&self, aOrigin: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrigin)(self, aOrigin)
    }



    /// `readonly attribute boolean persisted;`
    #[inline]
    pub unsafe fn GetPersisted(&self, aPersisted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPersisted)(self, aPersisted)
    }



    /// `readonly attribute unsigned long long usage;`
    #[inline]
    pub unsafe fn GetUsage(&self, aUsage: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetUsage)(self, aUsage)
    }



    /// `readonly attribute unsigned long long lastAccessed;`
    #[inline]
    pub unsafe fn GetLastAccessed(&self, aLastAccessed: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastAccessed)(self, aLastAccessed)
    }


}


/// `interface nsIQuotaOriginUsageResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaOriginUsageResult {
    vtable: *const nsIQuotaOriginUsageResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaOriginUsageResult.
unsafe impl XpCom for nsIQuotaOriginUsageResult {
    const IID: nsIID = nsID(0x96df03d2, 0x116a, 0x493f,
        [0xbb, 0x0b, 0x11, 0x8c, 0x21, 0x2a, 0x6b, 0x32]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaOriginUsageResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaOriginUsageResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaOriginUsageResultCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaOriginUsageResult`.
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaOriginUsageResultCoerce for nsIQuotaOriginUsageResult {
    #[inline]
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self {
        v
    }
}

impl nsIQuotaOriginUsageResult {
    /// Cast this `nsIQuotaOriginUsageResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaOriginUsageResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaOriginUsageResult {
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
impl<T: nsISupportsCoerce> nsIQuotaOriginUsageResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaOriginUsageResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaOriginUsageResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaOriginUsageResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long long usage; */
    pub GetUsage: unsafe extern "system" fn (this: *const nsIQuotaOriginUsageResult, aUsage: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long fileUsage; */
    pub GetFileUsage: unsafe extern "system" fn (this: *const nsIQuotaOriginUsageResult, aFileUsage: *mut u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaOriginUsageResult {


    /// `readonly attribute unsigned long long usage;`
    #[inline]
    pub unsafe fn GetUsage(&self, aUsage: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetUsage)(self, aUsage)
    }



    /// `readonly attribute unsigned long long fileUsage;`
    #[inline]
    pub unsafe fn GetFileUsage(&self, aFileUsage: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetFileUsage)(self, aFileUsage)
    }


}


/// `interface nsIQuotaEstimateResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaEstimateResult {
    vtable: *const nsIQuotaEstimateResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaEstimateResult.
unsafe impl XpCom for nsIQuotaEstimateResult {
    const IID: nsIID = nsID(0x9827fc69, 0x7ea9, 0x48ef,
        [0xb3, 0x0d, 0x2e, 0x2a, 0xe0, 0x45, 0x1e, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaEstimateResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaEstimateResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaEstimateResultCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaEstimateResult`.
    fn coerce_from(v: &nsIQuotaEstimateResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaEstimateResultCoerce for nsIQuotaEstimateResult {
    #[inline]
    fn coerce_from(v: &nsIQuotaEstimateResult) -> &Self {
        v
    }
}

impl nsIQuotaEstimateResult {
    /// Cast this `nsIQuotaEstimateResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaEstimateResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaEstimateResult {
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
impl<T: nsISupportsCoerce> nsIQuotaEstimateResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaEstimateResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaEstimateResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaEstimateResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long long usage; */
    pub GetUsage: unsafe extern "system" fn (this: *const nsIQuotaEstimateResult, aUsage: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long limit; */
    pub GetLimit: unsafe extern "system" fn (this: *const nsIQuotaEstimateResult, aLimit: *mut u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaEstimateResult {


    /// `readonly attribute unsigned long long usage;`
    #[inline]
    pub unsafe fn GetUsage(&self, aUsage: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetUsage)(self, aUsage)
    }



    /// `readonly attribute unsigned long long limit;`
    #[inline]
    pub unsafe fn GetLimit(&self, aLimit: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetLimit)(self, aLimit)
    }


}


