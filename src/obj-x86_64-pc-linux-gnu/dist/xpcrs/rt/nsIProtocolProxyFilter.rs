//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyFilter.idl
//


/// `interface nsIProxyProtocolFilterResult : nsISupports`
///

/// ```text
/// /**
///  * Recipient of the result of implementers of nsIProtocolProxy(Channel)Filter
///  * allowing the proxyinfo be provided asynchronously.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProxyProtocolFilterResult {
    vtable: *const nsIProxyProtocolFilterResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProxyProtocolFilterResult.
unsafe impl XpCom for nsIProxyProtocolFilterResult {
    const IID: nsIID = nsID(0x009e6c3f, 0xfb64, 0x40c5,
        [0x80, 0x93, 0xf1, 0x49, 0x5c, 0x64, 0x77, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProxyProtocolFilterResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProxyProtocolFilterResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProxyProtocolFilterResultCoerce {
    /// Cheaply cast a value of this type from a `nsIProxyProtocolFilterResult`.
    fn coerce_from(v: &nsIProxyProtocolFilterResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProxyProtocolFilterResultCoerce for nsIProxyProtocolFilterResult {
    #[inline]
    fn coerce_from(v: &nsIProxyProtocolFilterResult) -> &Self {
        v
    }
}

impl nsIProxyProtocolFilterResult {
    /// Cast this `nsIProxyProtocolFilterResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProxyProtocolFilterResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProxyProtocolFilterResult {
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
impl<T: nsISupportsCoerce> nsIProxyProtocolFilterResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProxyProtocolFilterResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProxyProtocolFilterResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProxyProtocolFilterResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onProxyFilterResult (in nsIProxyInfo aProxy); */
    pub OnProxyFilterResult: unsafe extern "system" fn (this: *const nsIProxyProtocolFilterResult, aProxy: *const nsIProxyInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProxyProtocolFilterResult {

    /// ```text
    /// /**
    ///    * It's mandatory to call this method exactly once when the applyFilter()
    ///    * implementation doesn't throw and to not call it when applyFilter() does
    ///    * throw.
    ///    *
    ///    * It's mandatory to call this method on the same thread as the call to
    ///    * applyFilter() has been made on.
    ///    *
    ///    * Following the above conditions, can be called either from within
    ///    * applyFilter() or asynchronouly any time later.
    ///    */
    /// ```
    ///

    /// `void onProxyFilterResult (in nsIProxyInfo aProxy);`
    #[inline]
    pub unsafe fn OnProxyFilterResult(&self, aProxy: *const nsIProxyInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnProxyFilterResult)(self, aProxy)
    }


}


/// `interface nsIProtocolProxyFilter : nsISupports`
///

/// ```text
/// /**
///  * This interface is used to apply filters to the proxies selected for a given
///  * URI.  Use nsIProtocolProxyService::registerFilter to hook up instances of
///  * this interface. See also nsIProtocolProxyChannelFilter.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolProxyFilter {
    vtable: *const nsIProtocolProxyFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolProxyFilter.
unsafe impl XpCom for nsIProtocolProxyFilter {
    const IID: nsIID = nsID(0xf424abd3, 0x32b4, 0x456c,
        [0x9f, 0x45, 0xb7, 0xe3, 0x37, 0x6c, 0xb0, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolProxyFilter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolProxyFilter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolProxyFilterCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolProxyFilter`.
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolProxyFilterCoerce for nsIProtocolProxyFilter {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self {
        v
    }
}

impl nsIProtocolProxyFilter {
    /// Cast this `nsIProtocolProxyFilter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolProxyFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolProxyFilter {
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
impl<T: nsISupportsCoerce> nsIProtocolProxyFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyFilter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolProxyFilter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolProxyFilterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void applyFilter (in nsIURI aURI, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
    pub ApplyFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyFilter, aURI: *const nsIURI, aProxy: *const nsIProxyInfo, aCallback: *const nsIProxyProtocolFilterResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolProxyFilter {

    /// ```text
    /// /**
    ///    * This method is called to apply proxy filter rules for the given URI
    ///    * and proxy object (or list of proxy objects).
    ///    *
    ///    * @param aURI
    ///    *        The URI for which these proxy settings apply.
    ///    * @param aProxy
    ///    *        The proxy (or list of proxies) that would be used by default for
    ///    *        the given URI.  This may be null.
    ///    *
    ///    * @param aCallback
    ///    *        An object that the implementer is obligated to call on with
    ///    *        the result (from within applyFilter() or asynchronously) when
    ///    *        applyFilter didn't throw.  The argument passed to onProxyFilterResult
    ///    *        is the proxy (or list of proxies) that should be used in place of
    ///    *        aProxy.  This can be just be aProxy if the filter chooses not to
    ///    *        modify the proxy.  It can also be null to indicate that a direct
    ///    *        connection should be used.  Use nsIProtocolProxyService.newProxyInfo
    ///    *        to construct nsIProxyInfo objects.
    ///    */
    /// ```
    ///

    /// `void applyFilter (in nsIURI aURI, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback);`
    #[inline]
    pub unsafe fn ApplyFilter(&self, aURI: *const nsIURI, aProxy: *const nsIProxyInfo, aCallback: *const nsIProxyProtocolFilterResult) -> ::nserror::nsresult {
        ((*self.vtable).ApplyFilter)(self, aURI, aProxy, aCallback)
    }


}


/// `interface nsIProtocolProxyChannelFilter : nsISupports`
///

/// ```text
/// /**
///  * This interface is used to apply filters to the proxies selected for a given
///  * channel.  Use nsIProtocolProxyService::registerChannelFilter to hook up instances of
///  * this interface. See also nsIProtocolProxyFilter.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolProxyChannelFilter {
    vtable: *const nsIProtocolProxyChannelFilterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolProxyChannelFilter.
unsafe impl XpCom for nsIProtocolProxyChannelFilter {
    const IID: nsIID = nsID(0x245b0880, 0x82c5, 0x4e6e,
        [0xbe, 0x6d, 0xbc, 0x58, 0x6a, 0xa5, 0x5a, 0x90]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolProxyChannelFilter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolProxyChannelFilter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolProxyChannelFilterCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolProxyChannelFilter`.
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolProxyChannelFilterCoerce for nsIProtocolProxyChannelFilter {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self {
        v
    }
}

impl nsIProtocolProxyChannelFilter {
    /// Cast this `nsIProtocolProxyChannelFilter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolProxyChannelFilterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolProxyChannelFilter {
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
impl<T: nsISupportsCoerce> nsIProtocolProxyChannelFilterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyChannelFilter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolProxyChannelFilter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolProxyChannelFilterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void applyFilter (in nsIChannel aChannel, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback); */
    pub ApplyFilter: unsafe extern "system" fn (this: *const nsIProtocolProxyChannelFilter, aChannel: *const nsIChannel, aProxy: *const nsIProxyInfo, aCallback: *const nsIProxyProtocolFilterResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolProxyChannelFilter {

    /// ```text
    /// /**
    ///    * This method is called to apply proxy filter rules for the given channel
    ///    * and proxy object (or list of proxy objects).
    ///    *
    ///    * @param aChannel
    ///    *        The channel for which these proxy settings apply.
    ///    * @param aProxy
    ///    *        The proxy (or list of proxies) that would be used by default for
    ///    *        the given channel. This may be null.
    ///    *
    ///    * @param aCallback
    ///    *        An object that the implementer is obligated to call on with
    ///    *        the result (from within applyFilter() or asynchronously) when
    ///    *        applyFilter didn't throw.  The argument passed to onProxyFilterResult
    ///    *        is the proxy (or list of proxies) that should be used in place of
    ///    *        aProxy.  This can be just be aProxy if the filter chooses not to
    ///    *        modify the proxy.  It can also be null to indicate that a direct
    ///    *        connection should be used.  Use nsIProtocolProxyService.newProxyInfo
    ///    *        to construct nsIProxyInfo objects.
    ///    */
    /// ```
    ///

    /// `void applyFilter (in nsIChannel aChannel, in nsIProxyInfo aProxy, in nsIProxyProtocolFilterResult aCallback);`
    #[inline]
    pub unsafe fn ApplyFilter(&self, aChannel: *const nsIChannel, aProxy: *const nsIProxyInfo, aCallback: *const nsIProxyProtocolFilterResult) -> ::nserror::nsresult {
        ((*self.vtable).ApplyFilter)(self, aChannel, aProxy, aCallback)
    }


}


