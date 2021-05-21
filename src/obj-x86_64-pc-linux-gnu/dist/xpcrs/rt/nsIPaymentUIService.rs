//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentUIService.idl
//


/// `interface nsIPaymentUIService : nsISupports`
///

/// ```text
/// /**
///  * nsIPaymentUIService is the interface used by Gecko to communicate with the
///  * payment UI.
///  * In general, the implementation of this interface should be a service that
///  * manages all payment UI components and receives the requested payment actions
///  * from Gecko and perform the corresponding UI behavior.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentUIService {
    vtable: *const nsIPaymentUIServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentUIService.
unsafe impl XpCom for nsIPaymentUIService {
    const IID: nsIID = nsID(0x01f8bd55, 0x9017, 0x438b,
        [0x85, 0xec, 0x7c, 0x15, 0xd2, 0xb3, 0x5c, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentUIService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentUIService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentUIServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentUIService`.
    fn coerce_from(v: &nsIPaymentUIService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentUIServiceCoerce for nsIPaymentUIService {
    #[inline]
    fn coerce_from(v: &nsIPaymentUIService) -> &Self {
        v
    }
}

impl nsIPaymentUIService {
    /// Cast this `nsIPaymentUIService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentUIServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentUIService {
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
impl<T: nsISupportsCoerce> nsIPaymentUIServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentUIService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentUIService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentUIServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void showPayment (in AString requestId); */
    pub ShowPayment: unsafe extern "system" fn (this: *const nsIPaymentUIService, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void abortPayment (in AString requestId); */
    pub AbortPayment: unsafe extern "system" fn (this: *const nsIPaymentUIService, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void completePayment (in AString requestId); */
    pub CompletePayment: unsafe extern "system" fn (this: *const nsIPaymentUIService, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void updatePayment (in AString requestId); */
    pub UpdatePayment: unsafe extern "system" fn (this: *const nsIPaymentUIService, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void closePayment (in AString requestId); */
    pub ClosePayment: unsafe extern "system" fn (this: *const nsIPaymentUIService, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentUIService {

    /// ```text
    /// /**
    ///    *  Show the payment UI to users.
    ///    *  The implementation gets the payment data through nsIPaymentRequestService
    ///    *  by the passed in requestId, then shows the payment UI and start to interact
    ///    *  with users.
    ///    *  According to user's action, nsIPaymentRequestService's APIs respondPayment,
    ///    *  changeShippingAddress, or changeShippingOtpion is possible to called in the
    ///    *  implementation.
    ///    *  @param requestId - the request identify of the payment request.
    ///    *                     Notice that this requestId is an internal request Id
    ///    *                     generated by Gecko
    ///    */
    /// ```
    ///

    /// `void showPayment (in AString requestId);`
    #[inline]
    pub unsafe fn ShowPayment(&self, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ShowPayment)(self, requestId)
    }


    /// ```text
    /// /**
    ///    *  Abort the payment.
    ///    *  The implementation must abort and close the showing payment UI then call
    ///    *  nsIPaymentRequestService respondPayment with nsIPaymentAbortActionResponse
    ///    *  to inform Gecko of the abort status.
    ///    *  @param requestId - the request identify of the payment request.
    ///    *                     Notice that this requestId is an internal request Id
    ///    *                     generated by Gecko
    ///    */
    /// ```
    ///

    /// `void abortPayment (in AString requestId);`
    #[inline]
    pub unsafe fn AbortPayment(&self, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AbortPayment)(self, requestId)
    }


    /// ```text
    /// /**
    ///    *  Complete the payment.
    ///    *  The implementation should close the showing payment UI, then call
    ///    *  nsIPaymentRequestService respondPayment with nsIPaymentCompleteActionResponse
    ///    *  to inform Gecko of the complete status.
    ///    *  @param requestId - the request identify of the payment request.
    ///    *                     Notice that this requestId is an internal request Id
    ///    *                     generated by Gecko
    ///    */
    /// ```
    ///

    /// `void completePayment (in AString requestId);`
    #[inline]
    pub unsafe fn CompletePayment(&self, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CompletePayment)(self, requestId)
    }


    /// ```text
    /// /**
    ///    *  Update the payment data in the payment UI.
    ///    *  The implementation should get the updated payment data through the
    ///    *  nsIPaymentRequestService again, and update the UI.
    ///    *  @param requestId - the request identify of the payment request.
    ///    *                     Notice that this requestId is an internal request Id
    ///    *                     generated by Gecko
    ///    */
    /// ```
    ///

    /// `void updatePayment (in AString requestId);`
    #[inline]
    pub unsafe fn UpdatePayment(&self, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UpdatePayment)(self, requestId)
    }


    /// ```text
    /// /**
    ///    *  Close the payment UI for the specified PaymentRequest.
    ///    *  The implementation should clean up the PaymentRequest data saved in the UI
    ///    *  component and close the UI if the specified PaymentRequest is showing to
    ///    *  the user.
    ///    *  Notice when the method is called, that means the PaymentRequest is invalid
    ///    *  in nsIPaymentRequestService.
    ///    *  @param requestId - the request identify of the payment request.
    ///    *                     Notice that this requestId is an internal request Id
    ///    *                     generated by Gecko
    ///    */
    /// ```
    ///

    /// `void closePayment (in AString requestId);`
    #[inline]
    pub unsafe fn ClosePayment(&self, requestId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ClosePayment)(self, requestId)
    }


}


