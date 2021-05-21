//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsILoginReputation.idl
//


/// `interface nsILoginReputationVerdictType : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginReputationVerdictType {
    vtable: *const nsILoginReputationVerdictTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginReputationVerdictType.
unsafe impl XpCom for nsILoginReputationVerdictType {
    const IID: nsIID = nsID(0x6219f9da, 0x297e, 0x446d,
        [0x8d, 0x47, 0xcc, 0xdd, 0x8e, 0x72, 0xa1, 0xd5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginReputationVerdictType {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginReputationVerdictType.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginReputationVerdictTypeCoerce {
    /// Cheaply cast a value of this type from a `nsILoginReputationVerdictType`.
    fn coerce_from(v: &nsILoginReputationVerdictType) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginReputationVerdictTypeCoerce for nsILoginReputationVerdictType {
    #[inline]
    fn coerce_from(v: &nsILoginReputationVerdictType) -> &Self {
        v
    }
}

impl nsILoginReputationVerdictType {
    /// Cast this `nsILoginReputationVerdictType` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginReputationVerdictTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginReputationVerdictType {
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
impl<T: nsISupportsCoerce> nsILoginReputationVerdictTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginReputationVerdictType) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginReputationVerdictType
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginReputationVerdictTypeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginReputationVerdictType {

    pub const UNSPECIFIED: i64 = 0;


    pub const SAFE: i64 = 1;


    pub const LOW_REPUTATION: i64 = 2;


    pub const PHISHING: i64 = 3;


}


/// `interface nsILoginReputationQuery : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginReputationQuery {
    vtable: *const nsILoginReputationQueryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginReputationQuery.
unsafe impl XpCom for nsILoginReputationQuery {
    const IID: nsIID = nsID(0xc21ffe59, 0x595f, 0x46c8,
        [0x90, 0x52, 0xfe, 0xfb, 0x63, 0x9e, 0x19, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginReputationQuery {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginReputationQuery.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginReputationQueryCoerce {
    /// Cheaply cast a value of this type from a `nsILoginReputationQuery`.
    fn coerce_from(v: &nsILoginReputationQuery) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginReputationQueryCoerce for nsILoginReputationQuery {
    #[inline]
    fn coerce_from(v: &nsILoginReputationQuery) -> &Self {
        v
    }
}

impl nsILoginReputationQuery {
    /// Cast this `nsILoginReputationQuery` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginReputationQueryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginReputationQuery {
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
impl<T: nsISupportsCoerce> nsILoginReputationQueryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginReputationQuery) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginReputationQuery
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginReputationQueryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIURI formURI; */
    pub GetFormURI: unsafe extern "system" fn (this: *const nsILoginReputationQuery, aFormURI: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginReputationQuery {


    /// `readonly attribute nsIURI formURI;`
    #[inline]
    pub unsafe fn GetFormURI(&self, aFormURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFormURI)(self, aFormURI)
    }


}


/// `interface nsILoginReputationQueryCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginReputationQueryCallback {
    vtable: *const nsILoginReputationQueryCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginReputationQueryCallback.
unsafe impl XpCom for nsILoginReputationQueryCallback {
    const IID: nsIID = nsID(0xb527be1e, 0x8fbb, 0x41d9,
        [0xbe, 0xe4, 0x26, 0x7a, 0x71, 0x23, 0x63, 0x68]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginReputationQueryCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginReputationQueryCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginReputationQueryCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsILoginReputationQueryCallback`.
    fn coerce_from(v: &nsILoginReputationQueryCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginReputationQueryCallbackCoerce for nsILoginReputationQueryCallback {
    #[inline]
    fn coerce_from(v: &nsILoginReputationQueryCallback) -> &Self {
        v
    }
}

impl nsILoginReputationQueryCallback {
    /// Cast this `nsILoginReputationQueryCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginReputationQueryCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginReputationQueryCallback {
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
impl<T: nsISupportsCoerce> nsILoginReputationQueryCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginReputationQueryCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginReputationQueryCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginReputationQueryCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsresult aStatus, in unsigned long aVerdict); */
    pub OnComplete: unsafe extern "system" fn (this: *const nsILoginReputationQueryCallback, aStatus: ::nserror::nsresult, aVerdict: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginReputationQueryCallback {


    /// `void onComplete (in nsresult aStatus, in unsigned long aVerdict);`
    #[inline]
    pub unsafe fn OnComplete(&self, aStatus: ::nserror::nsresult, aVerdict: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnComplete)(self, aStatus, aVerdict)
    }


}


/// `interface nsILoginReputationService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginReputationService {
    vtable: *const nsILoginReputationServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginReputationService.
unsafe impl XpCom for nsILoginReputationService {
    const IID: nsIID = nsID(0x1b3f1dfe, 0xce3a, 0x486b,
        [0x95, 0x3e, 0xce, 0x5a, 0xc8, 0x63, 0xef, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginReputationService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginReputationService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginReputationServiceCoerce {
    /// Cheaply cast a value of this type from a `nsILoginReputationService`.
    fn coerce_from(v: &nsILoginReputationService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginReputationServiceCoerce for nsILoginReputationService {
    #[inline]
    fn coerce_from(v: &nsILoginReputationService) -> &Self {
        v
    }
}

impl nsILoginReputationService {
    /// Cast this `nsILoginReputationService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginReputationServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginReputationService {
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
impl<T: nsISupportsCoerce> nsILoginReputationServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginReputationService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginReputationService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginReputationServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsILoginReputationService) -> ::nserror::nsresult,

    /* void queryReputationAsync (in HTMLInputElement aInput, in nsILoginReputationQueryCallback aCallback); */
    pub QueryReputationAsync: unsafe extern "system" fn (this: *const nsILoginReputationService, aInput: *const libc::c_void, aCallback: *const nsILoginReputationQueryCallback) -> ::nserror::nsresult,

    /* void queryReputation (in nsILoginReputationQuery aQuery, in nsILoginReputationQueryCallback aCallback); */
    pub QueryReputation: unsafe extern "system" fn (this: *const nsILoginReputationService, aQuery: *const nsILoginReputationQuery, aCallback: *const nsILoginReputationQueryCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginReputationService {


    /// `void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }



    /// `void queryReputationAsync (in HTMLInputElement aInput, in nsILoginReputationQueryCallback aCallback);`
    #[inline]
    pub unsafe fn QueryReputationAsync(&self, aInput: *const libc::c_void, aCallback: *const nsILoginReputationQueryCallback) -> ::nserror::nsresult {
        ((*self.vtable).QueryReputationAsync)(self, aInput, aCallback)
    }



    /// `void queryReputation (in nsILoginReputationQuery aQuery, in nsILoginReputationQueryCallback aCallback);`
    #[inline]
    pub unsafe fn QueryReputation(&self, aQuery: *const nsILoginReputationQuery, aCallback: *const nsILoginReputationQueryCallback) -> ::nserror::nsresult {
        ((*self.vtable).QueryReputation)(self, aQuery, aCallback)
    }


}


