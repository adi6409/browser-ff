//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserverProxy.idl
//


/// `interface nsIRequestObserverProxy : nsIRequestObserver`
///

/// ```text
/// /**
///  * A request observer proxy is used to ship data over to another thread
///  * specified by the thread's dispatch target. The "true" request observer's
///  * methods are invoked on the other thread.
///  *
///  * This interface only provides the initialization needed after construction.
///  * Otherwise, these objects are used simply as nsIRequestObserver's.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRequestObserverProxy {
    vtable: *const nsIRequestObserverProxyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRequestObserverProxy.
unsafe impl XpCom for nsIRequestObserverProxy {
    const IID: nsIID = nsID(0xc2b06151, 0x1bf8, 0x4eef,
        [0xae, 0xa9, 0x15, 0x32, 0xf1, 0x2f, 0x5a, 0x10]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRequestObserverProxy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRequestObserverProxy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRequestObserverProxyCoerce {
    /// Cheaply cast a value of this type from a `nsIRequestObserverProxy`.
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRequestObserverProxyCoerce for nsIRequestObserverProxy {
    #[inline]
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self {
        v
    }
}

impl nsIRequestObserverProxy {
    /// Cast this `nsIRequestObserverProxy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRequestObserverProxyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRequestObserverProxy {
    type Target = nsIRequestObserver;
    #[inline]
    fn deref(&self) -> &nsIRequestObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestObserverCoerce> nsIRequestObserverProxyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRequestObserverProxy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRequestObserverProxy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRequestObserverProxyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestObserverVTable,

    /* void init (in nsIRequestObserver observer, in nsISupports context); */
    pub Init: unsafe extern "system" fn (this: *const nsIRequestObserverProxy, observer: *const nsIRequestObserver, context: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRequestObserverProxy {

    /// ```text
    /// /**
    ///      * Initializes an nsIRequestObserverProxy.
    ///      *
    ///      * @param observer - receives observer notifications on the main thread
    ///      * @param context  - the context argument that will be passed to OnStopRequest
    ///      *                   and OnStartRequest. This has to be stored permanently on
    ///      *                   initialization because it sometimes can't be
    ///      *                   AddRef/Release'd off-main-thread.
    ///      */
    /// ```
    ///

    /// `void init (in nsIRequestObserver observer, in nsISupports context);`
    #[inline]
    pub unsafe fn Init(&self, observer: *const nsIRequestObserver, context: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, observer, context)
    }


}


