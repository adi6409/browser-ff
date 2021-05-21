//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkInterceptController.idl
//


/// `interface nsIInterceptedBodyCallback : nsISupports`
///

/// ```text
/// /**
///  * Interface allowing the nsIInterceptedChannel to callback when it is
///  * done reading from the body stream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInterceptedBodyCallback {
    vtable: *const nsIInterceptedBodyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInterceptedBodyCallback.
unsafe impl XpCom for nsIInterceptedBodyCallback {
    const IID: nsIID = nsID(0x51039eb6, 0xbea0, 0x40c7,
        [0xb5, 0x23, 0xcc, 0xab, 0x56, 0xcc, 0x4f, 0xde]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInterceptedBodyCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInterceptedBodyCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInterceptedBodyCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIInterceptedBodyCallback`.
    fn coerce_from(v: &nsIInterceptedBodyCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInterceptedBodyCallbackCoerce for nsIInterceptedBodyCallback {
    #[inline]
    fn coerce_from(v: &nsIInterceptedBodyCallback) -> &Self {
        v
    }
}

impl nsIInterceptedBodyCallback {
    /// Cast this `nsIInterceptedBodyCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInterceptedBodyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInterceptedBodyCallback {
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
impl<T: nsISupportsCoerce> nsIInterceptedBodyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterceptedBodyCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInterceptedBodyCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInterceptedBodyCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void bodyComplete (in nsresult aRv); */
    pub BodyComplete: unsafe extern "system" fn (this: *const nsIInterceptedBodyCallback, aRv: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInterceptedBodyCallback {


    /// `void bodyComplete (in nsresult aRv);`
    #[inline]
    pub unsafe fn BodyComplete(&self, aRv: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).BodyComplete)(self, aRv)
    }


}


/// `interface nsIInterceptedChannel : nsISupports`
///

/// ```text
/// /**
///  * Interface to allow implementors of nsINetworkInterceptController to control the behaviour
///  * of intercepted channels without tying implementation details of the interception to
///  * the actual channel. nsIInterceptedChannel is expected to be implemented by objects
///  * which do not implement nsIChannel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInterceptedChannel {
    vtable: *const nsIInterceptedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInterceptedChannel.
unsafe impl XpCom for nsIInterceptedChannel {
    const IID: nsIID = nsID(0xf4b82975, 0x6a86, 0x4cc4,
        [0x87, 0xfe, 0x9a, 0x1f, 0xd4, 0x30, 0xc8, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInterceptedChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInterceptedChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInterceptedChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIInterceptedChannel`.
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInterceptedChannelCoerce for nsIInterceptedChannel {
    #[inline]
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self {
        v
    }
}

impl nsIInterceptedChannel {
    /// Cast this `nsIInterceptedChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInterceptedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInterceptedChannel {
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
impl<T: nsISupportsCoerce> nsIInterceptedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterceptedChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInterceptedChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInterceptedChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resetInterception (); */
    pub ResetInterception: unsafe extern "system" fn (this: *const nsIInterceptedChannel) -> ::nserror::nsresult,

    /* void synthesizeStatus (in uint16_t status, in ACString reason); */
    pub SynthesizeStatus: unsafe extern "system" fn (this: *const nsIInterceptedChannel, status: uint16_t, reason: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void synthesizeHeader (in ACString name, in ACString value); */
    pub SynthesizeHeader: unsafe extern "system" fn (this: *const nsIInterceptedChannel, name: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void startSynthesizedResponse (in nsIInputStream body, in nsIInterceptedBodyCallback callback, in nsICacheInfoChannel channel, in ACString finalURLSpec, in bool responseRedirected); */
    pub StartSynthesizedResponse: unsafe extern "system" fn (this: *const nsIInterceptedChannel, body: *const nsIInputStream, callback: *const nsIInterceptedBodyCallback, channel: *const nsICacheInfoChannel, finalURLSpec: *const ::nsstring::nsACString, responseRedirected: bool) -> ::nserror::nsresult,

    /* void finishSynthesizedResponse (); */
    pub FinishSynthesizedResponse: unsafe extern "system" fn (this: *const nsIInterceptedChannel) -> ::nserror::nsresult,

    /* void cancelInterception (in nsresult status); */
    pub CancelInterception: unsafe extern "system" fn (this: *const nsIInterceptedChannel, status: ::nserror::nsresult) -> ::nserror::nsresult,

    /* readonly attribute nsIChannel channel; */
    pub GetChannel: unsafe extern "system" fn (this: *const nsIInterceptedChannel, aChannel: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* readonly attribute nsIURI secureUpgradedChannelURI; */
    pub GetSecureUpgradedChannelURI: unsafe extern "system" fn (this: *const nsIInterceptedChannel, aSecureUpgradedChannelURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [noscript] void setChannelInfo (in ChannelInfo channelInfo); */
    /// Unable to generate binding because `native type mozilla::dom::ChannelInfo unsupported`
    pub SetChannelInfo: *const ::libc::c_void,

    /* [noscript] readonly attribute nsContentPolicyType internalContentPolicyType; */
    pub GetInternalContentPolicyType: unsafe extern "system" fn (this: *const nsIInterceptedChannel, aInternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsIConsoleReportCollector consoleReportCollector; */
    pub GetConsoleReportCollector: unsafe extern "system" fn (this: *const nsIInterceptedChannel, aConsoleReportCollector: *mut*const nsIConsoleReportCollector) -> ::nserror::nsresult,

    /* [noscript] void SetLaunchServiceWorkerStart (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetLaunchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] void GetLaunchServiceWorkerStart (out TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLaunchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] void SetLaunchServiceWorkerEnd (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetLaunchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] void GetLaunchServiceWorkerEnd (out TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLaunchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] void SetDispatchFetchEventStart (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetDispatchFetchEventStart: *const ::libc::c_void,

    /* [noscript] void SetDispatchFetchEventEnd (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetDispatchFetchEventEnd: *const ::libc::c_void,

    /* [noscript] void SetHandleFetchEventStart (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetHandleFetchEventStart: *const ::libc::c_void,

    /* [noscript] void SetHandleFetchEventEnd (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetHandleFetchEventEnd: *const ::libc::c_void,

    /* [noscript] void SetFinishResponseStart (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetFinishResponseStart: *const ::libc::c_void,

    /* [noscript] void SetFinishSynthesizedResponseEnd (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetFinishSynthesizedResponseEnd: *const ::libc::c_void,

    /* [noscript] void SetChannelResetEnd (in TimeStamp aTimeStamp); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetChannelResetEnd: *const ::libc::c_void,

    /* [noscript] void SaveTimeStamps (); */
    pub SaveTimeStamps: unsafe extern "system" fn (this: *const nsIInterceptedChannel) -> ::nserror::nsresult,

    /* [noscript] void setReleaseHandle (in nsISupports aHandle); */
    pub SetReleaseHandle: unsafe extern "system" fn (this: *const nsIInterceptedChannel, aHandle: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInterceptedChannel {

    /// ```text
    /// /**
    ///      * Instruct a channel that has been intercepted to continue with the original
    ///      * network request.
    ///      */
    /// ```
    ///

    /// `void resetInterception ();`
    #[inline]
    pub unsafe fn ResetInterception(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetInterception)(self, )
    }


    /// ```text
    /// /**
    ///      * Set the status and reason for the forthcoming synthesized response.
    ///      * Multiple calls overwrite existing values.
    ///      */
    /// ```
    ///

    /// `void synthesizeStatus (in uint16_t status, in ACString reason);`
    #[inline]
    pub unsafe fn SynthesizeStatus(&self, status: uint16_t, reason: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SynthesizeStatus)(self, status, reason)
    }


    /// ```text
    /// /**
    ///      * Attach a header name/value pair to the forthcoming synthesized response.
    ///      * Overwrites any existing header value.
    ///      */
    /// ```
    ///

    /// `void synthesizeHeader (in ACString name, in ACString value);`
    #[inline]
    pub unsafe fn SynthesizeHeader(&self, name: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SynthesizeHeader)(self, name, value)
    }


    /// ```text
    /// /**
    ///      * Instruct a channel that has been intercepted that a response is
    ///      * starting to be synthesized.  No further header modification is allowed
    ///      * after this point.  There are a few parameters:
    ///      * - A body stream may be optionally passed.  If nullptr, then an
    ///      *   empty body is assumed.
    ///      * - A callback may be optionally passed.  It will be invoked
    ///      *   when the body is complete.  For a nullptr body this may be
    ///      *   synchronously on the current thread.  Otherwise it will be invoked
    ///      *   asynchronously on the current thread.
    ///      * - A cacheInfoChannel may be optionally passed. If the body stream is
    ///      *   from alternative data cache, this cacheInfoChannel provides needed
    ///      *   cache information.
    ///      * - The caller may optionally pass a spec for a URL that this response
    ///      *   originates from; an empty string will cause the original
    ///      *   intercepted request's URL to be used instead.
    ///      * - The responseRedirected flag is false will cause the channel do an
    ///      *   internal redirect when the original intercepted reauest's URL is
    ///      *   different from the response's URL. The flag is true will cause the
    ///      *   chaanel do a non-internal redirect when the URLs are different.
    ///      */
    /// ```
    ///

    /// `void startSynthesizedResponse (in nsIInputStream body, in nsIInterceptedBodyCallback callback, in nsICacheInfoChannel channel, in ACString finalURLSpec, in bool responseRedirected);`
    #[inline]
    pub unsafe fn StartSynthesizedResponse(&self, body: *const nsIInputStream, callback: *const nsIInterceptedBodyCallback, channel: *const nsICacheInfoChannel, finalURLSpec: *const ::nsstring::nsACString, responseRedirected: bool) -> ::nserror::nsresult {
        ((*self.vtable).StartSynthesizedResponse)(self, body, callback, channel, finalURLSpec, responseRedirected)
    }


    /// ```text
    /// /**
    ///      * Instruct a channel that has been intercepted that response synthesis
    ///      * has completed and all outstanding resources can be closed.
    ///      */
    /// ```
    ///

    /// `void finishSynthesizedResponse ();`
    #[inline]
    pub unsafe fn FinishSynthesizedResponse(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FinishSynthesizedResponse)(self, )
    }


    /// ```text
    /// /**
    ///      * Cancel the pending intercepted request.
    ///      * @return NS_ERROR_FAILURE if the response has already been synthesized or
    ///      *         the original request has been instructed to continue.
    ///      */
    /// ```
    ///

    /// `void cancelInterception (in nsresult status);`
    #[inline]
    pub unsafe fn CancelInterception(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CancelInterception)(self, status)
    }


    /// ```text
    /// /**
    ///      * The underlying channel object that was intercepted.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIChannel channel;`
    #[inline]
    pub unsafe fn GetChannel(&self, aChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetChannel)(self, aChannel)
    }


    /// ```text
    /// /**
    ///      * The URL of the underlying channel object, corrected for a potential
    ///      * secure upgrade.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIURI secureUpgradedChannelURI;`
    #[inline]
    pub unsafe fn GetSecureUpgradedChannelURI(&self, aSecureUpgradedChannelURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetSecureUpgradedChannelURI)(self, aSecureUpgradedChannelURI)
    }


    /// ```text
    /// /**
    ///      * This method allows to override the channel info for the channel.
    ///      */
    /// ```
    ///

    /// `[noscript] void setChannelInfo (in ChannelInfo channelInfo);`
    const _SetChannelInfo: () = ();

    /// ```text
    /// /**
    ///      * Get the internal load type from the underlying channel.
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute nsContentPolicyType internalContentPolicyType;`
    #[inline]
    pub unsafe fn GetInternalContentPolicyType(&self, aInternalContentPolicyType: *mut nsContentPolicyType) -> ::nserror::nsresult {
        ((*self.vtable).GetInternalContentPolicyType)(self, aInternalContentPolicyType)
    }



    /// `[noscript] readonly attribute nsIConsoleReportCollector consoleReportCollector;`
    #[inline]
    pub unsafe fn GetConsoleReportCollector(&self, aConsoleReportCollector: *mut*const nsIConsoleReportCollector) -> ::nserror::nsresult {
        ((*self.vtable).GetConsoleReportCollector)(self, aConsoleReportCollector)
    }


    /// ```text
    /// /**
    ///      * Save the timestamps of various service worker interception phases.
    ///      */
    /// ```
    ///

    /// `[noscript] void SetLaunchServiceWorkerStart (in TimeStamp aTimeStamp);`
    const _SetLaunchServiceWorkerStart: () = ();


    /// `[noscript] void GetLaunchServiceWorkerStart (out TimeStamp aTimeStamp);`
    const _GetLaunchServiceWorkerStart: () = ();


    /// `[noscript] void SetLaunchServiceWorkerEnd (in TimeStamp aTimeStamp);`
    const _SetLaunchServiceWorkerEnd: () = ();


    /// `[noscript] void GetLaunchServiceWorkerEnd (out TimeStamp aTimeStamp);`
    const _GetLaunchServiceWorkerEnd: () = ();


    /// `[noscript] void SetDispatchFetchEventStart (in TimeStamp aTimeStamp);`
    const _SetDispatchFetchEventStart: () = ();


    /// `[noscript] void SetDispatchFetchEventEnd (in TimeStamp aTimeStamp);`
    const _SetDispatchFetchEventEnd: () = ();


    /// `[noscript] void SetHandleFetchEventStart (in TimeStamp aTimeStamp);`
    const _SetHandleFetchEventStart: () = ();


    /// `[noscript] void SetHandleFetchEventEnd (in TimeStamp aTimeStamp);`
    const _SetHandleFetchEventEnd: () = ();


    /// `[noscript] void SetFinishResponseStart (in TimeStamp aTimeStamp);`
    const _SetFinishResponseStart: () = ();


    /// `[noscript] void SetFinishSynthesizedResponseEnd (in TimeStamp aTimeStamp);`
    const _SetFinishSynthesizedResponseEnd: () = ();


    /// `[noscript] void SetChannelResetEnd (in TimeStamp aTimeStamp);`
    const _SetChannelResetEnd: () = ();


    /// `[noscript] void SaveTimeStamps ();`
    #[inline]
    pub unsafe fn SaveTimeStamps(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SaveTimeStamps)(self, )
    }


    /// ```text
    /// /**
    ///      * Allow the ServiceWorkerManager to set an RAII-style object on the
    ///      * intercepted channel that should be released once the channel is
    ///      * torn down.
    ///      */
    /// ```
    ///

    /// `[noscript] void setReleaseHandle (in nsISupports aHandle);`
    #[inline]
    pub unsafe fn SetReleaseHandle(&self, aHandle: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetReleaseHandle)(self, aHandle)
    }


}


/// `interface nsINetworkInterceptController : nsISupports`
///

/// ```text
/// /**
///  * Interface to allow consumers to attach themselves to a channel's
///  * notification callbacks/loadgroup and determine if a given channel
///  * request should be intercepted before any network request is initiated.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetworkInterceptController {
    vtable: *const nsINetworkInterceptControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkInterceptController.
unsafe impl XpCom for nsINetworkInterceptController {
    const IID: nsIID = nsID(0x70d2b4fe, 0xa552, 0x48cd,
        [0x8d, 0x93, 0x1d, 0x84, 0x37, 0xa5, 0x6b, 0x53]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkInterceptController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkInterceptController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkInterceptControllerCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkInterceptController`.
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkInterceptControllerCoerce for nsINetworkInterceptController {
    #[inline]
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self {
        v
    }
}

impl nsINetworkInterceptController {
    /// Cast this `nsINetworkInterceptController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkInterceptControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkInterceptController {
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
impl<T: nsISupportsCoerce> nsINetworkInterceptControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkInterceptController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkInterceptController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkInterceptControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool shouldPrepareForIntercept (in nsIURI aURI, in nsIChannel aChannel); */
    pub ShouldPrepareForIntercept: unsafe extern "system" fn (this: *const nsINetworkInterceptController, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult,

    /* void channelIntercepted (in nsIInterceptedChannel aChannel); */
    pub ChannelIntercepted: unsafe extern "system" fn (this: *const nsINetworkInterceptController, aChannel: *const nsIInterceptedChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkInterceptController {

    /// ```text
    /// /**
    ///      * Returns true if a channel should avoid initiating any network
    ///      * requests until specifically instructed to do so.
    ///      *
    ///      * @param aURI The URI to be loaded.  Note, this may differ from
    ///      *             the channel's current URL in some cases.
    ///      * @param aChannel The channel that may be intercepted.  It will
    ///      *                 be in the state prior to calling OnStartRequest().
    ///      */
    /// ```
    ///

    /// `bool shouldPrepareForIntercept (in nsIURI aURI, in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn ShouldPrepareForIntercept(&self, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ShouldPrepareForIntercept)(self, aURI, aChannel, _retval)
    }


    /// ```text
    /// /**
    ///      * Notification when a given intercepted channel is prepared to accept a synthesized
    ///      * response via the provided stream.
    ///      *
    ///      * @param aChannel the controlling interface for a channel that has been intercepted
    ///      */
    /// ```
    ///

    /// `void channelIntercepted (in nsIInterceptedChannel aChannel);`
    #[inline]
    pub unsafe fn ChannelIntercepted(&self, aChannel: *const nsIInterceptedChannel) -> ::nserror::nsresult {
        ((*self.vtable).ChannelIntercepted)(self, aChannel)
    }


}


