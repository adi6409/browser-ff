//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentActionResponse.idl
//


/// `interface nsIPaymentResponseData : nsISupports`
///

/// ```text
/// /**
///  *  The base interface of response data for the specified payment method.
///  *  The response data is the content of the PaymentResponse's detail attribute.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentResponseData {
    vtable: *const nsIPaymentResponseDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentResponseData.
unsafe impl XpCom for nsIPaymentResponseData {
    const IID: nsIID = nsID(0x2a338575, 0xc688, 0x40ee,
        [0xa1, 0x57, 0x74, 0x88, 0xab, 0x29, 0x2e, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentResponseData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentResponseData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentResponseDataCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentResponseData`.
    fn coerce_from(v: &nsIPaymentResponseData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentResponseDataCoerce for nsIPaymentResponseData {
    #[inline]
    fn coerce_from(v: &nsIPaymentResponseData) -> &Self {
        v
    }
}

impl nsIPaymentResponseData {
    /// Cast this `nsIPaymentResponseData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentResponseDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentResponseData {
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
impl<T: nsISupportsCoerce> nsIPaymentResponseDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentResponseData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentResponseData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentResponseDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIPaymentResponseData, aType: *mut uint32_t) -> ::nserror::nsresult,

    /* void init (in uint32_t aType); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentResponseData, aType: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentResponseData {
    /// ```text
    /// /**
    ///    *  The consts for representing the response data type.
    ///    *  GENERAL_RESPONSE is the general purpose response data type. Except basic
    ///    *  card response data, all response data should belong to this type.
    ///    *  BASICCARD_RESPONSE is a special response data type for basic card response.
    ///    */
    /// ```
    ///

    pub const GENERAL_RESPONSE: i64 = 0;


    pub const BASICCARD_RESPONSE: i64 = 1;

    /// ```text
    /// /**
    ///    *  The response data type.
    ///    *  Using the above defined consts(GENERAL_RESPONSE or BASICCARD_RESPONSE).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    *  The initial method.
    ///    *  @param aType - the response data type.
    ///    */
    /// ```
    ///

    /// `void init (in uint32_t aType);`
    #[inline]
    pub unsafe fn Init(&self, aType: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aType)
    }


}


/// `interface nsIGeneralResponseData : nsIPaymentResponseData`
///

/// ```text
/// /**
///  * The general purpose response data.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGeneralResponseData {
    vtable: *const nsIGeneralResponseDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGeneralResponseData.
unsafe impl XpCom for nsIGeneralResponseData {
    const IID: nsIID = nsID(0xb986773e, 0x2b30, 0x4ed2,
        [0xb8, 0xfe, 0x6a, 0x96, 0x63, 0x1c, 0x80, 0x00]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGeneralResponseData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGeneralResponseData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGeneralResponseDataCoerce {
    /// Cheaply cast a value of this type from a `nsIGeneralResponseData`.
    fn coerce_from(v: &nsIGeneralResponseData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGeneralResponseDataCoerce for nsIGeneralResponseData {
    #[inline]
    fn coerce_from(v: &nsIGeneralResponseData) -> &Self {
        v
    }
}

impl nsIGeneralResponseData {
    /// Cast this `nsIGeneralResponseData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGeneralResponseDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGeneralResponseData {
    type Target = nsIPaymentResponseData;
    #[inline]
    fn deref(&self) -> &nsIPaymentResponseData {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentResponseDataCoerce> nsIGeneralResponseDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeneralResponseData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGeneralResponseData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGeneralResponseDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentResponseDataVTable,

    /* readonly attribute AString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIGeneralResponseData, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] void initData (in jsval aData); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub InitData: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGeneralResponseData {

    /// ```text
    /// /**
    ///    *  The stringified response data.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


    /// ```text
    /// /**
    ///    *  The initial method for nsIGeneralResponseData.
    ///    *  @param aData - the javascript object of the content.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void initData (in jsval aData);`
    const _InitData: () = ();

}


/// `interface nsIBasicCardResponseData : nsIPaymentResponseData`
///

/// ```text
/// /**
///  *  The basic card response data.
///  *  Since PaymentAddress is an no constructor interface type, UI code can not
///  *  easy create PaymentAddress by calling new PaymentAddress().
///  *  Unfortunately, BasicCardResponse has a PaymentAddress attribute, billingAddress
///  *  , it means UI can not create BsaicCardResponse by calling the init() with a
///  *  given JSObject directly, because PaymentAddress creation in JS code is hard.
///  *  To let UI code can create BasicCardResponse easier, nsIBasicCardResponse is
///  *  provided for UI by passing the raw data of BasicCardResponse,
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBasicCardResponseData {
    vtable: *const nsIBasicCardResponseDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBasicCardResponseData.
unsafe impl XpCom for nsIBasicCardResponseData {
    const IID: nsIID = nsID(0x0d55a5e6, 0xd185, 0x44f0,
        [0xb9, 0x92, 0xa8, 0xe1, 0x32, 0x1e, 0x4b, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBasicCardResponseData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBasicCardResponseData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBasicCardResponseDataCoerce {
    /// Cheaply cast a value of this type from a `nsIBasicCardResponseData`.
    fn coerce_from(v: &nsIBasicCardResponseData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBasicCardResponseDataCoerce for nsIBasicCardResponseData {
    #[inline]
    fn coerce_from(v: &nsIBasicCardResponseData) -> &Self {
        v
    }
}

impl nsIBasicCardResponseData {
    /// Cast this `nsIBasicCardResponseData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBasicCardResponseDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBasicCardResponseData {
    type Target = nsIPaymentResponseData;
    #[inline]
    fn deref(&self) -> &nsIPaymentResponseData {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentResponseDataCoerce> nsIBasicCardResponseDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBasicCardResponseData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBasicCardResponseData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBasicCardResponseDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentResponseDataVTable,

    /* readonly attribute AString cardholderName; */
    pub GetCardholderName: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aCardholderName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString cardNumber; */
    pub GetCardNumber: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aCardNumber: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString expiryMonth; */
    pub GetExpiryMonth: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aExpiryMonth: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString expiryYear; */
    pub GetExpiryYear: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aExpiryYear: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString cardSecurityCode; */
    pub GetCardSecurityCode: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aCardSecurityCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentAddress billingAddress; */
    pub GetBillingAddress: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aBillingAddress: *mut *const nsIPaymentAddress) -> ::nserror::nsresult,

    /* void initData (in AString aCardholderName, in AString aCardNumber, in AString aExpiryMonth, in AString aExpiryYear, in AString aCardSecurityCode, in nsIPaymentAddress billingAddress); */
    pub InitData: unsafe extern "system" fn (this: *const nsIBasicCardResponseData, aCardholderName: *const ::nsstring::nsAString, aCardNumber: *const ::nsstring::nsAString, aExpiryMonth: *const ::nsstring::nsAString, aExpiryYear: *const ::nsstring::nsAString, aCardSecurityCode: *const ::nsstring::nsAString, billingAddress: *const nsIPaymentAddress) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBasicCardResponseData {

    /// ```text
    /// /**
    ///    *  The cardholder name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString cardholderName;`
    #[inline]
    pub unsafe fn GetCardholderName(&self, aCardholderName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCardholderName)(self, aCardholderName)
    }


    /// ```text
    /// /**
    ///    *  The card number.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString cardNumber;`
    #[inline]
    pub unsafe fn GetCardNumber(&self, aCardNumber: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCardNumber)(self, aCardNumber)
    }


    /// ```text
    /// /**
    ///    *  The expiry month.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString expiryMonth;`
    #[inline]
    pub unsafe fn GetExpiryMonth(&self, aExpiryMonth: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetExpiryMonth)(self, aExpiryMonth)
    }


    /// ```text
    /// /**
    ///    *  The expiry year.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString expiryYear;`
    #[inline]
    pub unsafe fn GetExpiryYear(&self, aExpiryYear: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetExpiryYear)(self, aExpiryYear)
    }


    /// ```text
    /// /**
    ///    *  The card security number.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString cardSecurityCode;`
    #[inline]
    pub unsafe fn GetCardSecurityCode(&self, aCardSecurityCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCardSecurityCode)(self, aCardSecurityCode)
    }


