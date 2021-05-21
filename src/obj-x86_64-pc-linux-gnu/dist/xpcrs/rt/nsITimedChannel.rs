//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITimedChannel.idl
//


/// `interface nsIServerTiming : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServerTiming {
    vtable: *const nsIServerTimingVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServerTiming.
unsafe impl XpCom for nsIServerTiming {
    const IID: nsIID = nsID(0xc2d9e95b, 0x9cc9, 0x4f47,
        [0x9e, 0xf6, 0x1d, 0xe0, 0xcf, 0x7e, 0xbc, 0x75]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServerTiming {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServerTiming.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServerTimingCoerce {
    /// Cheaply cast a value of this type from a `nsIServerTiming`.
    fn coerce_from(v: &nsIServerTiming) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServerTimingCoerce for nsIServerTiming {
    #[inline]
    fn coerce_from(v: &nsIServerTiming) -> &Self {
        v
    }
}

impl nsIServerTiming {
    /// Cast this `nsIServerTiming` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServerTimingCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServerTiming {
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
impl<T: nsISupportsCoerce> nsIServerTimingCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServerTiming) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServerTiming
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServerTimingVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute ACString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIServerTiming, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] readonly attribute double duration; */
    pub GetDuration: unsafe extern "system" fn (this: *const nsIServerTiming, aDuration: *mut libc::c_double) -> ::nserror::nsresult,

    /* [must_use] readonly attribute ACString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsIServerTiming, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServerTiming {


    /// `[must_use] readonly attribute ACString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `[must_use] readonly attribute double duration;`
    #[inline]
    pub unsafe fn GetDuration(&self, aDuration: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDuration)(self, aDuration)
    }



    /// `[must_use] readonly attribute ACString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


}


/// `interface nsITimedChannel : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITimedChannel {
    vtable: *const nsITimedChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITimedChannel.
unsafe impl XpCom for nsITimedChannel {
    const IID: nsIID = nsID(0xca63784d, 0x959c, 0x4c3a,
        [0x9a, 0x59, 0x23, 0x4a, 0x2a, 0x52, 0x0d, 0xe0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITimedChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITimedChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITimedChannelCoerce {
    /// Cheaply cast a value of this type from a `nsITimedChannel`.
    fn coerce_from(v: &nsITimedChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITimedChannelCoerce for nsITimedChannel {
    #[inline]
    fn coerce_from(v: &nsITimedChannel) -> &Self {
        v
    }
}

impl nsITimedChannel {
    /// Cast this `nsITimedChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITimedChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITimedChannel {
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
impl<T: nsISupportsCoerce> nsITimedChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITimedChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITimedChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITimedChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean timingEnabled; */
    pub GetTimingEnabled: unsafe extern "system" fn (this: *const nsITimedChannel, aTimingEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean timingEnabled; */
    pub SetTimingEnabled: unsafe extern "system" fn (this: *const nsITimedChannel, aTimingEnabled: bool) -> ::nserror::nsresult,

    /* attribute uint8_t redirectCount; */
    pub GetRedirectCount: unsafe extern "system" fn (this: *const nsITimedChannel, aRedirectCount: *mut uint8_t) -> ::nserror::nsresult,

    /* attribute uint8_t redirectCount; */
    pub SetRedirectCount: unsafe extern "system" fn (this: *const nsITimedChannel, aRedirectCount: uint8_t) -> ::nserror::nsresult,

    /* attribute uint8_t internalRedirectCount; */
    pub GetInternalRedirectCount: unsafe extern "system" fn (this: *const nsITimedChannel, aInternalRedirectCount: *mut uint8_t) -> ::nserror::nsresult,

    /* attribute uint8_t internalRedirectCount; */
    pub SetInternalRedirectCount: unsafe extern "system" fn (this: *const nsITimedChannel, aInternalRedirectCount: uint8_t) -> ::nserror::nsresult,

    /* [noscript] attribute TimeStamp channelCreation; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetChannelCreation: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp channelCreation; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetChannelCreation: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp asyncOpen; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetAsyncOpen: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp asyncOpen; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetAsyncOpen: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLaunchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetLaunchServiceWorkerStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetLaunchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp launchServiceWorkerEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetLaunchServiceWorkerEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetDispatchFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetDispatchFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetDispatchFetchEventEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp dispatchFetchEventEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetDispatchFetchEventEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetHandleFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetHandleFetchEventStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetHandleFetchEventEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp handleFetchEventEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetHandleFetchEventEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp domainLookupStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetDomainLookupStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp domainLookupEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetDomainLookupEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp connectStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetConnectStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp tcpConnectEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetTcpConnectEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp secureConnectionStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetSecureConnectionStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp connectEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetConnectEnd: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp requestStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetRequestStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp responseStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetResponseStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp responseEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetResponseEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetRedirectStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetRedirectStart: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetRedirectEnd: *const ::libc::c_void,

    /* [noscript] attribute TimeStamp redirectEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetRedirectEnd: *const ::libc::c_void,

    /* [noscript] attribute AString initiatorType; */
    pub GetInitiatorType: unsafe extern "system" fn (this: *const nsITimedChannel, aInitiatorType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] attribute AString initiatorType; */
    pub SetInitiatorType: unsafe extern "system" fn (this: *const nsITimedChannel, aInitiatorType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] attribute boolean allRedirectsSameOrigin; */
    pub GetAllRedirectsSameOrigin: unsafe extern "system" fn (this: *const nsITimedChannel, aAllRedirectsSameOrigin: *mut bool) -> ::nserror::nsresult,

    /* [noscript] attribute boolean allRedirectsSameOrigin; */
    pub SetAllRedirectsSameOrigin: unsafe extern "system" fn (this: *const nsITimedChannel, aAllRedirectsSameOrigin: bool) -> ::nserror::nsresult,

    /* [noscript] attribute boolean allRedirectsPassTimingAllowCheck; */
    pub GetAllRedirectsPassTimingAllowCheck: unsafe extern "system" fn (this: *const nsITimedChannel, aAllRedirectsPassTimingAllowCheck: *mut bool) -> ::nserror::nsresult,

    /* [noscript] attribute boolean allRedirectsPassTimingAllowCheck; */
    pub SetAllRedirectsPassTimingAllowCheck: unsafe extern "system" fn (this: *const nsITimedChannel, aAllRedirectsPassTimingAllowCheck: bool) -> ::nserror::nsresult,

    /* [noscript] boolean timingAllowCheck (in nsIPrincipal origin); */
    pub TimingAllowCheck: unsafe extern "system" fn (this: *const nsITimedChannel, origin: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] readonly attribute TimeStamp cacheReadStart; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetCacheReadStart: *const ::libc::c_void,

    /* [noscript] readonly attribute TimeStamp cacheReadEnd; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetCacheReadEnd: *const ::libc::c_void,

    /* readonly attribute PRTime channelCreationTime; */
    pub GetChannelCreationTime: unsafe extern "system" fn (this: *const nsITimedChannel, aChannelCreationTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime asyncOpenTime; */
    pub GetAsyncOpenTime: unsafe extern "system" fn (this: *const nsITimedChannel, aAsyncOpenTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime launchServiceWorkerStartTime; */
    pub GetLaunchServiceWorkerStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aLaunchServiceWorkerStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime launchServiceWorkerEndTime; */
    pub GetLaunchServiceWorkerEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aLaunchServiceWorkerEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime dispatchFetchEventStartTime; */
    pub GetDispatchFetchEventStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aDispatchFetchEventStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime dispatchFetchEventEndTime; */
    pub GetDispatchFetchEventEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aDispatchFetchEventEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime handleFetchEventStartTime; */
    pub GetHandleFetchEventStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aHandleFetchEventStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime handleFetchEventEndTime; */
    pub GetHandleFetchEventEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aHandleFetchEventEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime domainLookupStartTime; */
    pub GetDomainLookupStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aDomainLookupStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime domainLookupEndTime; */
    pub GetDomainLookupEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aDomainLookupEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime connectStartTime; */
    pub GetConnectStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aConnectStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime tcpConnectEndTime; */
    pub GetTcpConnectEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aTcpConnectEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime secureConnectionStartTime; */
    pub GetSecureConnectionStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aSecureConnectionStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime connectEndTime; */
    pub GetConnectEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aConnectEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime requestStartTime; */
    pub GetRequestStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aRequestStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime responseStartTime; */
    pub GetResponseStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aResponseStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime responseEndTime; */
    pub GetResponseEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aResponseEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime cacheReadStartTime; */
    pub GetCacheReadStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aCacheReadStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime cacheReadEndTime; */
    pub GetCacheReadEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aCacheReadEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime redirectStartTime; */
    pub GetRedirectStartTime: unsafe extern "system" fn (this: *const nsITimedChannel, aRedirectStartTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime redirectEndTime; */
    pub GetRedirectEndTime: unsafe extern "system" fn (this: *const nsITimedChannel, aRedirectEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* [noscript] attribute boolean reportResourceTiming; */
    pub GetReportResourceTiming: unsafe extern "system" fn (this: *const nsITimedChannel, aReportResourceTiming: *mut bool) -> ::nserror::nsresult,

    /* [noscript] attribute boolean reportResourceTiming; */
    pub SetReportResourceTiming: unsafe extern "system" fn (this: *const nsITimedChannel, aReportResourceTiming: bool) -> ::nserror::nsresult,

    /* readonly attribute nsIArray serverTiming; */
    pub GetServerTiming: unsafe extern "system" fn (this: *const nsITimedChannel, aServerTiming: *mut*const nsIArray) -> ::nserror::nsresult,

    /* nsServerTimingArrayRef getNativeServerTiming (); */
    /// Unable to generate binding because `native type nsTArray<nsCOMPtr<nsIServerTiming>> unsupported`
    pub GetNativeServerTiming: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITimedChannel {


    /// `attribute boolean timingEnabled;`
    #[inline]
    pub unsafe fn GetTimingEnabled(&self, aTimingEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetTimingEnabled)(self, aTimingEnabled)
    }



    /// `attribute boolean timingEnabled;`
    #[inline]
    pub unsafe fn SetTimingEnabled(&self, aTimingEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetTimingEnabled)(self, aTimingEnabled)
    }



    /// `attribute uint8_t redirectCount;`
    #[inline]
    pub unsafe fn GetRedirectCount(&self, aRedirectCount: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirectCount)(self, aRedirectCount)
    }



    /// `attribute uint8_t redirectCount;`
    #[inline]
    pub unsafe fn SetRedirectCount(&self, aRedirectCount: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetRedirectCount)(self, aRedirectCount)
    }



    /// `attribute uint8_t internalRedirectCount;`
    #[inline]
    pub unsafe fn GetInternalRedirectCount(&self, aInternalRedirectCount: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetInternalRedirectCount)(self, aInternalRedirectCount)
    }



    /// `attribute uint8_t internalRedirectCount;`
    #[inline]
    pub unsafe fn SetInternalRedirectCount(&self, aInternalRedirectCount: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).SetInternalRedirectCount)(self, aInternalRedirectCount)
    }



    /// `[noscript] attribute TimeStamp channelCreation;`
    const _GetChannelCreation: () = ();


    /// `[noscript] attribute TimeStamp channelCreation;`
    const _SetChannelCreation: () = ();


    /// `[noscript] attribute TimeStamp asyncOpen;`
    const _GetAsyncOpen: () = ();


    /// `[noscript] attribute TimeStamp asyncOpen;`
    const _SetAsyncOpen: () = ();


    /// `[noscript] attribute TimeStamp launchServiceWorkerStart;`
    const _GetLaunchServiceWorkerStart: () = ();


    /// `[noscript] attribute TimeStamp launchServiceWorkerStart;`
    const _SetLaunchServiceWorkerStart: () = ();


    /// `[noscript] attribute TimeStamp launchServiceWorkerEnd;`
    const _GetLaunchServiceWorkerEnd: () = ();


    /// `[noscript] attribute TimeStamp launchServiceWorkerEnd;`
    const _SetLaunchServiceWorkerEnd: () = ();


    /// `[noscript] attribute TimeStamp dispatchFetchEventStart;`
    const _GetDispatchFetchEventStart: () = ();


    /// `[noscript] attribute TimeStamp dispatchFetchEventStart;`
    const _SetDispatchFetchEventStart: () = ();


    /// `[noscript] attribute TimeStamp dispatchFetchEventEnd;`
    const _GetDispatchFetchEventEnd: () = ();


    /// `[noscript] attribute TimeStamp dispatchFetchEventEnd;`
    const _SetDispatchFetchEventEnd: () = ();


    /// `[noscript] attribute TimeStamp handleFetchEventStart;`
    const _GetHandleFetchEventStart: () = ();


    /// `[noscript] attribute TimeStamp handleFetchEventStart;`
    const _SetHandleFetchEventStart: () = ();


    /// `[noscript] attribute TimeStamp handleFetchEventEnd;`
    const _GetHandleFetchEventEnd: () = ();


    /// `[noscript] attribute TimeStamp handleFetchEventEnd;`
    const _SetHandleFetchEventEnd: () = ();


    /// `[noscript] readonly attribute TimeStamp domainLookupStart;`
    const _GetDomainLookupStart: () = ();


    /// `[noscript] readonly attribute TimeStamp domainLookupEnd;`
    const _GetDomainLookupEnd: () = ();


    /// `[noscript] readonly attribute TimeStamp connectStart;`
    const _GetConnectStart: () = ();


    /// `[noscript] readonly attribute TimeStamp tcpConnectEnd;`
    const _GetTcpConnectEnd: () = ();


    /// `[noscript] readonly attribute TimeStamp secureConnectionStart;`
    const _GetSecureConnectionStart: () = ();


    /// `[noscript] readonly attribute TimeStamp connectEnd;`
    const _GetConnectEnd: () = ();


    /// `[noscript] readonly attribute TimeStamp requestStart;`
    const _GetRequestStart: () = ();


    /// `[noscript] readonly attribute TimeStamp responseStart;`
    const _GetResponseStart: () = ();


    /// `[noscript] readonly attribute TimeStamp responseEnd;`
    const _GetResponseEnd: () = ();


    /// `[noscript] attribute TimeStamp redirectStart;`
    const _GetRedirectStart: () = ();


    /// `[noscript] attribute TimeStamp redirectStart;`
    const _SetRedirectStart: () = ();


    /// `[noscript] attribute TimeStamp redirectEnd;`
    const _GetRedirectEnd: () = ();


    /// `[noscript] attribute TimeStamp redirectEnd;`
    const _SetRedirectEnd: () = ();


    /// `[noscript] attribute AString initiatorType;`
    #[inline]
    pub unsafe fn GetInitiatorType(&self, aInitiatorType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetInitiatorType)(self, aInitiatorType)
    }



