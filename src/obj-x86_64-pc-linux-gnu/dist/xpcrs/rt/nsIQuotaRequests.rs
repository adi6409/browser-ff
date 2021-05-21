//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/quota/nsIQuotaRequests.idl
//


/// `interface nsIQuotaRequestBase : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaRequestBase {
    vtable: *const nsIQuotaRequestBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaRequestBase.
unsafe impl XpCom for nsIQuotaRequestBase {
    const IID: nsIID = nsID(0x9af54222, 0x0407, 0x48fd,
        [0xa4, 0xab, 0x94, 0x57, 0xc9, 0x86, 0xfc, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaRequestBase {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaRequestBase.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaRequestBaseCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaRequestBase`.
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaRequestBaseCoerce for nsIQuotaRequestBase {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self {
        v
    }
}

impl nsIQuotaRequestBase {
    /// Cast this `nsIQuotaRequestBase` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaRequestBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaRequestBase {
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
impl<T: nsISupportsCoerce> nsIQuotaRequestBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequestBase) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaRequestBase
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaRequestBaseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIQuotaRequestBase, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsresult resultCode; */
    pub GetResultCode: unsafe extern "system" fn (this: *const nsIQuotaRequestBase, aResultCode: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString resultName; */
    pub GetResultName: unsafe extern "system" fn (this: *const nsIQuotaRequestBase, aResultName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaRequestBase {


    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `[must_use] readonly attribute nsresult resultCode;`
    #[inline]
    pub unsafe fn GetResultCode(&self, aResultCode: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetResultCode)(self, aResultCode)
    }



    /// `[must_use] readonly attribute ACString resultName;`
    #[inline]
    pub unsafe fn GetResultName(&self, aResultName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetResultName)(self, aResultName)
    }


}


/// `interface nsIQuotaUsageRequest : nsIQuotaRequestBase`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaUsageRequest {
    vtable: *const nsIQuotaUsageRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaUsageRequest.
unsafe impl XpCom for nsIQuotaUsageRequest {
    const IID: nsIID = nsID(0x166e28e6, 0xcf6d, 0x4927,
        [0xa6, 0xd7, 0xb5, 0x1b, 0xca, 0x9d, 0x34, 0x69]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaUsageRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaUsageRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaUsageRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaUsageRequest`.
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaUsageRequestCoerce for nsIQuotaUsageRequest {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self {
        v
    }
}

impl nsIQuotaUsageRequest {
    /// Cast this `nsIQuotaUsageRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaUsageRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaUsageRequest {
    type Target = nsIQuotaRequestBase;
    #[inline]
    fn deref(&self) -> &nsIQuotaRequestBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIQuotaRequestBaseCoerce> nsIQuotaUsageRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaUsageRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaUsageRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaUsageRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIQuotaRequestBaseVTable,

    /* [must_use] readonly attribute nsIVariant result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsIQuotaUsageRequest, aResult: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* attribute nsIQuotaUsageCallback callback; */
    pub GetCallback: unsafe extern "system" fn (this: *const nsIQuotaUsageRequest, aCallback: *mut*const nsIQuotaUsageCallback) -> ::nserror::nsresult,

    /* attribute nsIQuotaUsageCallback callback; */
    pub SetCallback: unsafe extern "system" fn (this: *const nsIQuotaUsageRequest, aCallback: *const nsIQuotaUsageCallback) -> ::nserror::nsresult,

    /* [must_use] void cancel (); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIQuotaUsageRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaUsageRequest {


    /// `[must_use] readonly attribute nsIVariant result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }



    /// `attribute nsIQuotaUsageCallback callback;`
    #[inline]
    pub unsafe fn GetCallback(&self, aCallback: *mut*const nsIQuotaUsageCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCallback)(self, aCallback)
    }



    /// `attribute nsIQuotaUsageCallback callback;`
    #[inline]
    pub unsafe fn SetCallback(&self, aCallback: *const nsIQuotaUsageCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCallback)(self, aCallback)
    }



    /// `[must_use] void cancel ();`
    #[inline]
    pub unsafe fn Cancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, )
    }


}


/// `interface nsIQuotaRequest : nsIQuotaRequestBase`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIQuotaRequest {
    vtable: *const nsIQuotaRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIQuotaRequest.
unsafe impl XpCom for nsIQuotaRequest {
    const IID: nsIID = nsID(0x22890e3e, 0xff25, 0x4372,
        [0x96, 0x84, 0xd9, 0x01, 0x06, 0x0e, 0x2f, 0x6c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIQuotaRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIQuotaRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIQuotaRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIQuotaRequest`.
    fn coerce_from(v: &nsIQuotaRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIQuotaRequestCoerce for nsIQuotaRequest {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequest) -> &Self {
        v
    }
}

impl nsIQuotaRequest {
    /// Cast this `nsIQuotaRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIQuotaRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIQuotaRequest {
    type Target = nsIQuotaRequestBase;
    #[inline]
    fn deref(&self) -> &nsIQuotaRequestBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIQuotaRequestBaseCoerce> nsIQuotaRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIQuotaRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIQuotaRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIQuotaRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIQuotaRequestBaseVTable,

    /* [must_use] readonly attribute nsIVariant result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsIQuotaRequest, aResult: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* attribute nsIQuotaCallback callback; */
    pub GetCallback: unsafe extern "system" fn (this: *const nsIQuotaRequest, aCallback: *mut*const nsIQuotaCallback) -> ::nserror::nsresult,

    /* attribute nsIQuotaCallback callback; */
    pub SetCallback: unsafe extern "system" fn (this: *const nsIQuotaRequest, aCallback: *const nsIQuotaCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIQuotaRequest {


    /// `[must_use] readonly attribute nsIVariant result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }



    /// `attribute nsIQuotaCallback callback;`
    #[inline]
    pub unsafe fn GetCallback(&self, aCallback: *mut*const nsIQuotaCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCallback)(self, aCallback)
    }



    /// `attribute nsIQuotaCallback callback;`
    #[inline]
    pub unsafe fn SetCallback(&self, aCallback: *const nsIQuotaCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCallback)(self, aCallback)
    }


}


