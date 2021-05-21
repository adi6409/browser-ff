//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketImpl.idl
//


/// `interface nsIWebSocketImpl : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebSocketImpl {
    vtable: *const nsIWebSocketImplVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebSocketImpl.
unsafe impl XpCom for nsIWebSocketImpl {
    const IID: nsIID = nsID(0xdb1f4e2b, 0x3cff, 0x4615,
        [0xa0, 0x3c, 0x34, 0x1f, 0xda, 0x66, 0xc5, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebSocketImpl {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebSocketImpl.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebSocketImplCoerce {
    /// Cheaply cast a value of this type from a `nsIWebSocketImpl`.
    fn coerce_from(v: &nsIWebSocketImpl) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebSocketImplCoerce for nsIWebSocketImpl {
    #[inline]
    fn coerce_from(v: &nsIWebSocketImpl) -> &Self {
        v
    }
}

impl nsIWebSocketImpl {
    /// Cast this `nsIWebSocketImpl` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebSocketImplCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebSocketImpl {
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
impl<T: nsISupportsCoerce> nsIWebSocketImplCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebSocketImpl) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebSocketImpl
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebSocketImplVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void sendMessage (in AString aMessage); */
    pub SendMessage: unsafe extern "system" fn (this: *const nsIWebSocketImpl, aMessage: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebSocketImpl {

    /// ```text
    /// /**
    ///      * Called to send message of type string through web socket
    ///      *
    ///      * @param aMessage the message to send
    ///      */
    /// ```
    ///

    /// `[must_use] void sendMessage (in AString aMessage);`
    #[inline]
    pub unsafe fn SendMessage(&self, aMessage: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendMessage)(self, aMessage)
    }


}