    /// `[noscript] attribute AString initiatorType;`
    #[inline]
    pub unsafe fn SetInitiatorType(&self, aInitiatorType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetInitiatorType)(self, aInitiatorType)
    }



    /// `[noscript] attribute boolean allRedirectsSameOrigin;`
    #[inline]
    pub unsafe fn GetAllRedirectsSameOrigin(&self, aAllRedirectsSameOrigin: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllRedirectsSameOrigin)(self, aAllRedirectsSameOrigin)
    }



    /// `[noscript] attribute boolean allRedirectsSameOrigin;`
    #[inline]
    pub unsafe fn SetAllRedirectsSameOrigin(&self, aAllRedirectsSameOrigin: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllRedirectsSameOrigin)(self, aAllRedirectsSameOrigin)
    }



    /// `[noscript] attribute boolean allRedirectsPassTimingAllowCheck;`
    #[inline]
    pub unsafe fn GetAllRedirectsPassTimingAllowCheck(&self, aAllRedirectsPassTimingAllowCheck: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllRedirectsPassTimingAllowCheck)(self, aAllRedirectsPassTimingAllowCheck)
    }



    /// `[noscript] attribute boolean allRedirectsPassTimingAllowCheck;`
    #[inline]
    pub unsafe fn SetAllRedirectsPassTimingAllowCheck(&self, aAllRedirectsPassTimingAllowCheck: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAllRedirectsPassTimingAllowCheck)(self, aAllRedirectsPassTimingAllowCheck)
    }



    /// `[noscript] boolean timingAllowCheck (in nsIPrincipal origin);`
    #[inline]
    pub unsafe fn TimingAllowCheck(&self, origin: *const nsIPrincipal, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).TimingAllowCheck)(self, origin, _retval)
    }



    /// `[noscript] readonly attribute TimeStamp cacheReadStart;`
    const _GetCacheReadStart: () = ();


    /// `[noscript] readonly attribute TimeStamp cacheReadEnd;`
    const _GetCacheReadEnd: () = ();


    /// `readonly attribute PRTime channelCreationTime;`
    #[inline]
    pub unsafe fn GetChannelCreationTime(&self, aChannelCreationTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetChannelCreationTime)(self, aChannelCreationTime)
    }



    /// `readonly attribute PRTime asyncOpenTime;`
    #[inline]
    pub unsafe fn GetAsyncOpenTime(&self, aAsyncOpenTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetAsyncOpenTime)(self, aAsyncOpenTime)
    }



    /// `readonly attribute PRTime launchServiceWorkerStartTime;`
    #[inline]
    pub unsafe fn GetLaunchServiceWorkerStartTime(&self, aLaunchServiceWorkerStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLaunchServiceWorkerStartTime)(self, aLaunchServiceWorkerStartTime)
    }



    /// `readonly attribute PRTime launchServiceWorkerEndTime;`
    #[inline]
    pub unsafe fn GetLaunchServiceWorkerEndTime(&self, aLaunchServiceWorkerEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLaunchServiceWorkerEndTime)(self, aLaunchServiceWorkerEndTime)
    }



    /// `readonly attribute PRTime dispatchFetchEventStartTime;`
    #[inline]
    pub unsafe fn GetDispatchFetchEventStartTime(&self, aDispatchFetchEventStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetDispatchFetchEventStartTime)(self, aDispatchFetchEventStartTime)
    }



    /// `readonly attribute PRTime dispatchFetchEventEndTime;`
    #[inline]
    pub unsafe fn GetDispatchFetchEventEndTime(&self, aDispatchFetchEventEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetDispatchFetchEventEndTime)(self, aDispatchFetchEventEndTime)
    }



    /// `readonly attribute PRTime handleFetchEventStartTime;`
    #[inline]
    pub unsafe fn GetHandleFetchEventStartTime(&self, aHandleFetchEventStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetHandleFetchEventStartTime)(self, aHandleFetchEventStartTime)
    }



    /// `readonly attribute PRTime handleFetchEventEndTime;`
    #[inline]
    pub unsafe fn GetHandleFetchEventEndTime(&self, aHandleFetchEventEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetHandleFetchEventEndTime)(self, aHandleFetchEventEndTime)
    }



    /// `readonly attribute PRTime domainLookupStartTime;`
    #[inline]
    pub unsafe fn GetDomainLookupStartTime(&self, aDomainLookupStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetDomainLookupStartTime)(self, aDomainLookupStartTime)
    }



    /// `readonly attribute PRTime domainLookupEndTime;`
    #[inline]
    pub unsafe fn GetDomainLookupEndTime(&self, aDomainLookupEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetDomainLookupEndTime)(self, aDomainLookupEndTime)
    }



    /// `readonly attribute PRTime connectStartTime;`
    #[inline]
    pub unsafe fn GetConnectStartTime(&self, aConnectStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectStartTime)(self, aConnectStartTime)
    }



    /// `readonly attribute PRTime tcpConnectEndTime;`
    #[inline]
    pub unsafe fn GetTcpConnectEndTime(&self, aTcpConnectEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetTcpConnectEndTime)(self, aTcpConnectEndTime)
    }



    /// `readonly attribute PRTime secureConnectionStartTime;`
    #[inline]
    pub unsafe fn GetSecureConnectionStartTime(&self, aSecureConnectionStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetSecureConnectionStartTime)(self, aSecureConnectionStartTime)
    }



    /// `readonly attribute PRTime connectEndTime;`
    #[inline]
    pub unsafe fn GetConnectEndTime(&self, aConnectEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectEndTime)(self, aConnectEndTime)
    }



    /// `readonly attribute PRTime requestStartTime;`
    #[inline]
    pub unsafe fn GetRequestStartTime(&self, aRequestStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestStartTime)(self, aRequestStartTime)
    }



    /// `readonly attribute PRTime responseStartTime;`
    #[inline]
    pub unsafe fn GetResponseStartTime(&self, aResponseStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseStartTime)(self, aResponseStartTime)
    }



    /// `readonly attribute PRTime responseEndTime;`
    #[inline]
    pub unsafe fn GetResponseEndTime(&self, aResponseEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetResponseEndTime)(self, aResponseEndTime)
    }



    /// `readonly attribute PRTime cacheReadStartTime;`
    #[inline]
    pub unsafe fn GetCacheReadStartTime(&self, aCacheReadStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheReadStartTime)(self, aCacheReadStartTime)
    }



    /// `readonly attribute PRTime cacheReadEndTime;`
    #[inline]
    pub unsafe fn GetCacheReadEndTime(&self, aCacheReadEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheReadEndTime)(self, aCacheReadEndTime)
    }



    /// `readonly attribute PRTime redirectStartTime;`
    #[inline]
    pub unsafe fn GetRedirectStartTime(&self, aRedirectStartTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirectStartTime)(self, aRedirectStartTime)
    }



    /// `readonly attribute PRTime redirectEndTime;`
    #[inline]
    pub unsafe fn GetRedirectEndTime(&self, aRedirectEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetRedirectEndTime)(self, aRedirectEndTime)
    }



    /// `[noscript] attribute boolean reportResourceTiming;`
    #[inline]
    pub unsafe fn GetReportResourceTiming(&self, aReportResourceTiming: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetReportResourceTiming)(self, aReportResourceTiming)
    }



    /// `[noscript] attribute boolean reportResourceTiming;`
    #[inline]
    pub unsafe fn SetReportResourceTiming(&self, aReportResourceTiming: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetReportResourceTiming)(self, aReportResourceTiming)
    }



    /// `readonly attribute nsIArray serverTiming;`
    #[inline]
    pub unsafe fn GetServerTiming(&self, aServerTiming: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetServerTiming)(self, aServerTiming)
    }



    /// `nsServerTimingArrayRef getNativeServerTiming ();`
    const _GetNativeServerTiming: () = ();

}


