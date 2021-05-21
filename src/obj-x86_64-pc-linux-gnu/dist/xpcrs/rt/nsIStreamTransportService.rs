//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamTransportService.idl
//


/// `interface nsIStreamTransportService : nsISupports`
///

/// ```text
/// /**
///  * This service read/writes a stream on a background thread.
///  *
///  * Note: instead of using this interface, probably you want to use
///  * NS_MakeAsyncNonBlockingInputStream.
///  *
///  * Use this service to transform any blocking stream (e.g., file stream)
///  * into a fully asynchronous stream that can be read/written without
///  * blocking the main thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamTransportService {
    vtable: *const nsIStreamTransportServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamTransportService.
unsafe impl XpCom for nsIStreamTransportService {
    const IID: nsIID = nsID(0x5e0adf7d, 0x9785, 0x45c3,
        [0xa1, 0x93, 0x04, 0xf2, 0x5a, 0x75, 0xda, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamTransportService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamTransportService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamTransportServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamTransportService`.
    fn coerce_from(v: &nsIStreamTransportService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamTransportServiceCoerce for nsIStreamTransportService {
    #[inline]
    fn coerce_from(v: &nsIStreamTransportService) -> &Self {
        v
    }
}

impl nsIStreamTransportService {
    /// Cast this `nsIStreamTransportService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamTransportServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamTransportService {
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
impl<T: nsISupportsCoerce> nsIStreamTransportServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamTransportService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamTransportService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamTransportServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsITransport createInputTransport (in nsIInputStream aStream, in boolean aCloseWhenDone); */
    pub CreateInputTransport: unsafe extern "system" fn (this: *const nsIStreamTransportService, aStream: *const nsIInputStream, aCloseWhenDone: bool, _retval: *mut*const nsITransport) -> ::nserror::nsresult,

    /* void InputAvailable (in nsIInputStream aStream, in nsIInputAvailableCallback aCallback); */
    pub InputAvailable: unsafe extern "system" fn (this: *const nsIStreamTransportService, aStream: *const nsIInputStream, aCallback: *const nsIInputAvailableCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamTransportService {

    /// ```text
    /// /**
    ///      * CreateInputTransport
    ///      *
    ///      * @param aStream
    ///      *        The input stream that will be read on a background thread.
    ///      *        This stream must implement "blocking" stream semantics.
    ///      * @param aCloseWhenDone
    ///      *        Specify this flag to have the input stream closed once its
    ///      *        contents have been completely read.
    ///      *
    ///      * @return nsITransport instance.
    ///      */
    /// ```
    ///

    /// `nsITransport createInputTransport (in nsIInputStream aStream, in boolean aCloseWhenDone);`
    #[inline]
    pub unsafe fn CreateInputTransport(&self, aStream: *const nsIInputStream, aCloseWhenDone: bool, _retval: *mut*const nsITransport) -> ::nserror::nsresult {
        ((*self.vtable).CreateInputTransport)(self, aStream, aCloseWhenDone, _retval)
    }



    /// `void InputAvailable (in nsIInputStream aStream, in nsIInputAvailableCallback aCallback);`
    #[inline]
    pub unsafe fn InputAvailable(&self, aStream: *const nsIInputStream, aCallback: *const nsIInputAvailableCallback) -> ::nserror::nsresult {
        ((*self.vtable).InputAvailable)(self, aStream, aCallback)
    }


}


/// `interface nsIInputAvailableCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputAvailableCallback {
    vtable: *const nsIInputAvailableCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputAvailableCallback.
unsafe impl XpCom for nsIInputAvailableCallback {
    const IID: nsIID = nsID(0xff2da731, 0x44d0, 0x4dd9,
        [0x82, 0x36, 0xc9, 0x93, 0x87, 0xfe, 0xc7, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputAvailableCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputAvailableCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputAvailableCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIInputAvailableCallback`.
    fn coerce_from(v: &nsIInputAvailableCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputAvailableCallbackCoerce for nsIInputAvailableCallback {
    #[inline]
    fn coerce_from(v: &nsIInputAvailableCallback) -> &Self {
        v
    }
}

impl nsIInputAvailableCallback {
    /// Cast this `nsIInputAvailableCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputAvailableCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputAvailableCallback {
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
impl<T: nsISupportsCoerce> nsIInputAvailableCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputAvailableCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputAvailableCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputAvailableCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onInputAvailableComplete (in unsigned long long available, in nsresult available_return_code); */
    pub OnInputAvailableComplete: unsafe extern "system" fn (this: *const nsIInputAvailableCallback, available: u64, available_return_code: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputAvailableCallback {


    /// `void onInputAvailableComplete (in unsigned long long available, in nsresult available_return_code);`
    #[inline]
    pub unsafe fn OnInputAvailableComplete(&self, available: u64, available_return_code: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnInputAvailableComplete)(self, available, available_return_code)
    }


}


