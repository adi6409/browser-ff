//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISpeculativeConnect.idl
//


/// `interface nsISpeculativeConnect : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeculativeConnect {
    vtable: *const nsISpeculativeConnectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeculativeConnect.
unsafe impl XpCom for nsISpeculativeConnect {
    const IID: nsIID = nsID(0xd74a17ac, 0x5b8a, 0x4824,
        [0xa3, 0x09, 0xb1, 0xf0, 0x4a, 0x3c, 0x4a, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeculativeConnect {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeculativeConnect.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeculativeConnectCoerce {
    /// Cheaply cast a value of this type from a `nsISpeculativeConnect`.
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeculativeConnectCoerce for nsISpeculativeConnect {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self {
        v
    }
}

impl nsISpeculativeConnect {
    /// Cast this `nsISpeculativeConnect` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeculativeConnectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeculativeConnect {
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
impl<T: nsISupportsCoerce> nsISpeculativeConnectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnect) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeculativeConnect
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeculativeConnectVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void speculativeConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    pub SpeculativeConnect: unsafe extern "system" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
    pub SpeculativeAnonymousConnect: unsafe extern "system" fn (this: *const nsISpeculativeConnect, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeculativeConnect {

    /// ```text
    /// /**
    ///      * Called as a hint to indicate a new transaction for the URI is likely coming
    ///      * soon. The implementer may use this information to start a TCP
    ///      * and/or SSL level handshake for that resource immediately so that it is
    ///      * ready and/or progressed when the transaction is actually submitted.
    ///      *
    ///      * No obligation is taken on by the implementer, nor is the submitter obligated
    ///      * to actually open the new channel.
    ///      *
    ///      * @param aURI the URI of the hinted transaction
    ///      * @param aPrincipal the principal that will be used for opening the
    ///      *        channel of the hinted transaction.
    ///      * @param aCallbacks any security callbacks for use with SSL for interfaces.
    ///      *        May be null.
    ///      *
    ///      */
    /// ```
    ///

    /// `void speculativeConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks);`
    #[inline]
    pub unsafe fn SpeculativeConnect(&self, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SpeculativeConnect)(self, aURI, aPrincipal, aCallbacks)
    }



    /// `void speculativeAnonymousConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks);`
    #[inline]
    pub unsafe fn SpeculativeAnonymousConnect(&self, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aCallbacks: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).SpeculativeAnonymousConnect)(self, aURI, aPrincipal, aCallbacks)
    }


}


/// `interface nsISpeculativeConnectionOverrider : nsISupports`
///

/// ```text
/// /**
///  * This is used to override the default values for various values (documented
    ///  * inline) to determine whether or not to actually make a speculative
///  * connection.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeculativeConnectionOverrider {
    vtable: *const nsISpeculativeConnectionOverriderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeculativeConnectionOverrider.
unsafe impl XpCom for nsISpeculativeConnectionOverrider {
    const IID: nsIID = nsID(0x1040ebe3, 0x6ed1, 0x45a6,
        [0x85, 0x87, 0x99, 0x5e, 0x08, 0x25, 0x18, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeculativeConnectionOverrider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeculativeConnectionOverrider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeculativeConnectionOverriderCoerce {
    /// Cheaply cast a value of this type from a `nsISpeculativeConnectionOverrider`.
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeculativeConnectionOverriderCoerce for nsISpeculativeConnectionOverrider {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self {
        v
    }
}

impl nsISpeculativeConnectionOverrider {
    /// Cast this `nsISpeculativeConnectionOverrider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeculativeConnectionOverriderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeculativeConnectionOverrider {
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
impl<T: nsISupportsCoerce> nsISpeculativeConnectionOverriderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeculativeConnectionOverrider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeculativeConnectionOverrider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeculativeConnectionOverriderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
    pub GetParallelSpeculativeConnectLimit: unsafe extern "system" fn (this: *const nsISpeculativeConnectionOverrider, aParallelSpeculativeConnectLimit: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean ignoreIdle; */
    pub GetIgnoreIdle: unsafe extern "system" fn (this: *const nsISpeculativeConnectionOverrider, aIgnoreIdle: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isFromPredictor; */
    pub GetIsFromPredictor: unsafe extern "system" fn (this: *const nsISpeculativeConnectionOverrider, aIsFromPredictor: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean allow1918; */
    pub GetAllow1918: unsafe extern "system" fn (this: *const nsISpeculativeConnectionOverrider, aAllow1918: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeculativeConnectionOverrider {

    /// ```text
    /// /**
    ///      * Used to determine the maximum number of unused speculative connections
    ///      * we will have open for a host at any one time
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit;`
    #[inline]
    pub unsafe fn GetParallelSpeculativeConnectLimit(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetParallelSpeculativeConnectLimit)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * Used to determine if we will ignore the existence of any currently idle
    ///      * connections when we decide whether or not to make a speculative
    ///      * connection.
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean ignoreIdle;`
    #[inline]
    pub unsafe fn GetIgnoreIdle(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIgnoreIdle)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean isFromPredictor;`
    #[inline]
    pub unsafe fn GetIsFromPredictor(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsFromPredictor)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///      * by default speculative connections are not made to RFC 1918 addresses
    ///      */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean allow1918;`
    #[inline]
    pub unsafe fn GetAllow1918(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetAllow1918)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


}


