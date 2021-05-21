//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/payments/nsIPaymentAddress.idl
//


/// `interface nsIPaymentAddress : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaymentAddress {
    vtable: *const nsIPaymentAddressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaymentAddress.
unsafe impl XpCom for nsIPaymentAddress {
    const IID: nsIID = nsID(0x49a02241, 0x7e48, 0x477a,
        [0x93, 0x45, 0x9f, 0x24, 0x69, 0x25, 0xdc, 0xb3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaymentAddress {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaymentAddress.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaymentAddressCoerce {
    /// Cheaply cast a value of this type from a `nsIPaymentAddress`.
    fn coerce_from(v: &nsIPaymentAddress) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaymentAddressCoerce for nsIPaymentAddress {
    #[inline]
    fn coerce_from(v: &nsIPaymentAddress) -> &Self {
        v
    }
}

impl nsIPaymentAddress {
    /// Cast this `nsIPaymentAddress` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaymentAddressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaymentAddress {
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
impl<T: nsISupportsCoerce> nsIPaymentAddressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaymentAddress) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaymentAddress
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaymentAddressVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString country; */
    pub GetCountry: unsafe extern "system" fn (this: *const nsIPaymentAddress, aCountry: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray addressLine; */
    pub GetAddressLine: unsafe extern "system" fn (this: *const nsIPaymentAddress, aAddressLine: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute AString region; */
    pub GetRegion: unsafe extern "system" fn (this: *const nsIPaymentAddress, aRegion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString regionCode; */
    pub GetRegionCode: unsafe extern "system" fn (this: *const nsIPaymentAddress, aRegionCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString city; */
    pub GetCity: unsafe extern "system" fn (this: *const nsIPaymentAddress, aCity: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString dependentLocality; */
    pub GetDependentLocality: unsafe extern "system" fn (this: *const nsIPaymentAddress, aDependentLocality: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString postalCode; */
    pub GetPostalCode: unsafe extern "system" fn (this: *const nsIPaymentAddress, aPostalCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString sortingCode; */
    pub GetSortingCode: unsafe extern "system" fn (this: *const nsIPaymentAddress, aSortingCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString organization; */
    pub GetOrganization: unsafe extern "system" fn (this: *const nsIPaymentAddress, aOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString recipient; */
    pub GetRecipient: unsafe extern "system" fn (this: *const nsIPaymentAddress, aRecipient: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString phone; */
    pub GetPhone: unsafe extern "system" fn (this: *const nsIPaymentAddress, aPhone: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void init (in AString aCountry, in nsIArray aAddressLine, in AString aRegion, in AString aRegionCode, in AString aCity, in AString aDependentLocality, in AString aPostalCode, in AString aSortingCode, in AString aOrganization, in AString aRecipient, in AString aPhone); */
    pub Init: unsafe extern "system" fn (this: *const nsIPaymentAddress, aCountry: *const ::nsstring::nsAString, aAddressLine: *const nsIArray, aRegion: *const ::nsstring::nsAString, aRegionCode: *const ::nsstring::nsAString, aCity: *const ::nsstring::nsAString, aDependentLocality: *const ::nsstring::nsAString, aPostalCode: *const ::nsstring::nsAString, aSortingCode: *const ::nsstring::nsAString, aOrganization: *const ::nsstring::nsAString, aRecipient: *const ::nsstring::nsAString, aPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaymentAddress {


    /// `readonly attribute AString country;`
    #[inline]
    pub unsafe fn GetCountry(&self, aCountry: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCountry)(self, aCountry)
    }



    /// `readonly attribute nsIArray addressLine;`
    #[inline]
    pub unsafe fn GetAddressLine(&self, aAddressLine: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetAddressLine)(self, aAddressLine)
    }



    /// `readonly attribute AString region;`
    #[inline]
    pub unsafe fn GetRegion(&self, aRegion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRegion)(self, aRegion)
    }



    /// `readonly attribute AString regionCode;`
    #[inline]
    pub unsafe fn GetRegionCode(&self, aRegionCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRegionCode)(self, aRegionCode)
    }



    /// `readonly attribute AString city;`
    #[inline]
    pub unsafe fn GetCity(&self, aCity: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCity)(self, aCity)
    }



    /// `readonly attribute AString dependentLocality;`
    #[inline]
    pub unsafe fn GetDependentLocality(&self, aDependentLocality: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDependentLocality)(self, aDependentLocality)
    }



    /// `readonly attribute AString postalCode;`
    #[inline]
    pub unsafe fn GetPostalCode(&self, aPostalCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPostalCode)(self, aPostalCode)
    }



    /// `readonly attribute AString sortingCode;`
    #[inline]
    pub unsafe fn GetSortingCode(&self, aSortingCode: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSortingCode)(self, aSortingCode)
    }



    /// `readonly attribute AString organization;`
    #[inline]
    pub unsafe fn GetOrganization(&self, aOrganization: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrganization)(self, aOrganization)
    }



    /// `readonly attribute AString recipient;`
    #[inline]
    pub unsafe fn GetRecipient(&self, aRecipient: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRecipient)(self, aRecipient)
    }



    /// `readonly attribute AString phone;`
    #[inline]
    pub unsafe fn GetPhone(&self, aPhone: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPhone)(self, aPhone)
    }



    /// `void init (in AString aCountry, in nsIArray aAddressLine, in AString aRegion, in AString aRegionCode, in AString aCity, in AString aDependentLocality, in AString aPostalCode, in AString aSortingCode, in AString aOrganization, in AString aRecipient, in AString aPhone);`
    #[inline]
    pub unsafe fn Init(&self, aCountry: *const ::nsstring::nsAString, aAddressLine: *const nsIArray, aRegion: *const ::nsstring::nsAString, aRegionCode: *const ::nsstring::nsAString, aCity: *const ::nsstring::nsAString, aDependentLocality: *const ::nsstring::nsAString, aPostalCode: *const ::nsstring::nsAString, aSortingCode: *const ::nsstring::nsAString, aOrganization: *const ::nsstring::nsAString, aRecipient: *const ::nsstring::nsAString, aPhone: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aCountry, aAddressLine, aRegion, aRegionCode, aCity, aDependentLocality, aPostalCode, aSortingCode, aOrganization, aRecipient, aPhone)
    }


}


