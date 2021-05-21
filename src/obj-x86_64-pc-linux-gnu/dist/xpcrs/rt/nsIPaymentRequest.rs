//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentRequest.idl
//


/// `interface nsIPaymentMethodData : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentMethodData {
    vtable: *const nsIPaymentMethodDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentMethodData.
unsafe impl XpCom for nsIPaymentMethodData {
    const IID: nsIID = nsID(0x2fe296cc, 0xd917, 0x4820,
        [0xb4, 0x92, 0xaa, 0x42, 0xdf, 0x23, 0xf9, 0xb4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentMethodData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentMethodData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentMethodDataCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentMethodData`.
    fn coerce_from(v: &nsIPaymentMethodData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentMethodDataCoerce for nsIPaymentMethodData {
    #[inline]
    fn coerce_from(v: &nsIPaymentMethodData) -> &Self {
        v
    }
}

impl nsIPaymentMethodData {
    /// Cast this `nsIPaymentMethodData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentMethodDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentMethodData {
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
impl<T: nsISupportsCoerce> nsIPaymentMethodDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentMethodData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentMethodData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentMethodDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString supportedMethods; */
    pub GetSupportedMethods: unsafe extern "system" fn (this: *const nsIPaymentMethodData, aSupportedMethods: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval data; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetData: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentMethodData {


    /// `readonly attribute AString supportedMethods;`
    #[inline]
    pub unsafe fn GetSupportedMethods(&self, aSupportedMethods: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedMethods)(self, aSupportedMethods)
    }



    /// `[implicit_jscontext] readonly attribute jsval data;`
    const _GetData: () = ();

}


/// `interface nsIPaymentCurrencyAmount : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentCurrencyAmount {
    vtable: *const nsIPaymentCurrencyAmountVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentCurrencyAmount.
unsafe impl XpCom for nsIPaymentCurrencyAmount {
    const IID: nsIID = nsID(0xd22a6f5f, 0x767b, 0x4fea,
        [0xbf, 0x92, 0x68, 0xb0, 0xb8, 0x00, 0x3e, 0xba]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentCurrencyAmount {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentCurrencyAmount.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentCurrencyAmountCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentCurrencyAmount`.
    fn coerce_from(v: &nsIPaymentCurrencyAmount) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentCurrencyAmountCoerce for nsIPaymentCurrencyAmount {
    #[inline]
    fn coerce_from(v: &nsIPaymentCurrencyAmount) -> &Self {
        v
    }
}

impl nsIPaymentCurrencyAmount {
    /// Cast this `nsIPaymentCurrencyAmount` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentCurrencyAmountCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentCurrencyAmount {
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
impl<T: nsISupportsCoerce> nsIPaymentCurrencyAmountCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentCurrencyAmount) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentCurrencyAmount
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentCurrencyAmountVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString currency; */
    pub GetCurrency: unsafe extern "system" fn (this: *const nsIPaymentCurrencyAmount, aCurrency: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIPaymentCurrencyAmount, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentCurrencyAmount {


    /// `readonly attribute AString currency;`
    #[inline]
    pub unsafe fn GetCurrency(&self, aCurrency: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrency)(self, aCurrency)
    }



    /// `readonly attribute AString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


}


/// `interface nsIPaymentItem : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentItem {
    vtable: *const nsIPaymentItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentItem.
unsafe impl XpCom for nsIPaymentItem {
    const IID: nsIID = nsID(0x4f78a59f, 0xb5ff, 0x4fb5,
        [0xab, 0x48, 0x3b, 0x37, 0xd0, 0x10, 0x1b, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentItemCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentItem`.
    fn coerce_from(v: &nsIPaymentItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentItemCoerce for nsIPaymentItem {
    #[inline]
    fn coerce_from(v: &nsIPaymentItem) -> &Self {
        v
    }
}

impl nsIPaymentItem {
    /// Cast this `nsIPaymentItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentItem {
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
impl<T: nsISupportsCoerce> nsIPaymentItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString label; */
    pub GetLabel: unsafe extern "system" fn (this: *const nsIPaymentItem, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentCurrencyAmount amount; */
    pub GetAmount: unsafe extern "system" fn (this: *const nsIPaymentItem, aAmount: *mut *const nsIPaymentCurrencyAmount) -> ::nserror::nsresult,

    /* readonly attribute boolean pending; */
    pub GetPending: unsafe extern "system" fn (this: *const nsIPaymentItem, aPending: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentItem {


    /// `readonly attribute AString label;`
    #[inline]
    pub unsafe fn GetLabel(&self, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabel)(self, aLabel)
    }



    /// `readonly attribute nsIPaymentCurrencyAmount amount;`
    #[inline]
    pub unsafe fn GetAmount(&self, aAmount: *mut *const nsIPaymentCurrencyAmount) -> ::nserror::nsresult {
        ((*self.vtable).GetAmount)(self, aAmount)
    }



    /// `readonly attribute boolean pending;`
    #[inline]
    pub unsafe fn GetPending(&self, aPending: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPending)(self, aPending)
    }


}


/// `interface nsIPaymentDetailsModifier : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentDetailsModifier {
    vtable: *const nsIPaymentDetailsModifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentDetailsModifier.
unsafe impl XpCom for nsIPaymentDetailsModifier {
    const IID: nsIID = nsID(0x74259861, 0xc318, 0x40e8,
        [0xb3, 0xd5, 0x51, 0x8e, 0x70, 0x1b, 0xed, 0x80]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentDetailsModifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentDetailsModifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentDetailsModifierCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentDetailsModifier`.
    fn coerce_from(v: &nsIPaymentDetailsModifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentDetailsModifierCoerce for nsIPaymentDetailsModifier {
    #[inline]
    fn coerce_from(v: &nsIPaymentDetailsModifier) -> &Self {
        v
    }
}

impl nsIPaymentDetailsModifier {
    /// Cast this `nsIPaymentDetailsModifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentDetailsModifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentDetailsModifier {
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
impl<T: nsISupportsCoerce> nsIPaymentDetailsModifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentDetailsModifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentDetailsModifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentDetailsModifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString supportedMethods; */
    pub GetSupportedMethods: unsafe extern "system" fn (this: *const nsIPaymentDetailsModifier, aSupportedMethods: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentItem total; */
    pub GetTotal: unsafe extern "system" fn (this: *const nsIPaymentDetailsModifier, aTotal: *mut *const nsIPaymentItem) -> ::nserror::nsresult,

    /* readonly attribute nsIArray additionalDisplayItems; */
    pub GetAdditionalDisplayItems: unsafe extern "system" fn (this: *const nsIPaymentDetailsModifier, aAdditionalDisplayItems: *mut*const nsIArray) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval data; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetData: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentDetailsModifier {


    /// `readonly attribute AString supportedMethods;`
    #[inline]
    pub unsafe fn GetSupportedMethods(&self, aSupportedMethods: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSupportedMethods)(self, aSupportedMethods)
    }



    /// `readonly attribute nsIPaymentItem total;`
    #[inline]
    pub unsafe fn GetTotal(&self, aTotal: *mut *const nsIPaymentItem) -> ::nserror::nsresult {
        ((*self.vtable).GetTotal)(self, aTotal)
    }



    /// `readonly attribute nsIArray additionalDisplayItems;`
    #[inline]
    pub unsafe fn GetAdditionalDisplayItems(&self, aAdditionalDisplayItems: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetAdditionalDisplayItems)(self, aAdditionalDisplayItems)
    }



    /// `[implicit_jscontext] readonly attribute jsval data;`
    const _GetData: () = ();

}


/// `interface nsIPaymentShippingOption : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentShippingOption {
    vtable: *const nsIPaymentShippingOptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentShippingOption.
unsafe impl XpCom for nsIPaymentShippingOption {
    const IID: nsIID = nsID(0x68341551, 0x3605, 0x4381,
        [0xb9, 0x36, 0x41, 0xe8, 0x30, 0xaa, 0x88, 0xfb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentShippingOption {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentShippingOption.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentShippingOptionCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentShippingOption`.
    fn coerce_from(v: &nsIPaymentShippingOption) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentShippingOptionCoerce for nsIPaymentShippingOption {
    #[inline]
    fn coerce_from(v: &nsIPaymentShippingOption) -> &Self {
        v
    }
}

impl nsIPaymentShippingOption {
    /// Cast this `nsIPaymentShippingOption` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentShippingOptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentShippingOption {
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
impl<T: nsISupportsCoerce> nsIPaymentShippingOptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentShippingOption) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentShippingOption
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentShippingOptionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPaymentShippingOption, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString label; */
    pub GetLabel: unsafe extern "system" fn (this: *const nsIPaymentShippingOption, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentCurrencyAmount amount; */
    pub GetAmount: unsafe extern "system" fn (this: *const nsIPaymentShippingOption, aAmount: *mut *const nsIPaymentCurrencyAmount) -> ::nserror::nsresult,

    /* attribute boolean selected; */
    pub GetSelected: unsafe extern "system" fn (this: *const nsIPaymentShippingOption, aSelected: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean selected; */
    pub SetSelected: unsafe extern "system" fn (this: *const nsIPaymentShippingOption, aSelected: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentShippingOption {


    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AString label;`
    #[inline]
    pub unsafe fn GetLabel(&self, aLabel: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabel)(self, aLabel)
    }



    /// `readonly attribute nsIPaymentCurrencyAmount amount;`
    #[inline]
    pub unsafe fn GetAmount(&self, aAmount: *mut *const nsIPaymentCurrencyAmount) -> ::nserror::nsresult {
        ((*self.vtable).GetAmount)(self, aAmount)
    }



    /// `attribute boolean selected;`
    #[inline]
    pub unsafe fn GetSelected(&self, aSelected: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSelected)(self, aSelected)
    }



    /// `attribute boolean selected;`
    #[inline]
    pub unsafe fn SetSelected(&self, aSelected: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSelected)(self, aSelected)
    }


}


/// `interface nsIPaymentDetails : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentDetails {
    vtable: *const nsIPaymentDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentDetails.
unsafe impl XpCom for nsIPaymentDetails {
    const IID: nsIID = nsID(0x73a5a3f1, 0x45b9, 0x4605,
        [0xa6, 0xe6, 0x7a, 0xa6, 0x0d, 0xaa, 0x90, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentDetails {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentDetails.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentDetailsCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentDetails`.
    fn coerce_from(v: &nsIPaymentDetails) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentDetailsCoerce for nsIPaymentDetails {
    #[inline]
    fn coerce_from(v: &nsIPaymentDetails) -> &Self {
        v
    }
}

impl nsIPaymentDetails {
    /// Cast this `nsIPaymentDetails` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentDetails {
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
impl<T: nsISupportsCoerce> nsIPaymentDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentDetails) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentDetails
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentDetailsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPaymentDetails, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentItem totalItem; */
    pub GetTotalItem: unsafe extern "system" fn (this: *const nsIPaymentDetails, aTotalItem: *mut *const nsIPaymentItem) -> ::nserror::nsresult,

    /* readonly attribute nsIArray displayItems; */
    pub GetDisplayItems: unsafe extern "system" fn (this: *const nsIPaymentDetails, aDisplayItems: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIArray shippingOptions; */
    pub GetShippingOptions: unsafe extern "system" fn (this: *const nsIPaymentDetails, aShippingOptions: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIArray modifiers; */
    pub GetModifiers: unsafe extern "system" fn (this: *const nsIPaymentDetails, aModifiers: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute AString error; */
    pub GetError: unsafe extern "system" fn (this: *const nsIPaymentDetails, aError: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval shippingAddressErrors; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetShippingAddressErrors: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval payerErrors; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetPayerErrors: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval paymentMethodErrors; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetPaymentMethodErrors: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentDetails {


    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute nsIPaymentItem totalItem;`
    #[inline]
    pub unsafe fn GetTotalItem(&self, aTotalItem: *mut *const nsIPaymentItem) -> ::nserror::nsresult {
        ((*self.vtable).GetTotalItem)(self, aTotalItem)
    }



    /// `readonly attribute nsIArray displayItems;`
    #[inline]
    pub unsafe fn GetDisplayItems(&self, aDisplayItems: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayItems)(self, aDisplayItems)
    }



    /// `readonly attribute nsIArray shippingOptions;`
    #[inline]
    pub unsafe fn GetShippingOptions(&self, aShippingOptions: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetShippingOptions)(self, aShippingOptions)
    }



    /// `readonly attribute nsIArray modifiers;`
    #[inline]
    pub unsafe fn GetModifiers(&self, aModifiers: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetModifiers)(self, aModifiers)
    }



    /// `readonly attribute AString error;`
    #[inline]
    pub unsafe fn GetError(&self, aError: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetError)(self, aError)
    }



    /// `[implicit_jscontext] readonly attribute jsval shippingAddressErrors;`
    const _GetShippingAddressErrors: () = ();


    /// `[implicit_jscontext] readonly attribute jsval payerErrors;`
    const _GetPayerErrors: () = ();


    /// `[implicit_jscontext] readonly attribute jsval paymentMethodErrors;`
    const _GetPaymentMethodErrors: () = ();

}


/// `interface nsIPaymentOptions : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentOptions {
    vtable: *const nsIPaymentOptionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentOptions.
unsafe impl XpCom for nsIPaymentOptions {
    const IID: nsIID = nsID(0xd53f9f20, 0x138e, 0x47cc,
        [0x9f, 0xd5, 0xdb, 0x16, 0xa3, 0xf6, 0xd3, 0x01]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentOptions {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentOptions.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentOptionsCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentOptions`.
    fn coerce_from(v: &nsIPaymentOptions) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentOptionsCoerce for nsIPaymentOptions {
    #[inline]
    fn coerce_from(v: &nsIPaymentOptions) -> &Self {
        v
    }
}

impl nsIPaymentOptions {
    /// Cast this `nsIPaymentOptions` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentOptionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentOptions {
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
impl<T: nsISupportsCoerce> nsIPaymentOptionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentOptions) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentOptions
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentOptionsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean requestPayerName; */
    pub GetRequestPayerName: unsafe extern "system" fn (this: *const nsIPaymentOptions, aRequestPayerName: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean requestPayerEmail; */
    pub GetRequestPayerEmail: unsafe extern "system" fn (this: *const nsIPaymentOptions, aRequestPayerEmail: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean requestPayerPhone; */
    pub GetRequestPayerPhone: unsafe extern "system" fn (this: *const nsIPaymentOptions, aRequestPayerPhone: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean requestShipping; */
    pub GetRequestShipping: unsafe extern "system" fn (this: *const nsIPaymentOptions, aRequestShipping: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean requestBillingAddress; */
    pub GetRequestBillingAddress: unsafe extern "system" fn (this: *const nsIPaymentOptions, aRequestBillingAddress: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString shippingType; */
    pub GetShippingType: unsafe extern "system" fn (this: *const nsIPaymentOptions, aShippingType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentOptions {


    /// `readonly attribute boolean requestPayerName;`
    #[inline]
    pub unsafe fn GetRequestPayerName(&self, aRequestPayerName: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestPayerName)(self, aRequestPayerName)
    }



    /// `readonly attribute boolean requestPayerEmail;`
    #[inline]
    pub unsafe fn GetRequestPayerEmail(&self, aRequestPayerEmail: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestPayerEmail)(self, aRequestPayerEmail)
    }



    /// `readonly attribute boolean requestPayerPhone;`
    #[inline]
    pub unsafe fn GetRequestPayerPhone(&self, aRequestPayerPhone: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestPayerPhone)(self, aRequestPayerPhone)
    }



    /// `readonly attribute boolean requestShipping;`
    #[inline]
    pub unsafe fn GetRequestShipping(&self, aRequestShipping: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestShipping)(self, aRequestShipping)
    }



    /// `readonly attribute boolean requestBillingAddress;`
    #[inline]
    pub unsafe fn GetRequestBillingAddress(&self, aRequestBillingAddress: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestBillingAddress)(self, aRequestBillingAddress)
    }



    /// `readonly attribute AString shippingType;`
    #[inline]
    pub unsafe fn GetShippingType(&self, aShippingType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetShippingType)(self, aShippingType)
    }


}


/// `interface nsIPaymentRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentRequest {
    vtable: *const nsIPaymentRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentRequest.
unsafe impl XpCom for nsIPaymentRequest {
    const IID: nsIID = nsID(0x2fa36783, 0xd684, 0x4487,
        [0xb7, 0xa8, 0x9d, 0xef, 0x6a, 0xe3, 0x12, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentRequest`.
    fn coerce_from(v: &nsIPaymentRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentRequestCoerce for nsIPaymentRequest {
    #[inline]
    fn coerce_from(v: &nsIPaymentRequest) -> &Self {
        v
    }
}

impl nsIPaymentRequest {
    /// Cast this `nsIPaymentRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentRequest {
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
impl<T: nsISupportsCoerce> nsIPaymentRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint64_t topOuterWindowId; */
    pub GetTopOuterWindowId: unsafe extern "system" fn (this: *const nsIPaymentRequest, aTopOuterWindowId: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal topLevelPrincipal; */
    pub GetTopLevelPrincipal: unsafe extern "system" fn (this: *const nsIPaymentRequest, aTopLevelPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute AString requestId; */
    pub GetRequestId: unsafe extern "system" fn (this: *const nsIPaymentRequest, aRequestId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString completeStatus; */
    pub GetCompleteStatus: unsafe extern "system" fn (this: *const nsIPaymentRequest, aCompleteStatus: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray paymentMethods; */
    pub GetPaymentMethods: unsafe extern "system" fn (this: *const nsIPaymentRequest, aPaymentMethods: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentDetails paymentDetails; */
    pub GetPaymentDetails: unsafe extern "system" fn (this: *const nsIPaymentRequest, aPaymentDetails: *mut *const nsIPaymentDetails) -> ::nserror::nsresult,

    /* readonly attribute nsIPaymentOptions paymentOptions; */
    pub GetPaymentOptions: unsafe extern "system" fn (this: *const nsIPaymentRequest, aPaymentOptions: *mut *const nsIPaymentOptions) -> ::nserror::nsresult,

    /* readonly attribute AString shippingOption; */
    pub GetShippingOption: unsafe extern "system" fn (this: *const nsIPaymentRequest, aShippingOption: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentRequest {


    /// `readonly attribute uint64_t topOuterWindowId;`
    #[inline]
    pub unsafe fn GetTopOuterWindowId(&self, aTopOuterWindowId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTopOuterWindowId)(self, aTopOuterWindowId)
    }



    /// `readonly attribute nsIPrincipal topLevelPrincipal;`
    #[inline]
    pub unsafe fn GetTopLevelPrincipal(&self, aTopLevelPrincipal: *mut *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTopLevelPrincipal)(self, aTopLevelPrincipal)
    }



    /// `readonly attribute AString requestId;`
    #[inline]
    pub unsafe fn GetRequestId(&self, aRequestId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestId)(self, aRequestId)
    }



    /// `readonly attribute AString completeStatus;`
    #[inline]
    pub unsafe fn GetCompleteStatus(&self, aCompleteStatus: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCompleteStatus)(self, aCompleteStatus)
    }



    /// `readonly attribute nsIArray paymentMethods;`
    #[inline]
    pub unsafe fn GetPaymentMethods(&self, aPaymentMethods: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetPaymentMethods)(self, aPaymentMethods)
    }



    /// `readonly attribute nsIPaymentDetails paymentDetails;`
    #[inline]
    pub unsafe fn GetPaymentDetails(&self, aPaymentDetails: *mut *const nsIPaymentDetails) -> ::nserror::nsresult {
        ((*self.vtable).GetPaymentDetails)(self, aPaymentDetails)
    }



    /// `readonly attribute nsIPaymentOptions paymentOptions;`
    #[inline]
    pub unsafe fn GetPaymentOptions(&self, aPaymentOptions: *mut *const nsIPaymentOptions) -> ::nserror::nsresult {
        ((*self.vtable).GetPaymentOptions)(self, aPaymentOptions)
    }



    /// `readonly attribute AString shippingOption;`
    #[inline]
    pub unsafe fn GetShippingOption(&self, aShippingOption: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetShippingOption)(self, aShippingOption)
    }


}