    /// ```text
    /// /**
    ///    *  The billing address.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPaymentAddress billingAddress;`
    #[inline]
    pub unsafe fn GetBillingAddress(&self, aBillingAddress: *mut *const nsIPaymentAddress) -> ::nserror::nsresult {
        ((*self.vtable).GetBillingAddress)(self, aBillingAddress)
    }


    /// ```text
    /// /**
    ///    *  The initial method for nsIBasicCardResponseData.
    ///    *  @param aCardholderName   - the cardholder name.
    ///    *  @param aCardNumber       - the card number.
    ///    *  @param aExpiryMonth      - the expiry month.
    ///    *  @param aExpiryYear       - the expiry year.
    ///    *  @param aCardSecurityCode - the card security code.
    ///    *  @param aBillingAddreess  - the billing address.
    ///    */
    /// ```
    ///

    /// `void initData (in AString aCardholderName, in AString aCardNumber, in AString aExpiryMonth, in AString aExpiryYear, in AString aCardSecurityCode, in nsIPaymentAddress billingAddress);`
    #[inline]
    pub unsafe fn InitData(&self, aCardholderName: *const ::nsstring::nsAString, aCardNumber: *const ::nsstring::nsAString, aExpiryMonth: *const ::nsstring::nsAString, aExpiryYear: *const ::nsstring::nsAString, aCardSecurityCode: *const ::nsstring::nsAString, billingAddress: *const nsIPaymentAddress) -> ::nserror::nsresult {
        ((*self.vtable).InitData)(self, aCardholderName, aCardNumber, aExpiryMonth, aExpiryYear, aCardSecurityCode, billingAddress)
    }


}


/// `interface nsIPaymentActionResponse : nsISupports`
///

/// ```text
/// /**
///  *  The base interface of user's response.
///  *  Payment UI should create different sub-interface of nsIPaymentActionResponse
///  *  according to user's action, and call nsIPaymentRequestService::respondPayment
///  *  with the created action to inform the merchant.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentActionResponse {
    vtable: *const nsIPaymentActionResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentActionResponse.
unsafe impl XpCom for nsIPaymentActionResponse {
    const IID: nsIID = nsID(0xa607c095, 0xef60, 0x4a9b,
        [0xa3, 0xd0, 0x05, 0x06, 0xc6, 0x07, 0x28, 0xb3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentActionResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentActionResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentActionResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentActionResponse`.
    fn coerce_from(v: &nsIPaymentActionResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentActionResponseCoerce for nsIPaymentActionResponse {
    #[inline]
    fn coerce_from(v: &nsIPaymentActionResponse) -> &Self {
        v
    }
}

impl nsIPaymentActionResponse {
    /// Cast this `nsIPaymentActionResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentActionResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentActionResponse {
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
impl<T: nsISupportsCoerce> nsIPaymentActionResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentActionResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentActionResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentActionResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString requestId; */
    pub GetRequestId: unsafe extern "system" fn (this: *const nsIPaymentActionResponse, aRequestId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIPaymentActionResponse, aType: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentActionResponse {
    /// ```text
    /// /**
    ///    *  The response type.
    ///    *  Align type to nsIPaymentActionRequest types,
    ///    *  where 1 is for payment request creation.
    ///    *  the action expects no response from UI module.
    ///    */
    /// ```
    ///

    pub const NO_TYPE: i64 = 0;


    pub const CANMAKE_ACTION: i64 = 2;


    pub const SHOW_ACTION: i64 = 3;


    pub const ABORT_ACTION: i64 = 4;


    pub const COMPLETE_ACTION: i64 = 5;

    /// ```text
    /// /**
    ///    *  The abort status.
    ///    */
    /// ```
    ///

    pub const ABORT_SUCCEEDED: i64 = 1;


    pub const ABORT_FAILED: i64 = 0;

    /// ```text
    /// /**
    ///    *  The payment status.
    ///    */
    /// ```
    ///

    pub const PAYMENT_REJECTED: i64 = 0;


    pub const PAYMENT_ACCEPTED: i64 = 1;


    pub const PAYMENT_NOTSUPPORTED: i64 = 2;

    /// ```text
    /// /**
    ///    *  The complete status.
    ///    */
    /// ```
    ///

    pub const COMPLETE_SUCCEEDED: i64 = 1;


    pub const COMPLETE_FAILED: i64 = 0;


    /// `readonly attribute AString requestId;`
    #[inline]
    pub unsafe fn GetRequestId(&self, aRequestId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestId)(self, aRequestId)
    }



    /// `readonly attribute uint32_t type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


/// `interface nsIPaymentCanMakeActionResponse : nsIPaymentActionResponse`
///

/// ```text
/// /**
///  *  The response for canMakePayment action.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentCanMakeActionResponse {
    vtable: *const nsIPaymentCanMakeActionResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentCanMakeActionResponse.
unsafe impl XpCom for nsIPaymentCanMakeActionResponse {
    const IID: nsIID = nsID(0x52fc3f9f, 0xc0cb, 0x4874,
        [0xb3, 0xd4, 0xee, 0x4b, 0x6e, 0x9c, 0xbe, 0x9c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentCanMakeActionResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentCanMakeActionResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentCanMakeActionResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentCanMakeActionResponse`.
    fn coerce_from(v: &nsIPaymentCanMakeActionResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentCanMakeActionResponseCoerce for nsIPaymentCanMakeActionResponse {
    #[inline]
    fn coerce_from(v: &nsIPaymentCanMakeActionResponse) -> &Self {
        v
    }
}

impl nsIPaymentCanMakeActionResponse {
    /// Cast this `nsIPaymentCanMakeActionResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentCanMakeActionResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentCanMakeActionResponse {
    type Target = nsIPaymentActionResponse;
    #[inline]
    fn deref(&self) -> &nsIPaymentActionResponse {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentActionResponseCoerce> nsIPaymentCanMakeActionResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentCanMakeActionResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentCanMakeActionResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentCanMakeActionResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentActionResponseVTable,

    /* readonly attribute bool result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsIPaymentCanMakeActionResponse, aResult: *mut bool) -> ::nserror::nsresult,

    /* void init (in AString aRequestId, in bool aResult); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentCanMakeActionResponse, aRequestId: *const ::nsstring::nsAString, aResult: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentCanMakeActionResponse {

    /// ```text
    /// /**
    ///    *  The result of canMakePayment action.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }


    /// ```text
    /// /**
    ///    *  The initial method.
    ///    *  @param aRequestId - the request identifier of the payment request.
    ///    *  @param aResult - the canMakePayment result.
    ///    */
    /// ```
    ///

    /// `void init (in AString aRequestId, in bool aResult);`
    #[inline]
    pub unsafe fn Init(&self, aRequestId: *const ::nsstring::nsAString, aResult: bool) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aRequestId, aResult)
    }


}


/// `interface nsIPaymentShowActionResponse : nsIPaymentActionResponse`
///

/// ```text
/// /**
///  *  The response for show action.
///  *  Notice that to represent user's cancel, we should use nsIPaymentShowActionResponse
///  *  with PAYMENT_REJECTED status, not nsIPaymentAbortActionResponse.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentShowActionResponse {
    vtable: *const nsIPaymentShowActionResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentShowActionResponse.
unsafe impl XpCom for nsIPaymentShowActionResponse {
    const IID: nsIID = nsID(0x184385cb, 0x2d35, 0x4b99,
        [0xa9, 0xa3, 0x7c, 0x78, 0x0b, 0xf6, 0x6b, 0x9b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentShowActionResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentShowActionResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentShowActionResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentShowActionResponse`.
    fn coerce_from(v: &nsIPaymentShowActionResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentShowActionResponseCoerce for nsIPaymentShowActionResponse {
    #[inline]
    fn coerce_from(v: &nsIPaymentShowActionResponse) -> &Self {
        v
    }
}

impl nsIPaymentShowActionResponse {
    /// Cast this `nsIPaymentShowActionResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentShowActionResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentShowActionResponse {
    type Target = nsIPaymentActionResponse;
    #[inline]
    fn deref(&self) -> &nsIPaymentActionResponse {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentActionResponseCoerce> nsIPaymentShowActionResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentShowActionResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentShowActionResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentShowActionResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentActionResponseVTable,

    /* readonly attribute uint32_t acceptStatus; */
    pub GetAcceptStatus: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aAcceptStatus: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute AString methodName; */
    pub GetMethodName: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aMethodName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentResponseData data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aData: *mut *const nsIPaymentResponseData) -> ::nserror::nsresult,

    /* readonly attribute AString payerName; */
    pub GetPayerName: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aPayerName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString payerEmail; */
    pub GetPayerEmail: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aPayerEmail: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString payerPhone; */
    pub GetPayerPhone: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aPayerPhone: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void init (in AString aRequestId, in uint32_t aAcceptStatus, in AString aMethodName, in nsIPaymentResponseData aData, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentShowActionResponse, aRequestId: *const ::nsstring::nsAString, aAcceptStatus: uint32_t, aMethodName: *const ::nsstring::nsAString, aData: *const nsIPaymentResponseData, aPayerName: *const ::nsstring::nsAString, aPayerEmail: *const ::nsstring::nsAString, aPayerPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentShowActionResponse {

    /// ```text
    /// /**
    ///    *  Accept status of the payment.
    ///    *  Using the defined consts(PAYMENT_XXX) in nsIPaymentActionResponse.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t acceptStatus;`
    #[inline]
    pub unsafe fn GetAcceptStatus(&self, aAcceptStatus: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAcceptStatus)(self, aAcceptStatus)
    }


    /// ```text
    /// /**
    ///    *  The decided payment method name. i.e. "basic-card".
    ///    */
    /// ```
    ///

    /// `readonly attribute AString methodName;`
    #[inline]
    pub unsafe fn GetMethodName(&self, aMethodName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetMethodName)(self, aMethodName)
    }


    /// ```text
    /// /**
    ///    *  The data needed by the payment method. (it must be serializable)
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPaymentResponseData data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsIPaymentResponseData) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


    /// ```text
    /// /**
    ///    *  The payer name information.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString payerName;`
    #[inline]
    pub unsafe fn GetPayerName(&self, aPayerName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPayerName)(self, aPayerName)
    }


    /// ```text
    /// /**
    ///    *  The payer email information.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString payerEmail;`
    #[inline]
    pub unsafe fn GetPayerEmail(&self, aPayerEmail: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPayerEmail)(self, aPayerEmail)
    }


    /// ```text
    /// /**
    ///    *  The payer phone information.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString payerPhone;`
    #[inline]
    pub unsafe fn GetPayerPhone(&self, aPayerPhone: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPayerPhone)(self, aPayerPhone)
    }


    /// ```text
    /// /**
    ///    *  The initial method.
    ///    *  @param aRequestId - the request identifier of the payment request.
    ///    *  @param aAcceptStatus - the payment status.
    ///    *  @param aMethodName - the decided method name.
    ///    *  @param aData - the response data for the decided payment method.
    ///    *  @param aPayerName - the payer's name.
    ///    *  @param aPayerEmail - the payer's email.
    ///    *  @param aPayerPhone - the payer's phone.
    ///    */
    /// ```
    ///

    /// `void init (in AString aRequestId, in uint32_t aAcceptStatus, in AString aMethodName, in nsIPaymentResponseData aData, in AString aPayerName, in AString aPayerEmail, in AString aPayerPhone);`
    #[inline]
    pub unsafe fn Init(&self, aRequestId: *const ::nsstring::nsAString, aAcceptStatus: uint32_t, aMethodName: *const ::nsstring::nsAString, aData: *const nsIPaymentResponseData, aPayerName: *const ::nsstring::nsAString, aPayerEmail: *const ::nsstring::nsAString, aPayerPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aRequestId, aAcceptStatus, aMethodName, aData, aPayerName, aPayerEmail, aPayerPhone)
    }


}


/// `interface nsIPaymentAbortActionResponse : nsIPaymentActionResponse`
///

/// ```text
/// /**
///  *  The response for abort action.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentAbortActionResponse {
    vtable: *const nsIPaymentAbortActionResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentAbortActionResponse.
unsafe impl XpCom for nsIPaymentAbortActionResponse {
    const IID: nsIID = nsID(0x8c72bcdb, 0x0c37, 0x4786,
        [0xa9, 0xe5, 0x51, 0x0a, 0xfa, 0x2f, 0x8e, 0xde]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentAbortActionResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentAbortActionResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentAbortActionResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentAbortActionResponse`.
    fn coerce_from(v: &nsIPaymentAbortActionResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentAbortActionResponseCoerce for nsIPaymentAbortActionResponse {
    #[inline]
    fn coerce_from(v: &nsIPaymentAbortActionResponse) -> &Self {
        v
    }
}

impl nsIPaymentAbortActionResponse {
    /// Cast this `nsIPaymentAbortActionResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentAbortActionResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentAbortActionResponse {
    type Target = nsIPaymentActionResponse;
    #[inline]
    fn deref(&self) -> &nsIPaymentActionResponse {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentActionResponseCoerce> nsIPaymentAbortActionResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentAbortActionResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentAbortActionResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentAbortActionResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentActionResponseVTable,

    /* readonly attribute uint32_t abortStatus; */
    pub GetAbortStatus: unsafe extern "system" fn (this: *const nsIPaymentAbortActionResponse, aAbortStatus: *mut uint32_t) -> ::nserror::nsresult,

    /* void init (in AString aRequestId, in uint32_t aAbortStatus); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentAbortActionResponse, aRequestId: *const ::nsstring::nsAString, aAbortStatus: uint32_t) -> ::nserror::nsresult,

    /* bool isSucceeded (); */
    pub IsSucceeded: unsafe extern "system" fn (this: *const nsIPaymentAbortActionResponse, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentAbortActionResponse {

    /// ```text
    /// /**
    ///    *  The abort action status.
    ///    *  Using the defined consts(ABORT_XXX) in nsIPaymentActionResponse.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t abortStatus;`
    #[inline]
    pub unsafe fn GetAbortStatus(&self, aAbortStatus: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetAbortStatus)(self, aAbortStatus)
    }


    /// ```text
    /// /**
    ///    *  The Initial method.
    ///    *  @param aRequestId - the request identifier of payment request.
    ///    *  @param aAbortStatus - the abort action result.
    ///    */
    /// ```
    ///

    /// `void init (in AString aRequestId, in uint32_t aAbortStatus);`
    #[inline]
    pub unsafe fn Init(&self, aRequestId: *const ::nsstring::nsAString, aAbortStatus: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aRequestId, aAbortStatus)
    }


    /// ```text
    /// /**
    ///    *  Check if the abort action is succeeded
    ///    */
    /// ```
    ///

    /// `bool isSucceeded ();`
    #[inline]
    pub unsafe fn IsSucceeded(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSucceeded)(self, _retval)
    }


}


/// `interface nsIPaymentCompleteActionResponse : nsIPaymentActionResponse`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentCompleteActionResponse {
    vtable: *const nsIPaymentCompleteActionResponseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentCompleteActionResponse.
unsafe impl XpCom for nsIPaymentCompleteActionResponse {
    const IID: nsIID = nsID(0x62c01e69, 0x9ca4, 0x4060,
        [0x99, 0xe4, 0xb9, 0x5f, 0x62, 0x8c, 0x8e, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentCompleteActionResponse {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentCompleteActionResponse.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentCompleteActionResponseCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentCompleteActionResponse`.
    fn coerce_from(v: &nsIPaymentCompleteActionResponse) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentCompleteActionResponseCoerce for nsIPaymentCompleteActionResponse {
    #[inline]
    fn coerce_from(v: &nsIPaymentCompleteActionResponse) -> &Self {
        v
    }
}

impl nsIPaymentCompleteActionResponse {
    /// Cast this `nsIPaymentCompleteActionResponse` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentCompleteActionResponseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentCompleteActionResponse {
    type Target = nsIPaymentActionResponse;
    #[inline]
    fn deref(&self) -> &nsIPaymentActionResponse {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPaymentActionResponseCoerce> nsIPaymentCompleteActionResponseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentCompleteActionResponse) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentCompleteActionResponse
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentCompleteActionResponseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPaymentActionResponseVTable,

    /* readonly attribute uint32_t completeStatus; */
    pub GetCompleteStatus: unsafe extern "system" fn (this: *const nsIPaymentCompleteActionResponse, aCompleteStatus: *mut uint32_t) -> ::nserror::nsresult,

    /* void init (in AString aRequestId, in uint32_t aCompleteStatus); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentCompleteActionResponse, aRequestId: *const ::nsstring::nsAString, aCompleteStatus: uint32_t) -> ::nserror::nsresult,

    /* bool isCompleted (); */
    pub IsCompleted: unsafe extern "system" fn (this: *const nsIPaymentCompleteActionResponse, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentCompleteActionResponse {

    /// ```text
    /// /**
    ///    *  The complete action status.
    ///    *  Using the defined consts(COMPLETE_XXX) in nsIPaymentActionResponse.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t completeStatus;`
    #[inline]
    pub unsafe fn GetCompleteStatus(&self, aCompleteStatus: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCompleteStatus)(self, aCompleteStatus)
    }


    /// ```text
    /// /**
    ///    *  The Initial method.
    ///    *  @param aRequestId - the request identifier of payment request.
    ///    *  @param aCompleteStatus - the complete action result.
    ///    */
    /// ```
    ///

    /// `void init (in AString aRequestId, in uint32_t aCompleteStatus);`
    #[inline]
    pub unsafe fn Init(&self, aRequestId: *const ::nsstring::nsAString, aCompleteStatus: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aRequestId, aCompleteStatus)
    }


    /// ```text
    /// /**
    ///    *  Check if the complete action is succeeded.
    ///    */
    /// ```
    ///

    /// `bool isCompleted ();`
    #[inline]
    pub unsafe fn IsCompleted(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCompleted)(self, _retval)
    }


}


/// `interface nsIMethodChangeDetails : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMethodChangeDetails {
    vtable: *const nsIMethodChangeDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMethodChangeDetails.
unsafe impl XpCom for nsIMethodChangeDetails {
    const IID: nsIID = nsID(0x2035e0a9, 0xc9ab, 0x4c9f,
        [0xb8, 0xe9, 0x28, 0xb2, 0xed, 0x61, 0x54, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMethodChangeDetails {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMethodChangeDetails.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMethodChangeDetailsCoerce {
    /// Cheaply cast a value of this type from a `nsIMethodChangeDetails`.
    fn coerce_from(v: &nsIMethodChangeDetails) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMethodChangeDetailsCoerce for nsIMethodChangeDetails {
    #[inline]
    fn coerce_from(v: &nsIMethodChangeDetails) -> &Self {
        v
    }
}

impl nsIMethodChangeDetails {
    /// Cast this `nsIMethodChangeDetails` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMethodChangeDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMethodChangeDetails {
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
impl<T: nsISupportsCoerce> nsIMethodChangeDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMethodChangeDetails) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMethodChangeDetails
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMethodChangeDetailsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIMethodChangeDetails, aType: *mut uint32_t) -> ::nserror::nsresult,

    /* void init (in uint32_t aType); */
    pub Init: unsafe extern "system" fn (this: *const nsIMethodChangeDetails, aType: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMethodChangeDetails {
    /// ```text
    /// /**
    ///    *  The consts for representing the method change details data type.
    ///    *  GENERAL_DETAILS is the general purpose details data type. Except basic
    ///    *  card details, all details should belong to this type.
    ///    *  BASICCARD_DETAILS is a special details data type for basic card change
    ///    *  details.
    ///    */
    /// ```
    ///

    pub const GENERAL_DETAILS: i64 = 0;


    pub const BASICCARD_DETAILS: i64 = 1;

    /// ```text
    /// /**
    ///    *  The method change details data type.
    ///    *  Using the above defined consts(GENERAL_DETAILS or BASICCARD_DETAILS).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    *  The initial method.
    ///    *  @param aType - the method change details data type.
    ///    */
    /// ```
    ///

    /// `void init (in uint32_t aType);`
    #[inline]
    pub unsafe fn Init(&self, aType: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aType)
    }


}


/// `interface nsIGeneralChangeDetails : nsIMethodChangeDetails`
///

/// ```text
/// /**
///  * The general purpose method change details.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGeneralChangeDetails {
    vtable: *const nsIGeneralChangeDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGeneralChangeDetails.
unsafe impl XpCom for nsIGeneralChangeDetails {
    const IID: nsIID = nsID(0xe031267e, 0xbec8, 0x4f3c,
        [0xb0, 0xb1, 0x39, 0x6b, 0x77, 0xca, 0x26, 0x0c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGeneralChangeDetails {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGeneralChangeDetails.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGeneralChangeDetailsCoerce {
    /// Cheaply cast a value of this type from a `nsIGeneralChangeDetails`.
    fn coerce_from(v: &nsIGeneralChangeDetails) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGeneralChangeDetailsCoerce for nsIGeneralChangeDetails {
    #[inline]
    fn coerce_from(v: &nsIGeneralChangeDetails) -> &Self {
        v
    }
}

impl nsIGeneralChangeDetails {
    /// Cast this `nsIGeneralChangeDetails` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGeneralChangeDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGeneralChangeDetails {
    type Target = nsIMethodChangeDetails;
    #[inline]
    fn deref(&self) -> &nsIMethodChangeDetails {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIMethodChangeDetailsCoerce> nsIGeneralChangeDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGeneralChangeDetails) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGeneralChangeDetails
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGeneralChangeDetailsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIMethodChangeDetailsVTable,

    /* readonly attribute AString details; */
    pub GetDetails: unsafe extern "system" fn (this: *const nsIGeneralChangeDetails, aDetails: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] void initData (in jsval aDetails); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub InitData: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGeneralChangeDetails {

    /// ```text
    /// /**
    ///    *  The stringified change details.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString details;`
    #[inline]
    pub unsafe fn GetDetails(&self, aDetails: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDetails)(self, aDetails)
    }


    /// ```text
    /// /**
    ///    *  The initial method for nsIGeneralChangeDetails.
    ///    *  @param aData - the javascript object of the content.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void initData (in jsval aDetails);`
    const _InitData: () = ();

}


/// `interface nsIBasicCardChangeDetails : nsIMethodChangeDetails`
///

/// ```text
/// /**
///  *  The basic card change details.
///  *  Since PaymentAddress is an no constructor interface type, UI code can not
///  *  easy create PaymentAddress by calling new PaymentAddress().
///  *  Unfortunately, BasicCardResponse has a PaymentAddress attribute, billingAddress
///  *  , it means UI can not create BsaicCardChangeDetails by calling the init() with a
///  *  given JSObject directly, because PaymentAddress creation in JS code is hard.
///  *  To let UI code can create BasicCardResponse easier, nsIBasicCardResponse is
///  *  provided for UI by passing the raw data of BasicCardResponse,
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBasicCardChangeDetails {
    vtable: *const nsIBasicCardChangeDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBasicCardChangeDetails.
unsafe impl XpCom for nsIBasicCardChangeDetails {
    const IID: nsIID = nsID(0x5296f79e, 0x15ea, 0x40c3,
        [0x81, 0x96, 0x19, 0xcf, 0xa6, 0x4d, 0x32, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBasicCardChangeDetails {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBasicCardChangeDetails.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBasicCardChangeDetailsCoerce {
    /// Cheaply cast a value of this type from a `nsIBasicCardChangeDetails`.
    fn coerce_from(v: &nsIBasicCardChangeDetails) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBasicCardChangeDetailsCoerce for nsIBasicCardChangeDetails {
    #[inline]
    fn coerce_from(v: &nsIBasicCardChangeDetails) -> &Self {
        v
    }
}

impl nsIBasicCardChangeDetails {
    /// Cast this `nsIBasicCardChangeDetails` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBasicCardChangeDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBasicCardChangeDetails {
    type Target = nsIMethodChangeDetails;
    #[inline]
    fn deref(&self) -> &nsIMethodChangeDetails {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIMethodChangeDetailsCoerce> nsIBasicCardChangeDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBasicCardChangeDetails) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBasicCardChangeDetails
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBasicCardChangeDetailsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIMethodChangeDetailsVTable,

    /* readonly attribute nsIPaymentAddress billingAddress; */
    pub GetBillingAddress: unsafe extern "system" fn (this: *const nsIBasicCardChangeDetails, aBillingAddress: *mut *const nsIPaymentAddress) -> ::nserror::nsresult,

    /* void initData (in nsIPaymentAddress billingAddress); */
    pub InitData: unsafe extern "system" fn (this: *const nsIBasicCardChangeDetails, billingAddress: *const nsIPaymentAddress) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBasicCardChangeDetails {

    /// ```text
    /// /**
    ///    *  The billing address.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPaymentAddress billingAddress;`
    #[inline]
    pub unsafe fn GetBillingAddress(&self, aBillingAddress: *mut *const nsIPaymentAddress) -> ::nserror::nsresult {
        ((*self.vtable).GetBillingAddress)(self, aBillingAddress)
    }


    /// ```text
    /// /**
    ///    *  The initial method for nsIBasicCardChangeDetails.
    ///    *  @param aBillingAddreess  - the billing address.
    ///    */
    /// ```
    ///

    /// `void initData (in nsIPaymentAddress billingAddress);`
    #[inline]
    pub unsafe fn InitData(&self, billingAddress: *const nsIPaymentAddress) -> ::nserror::nsresult {
        ((*self.vtable).InitData)(self, billingAddress)
    }


}


