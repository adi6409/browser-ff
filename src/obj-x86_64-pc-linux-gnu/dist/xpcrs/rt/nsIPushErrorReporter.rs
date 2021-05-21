//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushErrorReporter.idl
//


/// `interface nsIPushErrorReporter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushErrorReporter {
    vtable: *const nsIPushErrorReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushErrorReporter.
unsafe impl XpCom for nsIPushErrorReporter {
    const IID: nsIID = nsID(0xb58249f9, 0x1a04, 0x48cc,
        [0xbc, 0x20, 0x2c, 0x99, 0x2d, 0x64, 0xc7, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushErrorReporter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushErrorReporter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushErrorReporterCoerce {
    /// Cheaply cast a value of this type from a `nsIPushErrorReporter`.
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushErrorReporterCoerce for nsIPushErrorReporter {
    #[inline]
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self {
        v
    }
}

impl nsIPushErrorReporter {
    /// Cast this `nsIPushErrorReporter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushErrorReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushErrorReporter {
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
impl<T: nsISupportsCoerce> nsIPushErrorReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushErrorReporter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushErrorReporter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushErrorReporterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void reportDeliveryError (in AString messageId, [optional] in uint16_t reason); */
    pub ReportDeliveryError: unsafe extern "system" fn (this: *const nsIPushErrorReporter, messageId: *const ::nsstring::nsAString, reason: uint16_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushErrorReporter {
    /// ```text
    /// /**
    ///    * Ack types, reported when the Push service acknowledges an incoming message.
    ///    *
    ///    * Acks are sent before the message is dispatched to the service worker,
    ///    * since the server delays new messages until all outstanding ones have been
    ///    * acked. |reportDeliveryError| will be called if an error occurs in the
    ///    * worker's `push` event handler after acking the message.
    ///   */
    /// ```
    ///

    pub const ACK_DELIVERED: i64 = 0;


    pub const ACK_DECRYPTION_ERROR: i64 = 1;


    pub const ACK_NOT_DELIVERED: i64 = 2;

    /// ```text
    /// /**
    ///    * Unsubscribe reasons, reported when the service drops a subscription.
    ///    */
    /// ```
    ///

    pub const UNSUBSCRIBE_MANUAL: i64 = 3;


    pub const UNSUBSCRIBE_QUOTA_EXCEEDED: i64 = 4;


    pub const UNSUBSCRIBE_PERMISSION_REVOKED: i64 = 5;

    /// ```text
    /// /**
    ///    * Delivery error reasons, reported when a service worker fails to handle
    ///    * an incoming push message in its `push` event handler.
    ///    */
    /// ```
    ///

    pub const DELIVERY_UNCAUGHT_EXCEPTION: i64 = 6;


    pub const DELIVERY_UNHANDLED_REJECTION: i64 = 7;


    pub const DELIVERY_INTERNAL_ERROR: i64 = 8;

    /// ```text
    /// /**
    ///    * Reports a `push` event handler error to the Push service. |messageId| is
    ///    * an opaque string passed to `nsIPushNotifier.notifyPush{WithData}`.
    ///    * |reason| is a delivery error reason.
    ///    */
    /// ```
    ///

    /// `void reportDeliveryError (in AString messageId, [optional] in uint16_t reason);`
    #[inline]
    pub unsafe fn ReportDeliveryError(&self, messageId: *const ::nsstring::nsAString, reason: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).ReportDeliveryError)(self, messageId, reason)
    }


}


