//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequestService.idl
//


/// `interface nsIPaymentRequestService : nsISupports`
///

/// ```text
/// /**
///  *  nsPaymentRequestService is used to manage the created PaymentRequest in the
///  *  chrome process. It is also the IPC agent for payment UI to communicate with
///  *  merchant side.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentRequestService {
    vtable: *const nsIPaymentRequestServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentRequestService.
unsafe impl XpCom for nsIPaymentRequestService {
    const IID: nsIID = nsID(0xcccd665f, 0xedf3, 0x41fc,
        [0xab, 0x9b, 0xfc, 0x55, 0xb3, 0x73, 0x40, 0xaa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentRequestService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentRequestService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentRequestServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentRequestService`.
    fn coerce_from(v: &nsIPaymentRequestService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentRequestServiceCoerce for nsIPaymentRequestService {
    #[inline]
    fn coerce_from(v: &nsIPaymentRequestService) -> &Self {
        v
    }
}

impl nsIPaymentRequestService {
    /// Cast this `nsIPaymentRequestService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentRequestServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentRequestService {
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
impl<T: nsISupportsCoerce> nsIPaymentRequestServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentRequestService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentRequestService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentRequestServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIPaymentRequest getPaymentRequestById (in AString aRequestId); */
    pub GetPaymentRequestById: unsafe extern "system" fn (this: *const nsIPaymentRequestService, aRequestId: *const ::nsstring::nsAString, _retval: *mut *const nsIPaymentRequest) -> ::nserror::nsresult,

    /* nsISimpleEnumerator enumerate (); */
    pub Enumerate: unsafe extern "system" fn (this: *const nsIPaymentRequestService, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* void respondPayment (in nsIPaymentActionResponse aResponse); */
    pub RespondPayment: unsafe extern "system" fn (this: *const nsIPaymentRequestService, aResponse: *const nsIPaymentActionResponse) -> ::nserror::nsresult,

    /* void changeShippingAddress (in AString requestId, in nsIPaymentAddress aAddress); */
    pub ChangeShippingAddress: unsafe extern "system" fn (this: *const nsIPaymentRequestService, requestId: *const ::nsstring::nsAString, aAddress: *const nsIPaymentAddress) -> ::nserror::nsresult,

    /* void changeShippingOption (in AString requestId, in AString option); */
    pub ChangeShippingOption: unsafe extern "system" fn (this: *const nsIPaymentRequestService, requestId: *const ::nsstring::nsAString, option: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void changePayerDetail (in AString requestId, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
    pub ChangePayerDetail: unsafe extern "system" fn (this: *const nsIPaymentRequestService, requestId: *const ::nsstring::nsAString, aPayerName: *const ::nsstring::nsAString, aPayerEmail: *const ::nsstring::nsAString, aPayerPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void changePaymentMethod (in AString requestId, in AString aMethodName, in nsIMethodChangeDetails aMethodDetails); */
    pub ChangePaymentMethod: unsafe extern "system" fn (this: *const nsIPaymentRequestService, requestId: *const ::nsstring::nsAString, aMethodName: *const ::nsstring::nsAString, aMethodDetails: *const nsIMethodChangeDetails) -> ::nserror::nsresult,

    /* void cleanup (); */
    pub Cleanup: unsafe extern "system" fn (this: *const nsIPaymentRequestService) -> ::nserror::nsresult,

    /* void setTestingUIService (in nsIPaymentUIService aUIService); */
    pub SetTestingUIService: unsafe extern "system" fn (this: *const nsIPaymentRequestService, aUIService: *const nsIPaymentUIService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentRequestService {

    /// ```text
    /// /**
    ///    *  Get the nsIPaymentRequest through the given payment request identifier.
    ///    *  @param aRequestId - the payment request identifier.
    ///    *                      This is an internal id generated by Gecko.
    ///    *  @return           - the requested payment request. null if there is no
    ///    *                      coressponding nsIPaymentRequest for aRequestId.
    ///    */
    /// ```
    ///

    /// `nsIPaymentRequest getPaymentRequestById (in AString aRequestId);`
    #[inline]
    pub unsafe fn GetPaymentRequestById(&self, aRequestId: *const ::nsstring::nsAString, _retval: *mut *const nsIPaymentRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetPaymentRequestById)(self, aRequestId, _retval)
    }


    /// ```text
    /// /**
    ///    *  Get the enumerator for all managed nsIPaymentRequests.
    ///    *  @return - an enumerator for all managed nsIPaymentRequests.
    ///    */
    /// ```
    ///

    /// `nsISimpleEnumerator enumerate ();`
    #[inline]
    pub unsafe fn Enumerate(&self, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Enumerate)(self, _retval)
    }


    /// ```text
    /// /**
    ///    *  Send the user's response to the merchant.
    ///    *  @param aResponse - the user's response.
    ///    */
    /// ```
    ///

    /// `void respondPayment (in nsIPaymentActionResponse aResponse);`
    #[inline]
    pub unsafe fn RespondPayment(&self, aResponse: *const nsIPaymentActionResponse) -> ::nserror::nsresult {
        ((*self.vtable).RespondPayment)(self, aResponse)
    }


    /// ```text
    /// /**
    ///    *  Inform the merchant the shipping address has changed.
    ///    *  @param requestId - the request identifier of the payment request.
    ///    *  @param aAddress - the new payment address.
    ///    */
    /// ```
    ///

    /// `void changeShippingAddress (in AString requestId, in nsIPaymentAddress aAddress);`
    #[inline]
    pub unsafe fn ChangeShippingAddress(&self, requestId: *const ::nsstring::nsAString, aAddress: *const nsIPaymentAddress) -> ::nserror::nsresult {
        ((*self.vtable).ChangeShippingAddress)(self, requestId, aAddress)
    }


    /// ```text
    /// /**
    ///    *  Inform the merchant the shipping option has changed.
    ///    *  @param requestId - the request identifier of the payment request.
    ///    *  @param option - the shipping option ID string.
    ///    */
    /// ```
    ///

    /// `void changeShippingOption (in AString requestId, in AString option);`
    #[inline]
    pub unsafe fn ChangeShippingOption(&self, requestId: *const ::nsstring::nsAString, option: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ChangeShippingOption)(self, requestId, option)
    }


    /// ```text
    /// /**
    ///    *  Inform the merchant the payer's details changed in the PaymentResponse.
    ///    *  @param requestId - the request identifier of the payment request.
    ///    *  @param aPayerName - the changed payer's name.
    ///    *  @param aPayerEmail - the changed payer's email.
    ///    *  @param aPayerPhone - the changed payer's phone.
    ///    */
    /// ```
    ///

    /// `void changePayerDetail (in AString requestId, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone);`
    #[inline]
    pub unsafe fn ChangePayerDetail(&self, requestId: *const ::nsstring::nsAString, aPayerName: *const ::nsstring::nsAString, aPayerEmail: *const ::nsstring::nsAString, aPayerPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ChangePayerDetail)(self, requestId, aPayerName, aPayerEmail, aPayerPhone)
    }


    /// ```text
    /// /**
    ///    *  Inform the merchant the payment method has changed.
    ///    *  @param requestId - the request identifier of the payment request.
    ///    *  @param aMethodName - the changed payment method's name.
    ///    *  @param aMethodDetails - the changed payment method's details.
    ///    */
    /// ```
    ///

    /// `void changePaymentMethod (in AString requestId, in AString aMethodName, in nsIMethodChangeDetails aMethodDetails);`
    #[inline]
    pub unsafe fn ChangePaymentMethod(&self, requestId: *const ::nsstring::nsAString, aMethodName: *const ::nsstring::nsAString, aMethodDetails: *const nsIMethodChangeDetails) -> ::nserror::nsresult {
        ((*self.vtable).ChangePaymentMethod)(self, requestId, aMethodName, aMethodDetails)
    }


    /// ```text
    /// /**
    ///    *  Following APIs are for testing or platform code only. UI implementation
    ///    *  should not use them.
    ///    */
    /// /**
    ///    *  Clean up the all managed payment requests.
    ///    *  This API is for testing only.
    ///    */
    /// ```
    ///

    /// `void cleanup ();`
    #[inline]
    pub unsafe fn Cleanup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cleanup)(self, )
    }


    /// ```text
    /// /**
    ///    *  Setup the customized nsIPaymentUIService.
    ///    *  This API is for testing only.
    ///    */
    /// ```
    ///

    /// `void setTestingUIService (in nsIPaymentUIService aUIService);`
    #[inline]
    pub unsafe fn SetTestingUIService(&self, aUIService: *const nsIPaymentUIService) -> ::nserror::nsresult {
        ((*self.vtable).SetTestingUIService)(self, aUIService)
    }


}


