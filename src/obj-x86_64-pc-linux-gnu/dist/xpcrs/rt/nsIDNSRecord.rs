//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSRecord.idl
//


/// `interface nsIDNSRecord : nsISupports`
///

/// ```text
/// /**
///  * nsIDNSRecord
///  *
///  * this interface represents the result of a DNS lookup.  since a DNS
///  * query may return more than one resolved IP address, the record acts
///  * like an enumerator, allowing the caller to easily step through the
///  * list of IP addresses.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSRecord {
    vtable: *const nsIDNSRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSRecord.
unsafe impl XpCom for nsIDNSRecord {
    const IID: nsIID = nsID(0xf92228ae, 0xc417, 0x4188,
        [0xa6, 0x04, 0x08, 0x30, 0xa9, 0x5e, 0x7e, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSRecord`.
    fn coerce_from(v: &nsIDNSRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSRecordCoerce for nsIDNSRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSRecord) -> &Self {
        v
    }
}

impl nsIDNSRecord {
    /// Cast this `nsIDNSRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSRecord {
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
impl<T: nsISupportsCoerce> nsIDNSRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSRecord {


}


/// `interface nsIDNSAddrRecord : nsIDNSRecord`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSAddrRecord {
    vtable: *const nsIDNSAddrRecordVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSAddrRecord.
unsafe impl XpCom for nsIDNSAddrRecord {
    const IID: nsIID = nsID(0xcb260e20, 0x943f, 0x4309,
        [0x95, 0x3b, 0x78, 0xc9, 0x0d, 0x3a, 0x76, 0x38]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSAddrRecord {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSAddrRecord.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSAddrRecordCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSAddrRecord`.
    fn coerce_from(v: &nsIDNSAddrRecord) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSAddrRecordCoerce for nsIDNSAddrRecord {
    #[inline]
    fn coerce_from(v: &nsIDNSAddrRecord) -> &Self {
        v
    }
}

impl nsIDNSAddrRecord {
    /// Cast this `nsIDNSAddrRecord` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSAddrRecordCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSAddrRecord {
    type Target = nsIDNSRecord;
    #[inline]
    fn deref(&self) -> &nsIDNSRecord {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDNSRecordCoerce> nsIDNSAddrRecordCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSAddrRecord) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSAddrRecord
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSAddrRecordVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDNSRecordVTable,

    /* readonly attribute ACString canonicalName; */
    pub GetCanonicalName: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aCanonicalName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] NetAddr getNextAddr (in uint16_t aPort); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetNextAddr: *const ::libc::c_void,

    /* [noscript] void getAddresses (out nsNetAddrTArrayRef aAddressArray); */
    /// Unable to generate binding because `native type nsTArray<mozilla::net::NetAddr> unsupported`
    pub GetAddresses: *const ::libc::c_void,

    /* nsINetAddr getScriptableNextAddr (in uint16_t aPort); */
    pub GetScriptableNextAddr: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aPort: uint16_t, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* ACString getNextAddrAsString (); */
    pub GetNextAddrAsString: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean hasMore (); */
    pub HasMore: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, _retval: *mut bool) -> ::nserror::nsresult,

    /* void rewind (); */
    pub Rewind: unsafe extern "system" fn (this: *const nsIDNSAddrRecord) -> ::nserror::nsresult,

    /* void reportUnusable (in uint16_t aPort); */
    pub ReportUnusable: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aPort: uint16_t) -> ::nserror::nsresult,

    /* bool IsTRR (); */
    pub IsTRR: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute double trrFetchDuration; */
    pub GetTrrFetchDuration: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aTrrFetchDuration: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double trrFetchDurationNetworkOnly; */
    pub GetTrrFetchDurationNetworkOnly: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aTrrFetchDurationNetworkOnly: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute unsigned long effectiveTRRMode; */
    pub GetEffectiveTRRMode: unsafe extern "system" fn (this: *const nsIDNSAddrRecord, aEffectiveTRRMode: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSAddrRecord {

    /// ```text
    /// /**
    ///      * @return the canonical hostname for this record.  this value is empty if
    ///      * the record was not fetched with the RESOLVE_CANONICAL_NAME flag.
    ///      *
    ///      * e.g., www.mozilla.org --> rheet.mozilla.org
    ///      *
    ///      * That the result, if IDN will be returned as punycode.
    ///      * e.g., élève.w3c-test.org --> xn--lve-6lad.w3c-test.org
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString canonicalName;`
    #[inline]
    pub unsafe fn GetCanonicalName(&self, aCanonicalName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCanonicalName)(self, aCanonicalName)
    }


    /// ```text
    /// /**
    ///      * this function copies the value of the next IP address into the
    ///      * given NetAddr struct and increments the internal address iterator.
    ///      *
    ///      * @param aPort
    ///      *        A port number to initialize the NetAddr with.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if there is not another IP address in
    ///      * the record.
    ///      */
    /// ```
    ///

    /// `[noscript] NetAddr getNextAddr (in uint16_t aPort);`
    const _GetNextAddr: () = ();

    /// ```text
    /// /**
    ///      * this function copies the value of all working members of the RR
    ///      * set into the output array.
    ///      *
    ///      * @param aAddressArray
    ///      *        The result set
    ///      */
    /// ```
    ///

    /// `[noscript] void getAddresses (out nsNetAddrTArrayRef aAddressArray);`
    const _GetAddresses: () = ();

    /// ```text
    /// /**
    ///      * this function returns the value of the next IP address as a
    ///      * scriptable address and increments the internal address iterator.
    ///      *
    ///      * @param aPort
    ///      *        A port number to initialize the nsINetAddr with.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if there is not another IP address in
    ///      * the record.
    ///      */
    /// ```
    ///

    /// `nsINetAddr getScriptableNextAddr (in uint16_t aPort);`
    #[inline]
    pub unsafe fn GetScriptableNextAddr(&self, aPort: uint16_t, _retval: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptableNextAddr)(self, aPort, _retval)
    }


    /// ```text
    /// /**
    ///      * this function returns the value of the next IP address as a
    ///      * string and increments the internal address iterator.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if there is not another IP address in
    ///      * the record.
    ///      */
    /// ```
    ///

    /// `ACString getNextAddrAsString ();`
    #[inline]
    pub unsafe fn GetNextAddrAsString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNextAddrAsString)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * this function returns true if there is another address in the record.
    ///      */
    /// ```
    ///

    /// `boolean hasMore ();`
    #[inline]
    pub unsafe fn HasMore(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMore)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * this function resets the internal address iterator to the first
    ///      * address in the record.
    ///      */
    /// ```
    ///

    /// `void rewind ();`
    #[inline]
    pub unsafe fn Rewind(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Rewind)(self, )
    }


    /// ```text
    /// /**
    ///      * This function indicates that the last address obtained via getNextAddr*()
    ///      * was not usuable and should be skipped in future uses of this
    ///      * record if other addresses are available.
    ///      *
    ///      * @param aPort is the port number associated with the failure, if any.
    ///      *        It may be zero if not applicable.
    ///      */
    /// ```
    ///

    /// `void reportUnusable (in uint16_t aPort);`
    #[inline]
    pub unsafe fn ReportUnusable(&self, aPort: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).ReportUnusable)(self, aPort)
    }


    /// ```text
    /// /**
    ///      * Record retreived with TRR.
    ///      */
    /// ```
    ///

    /// `bool IsTRR ();`
    #[inline]
    pub unsafe fn IsTRR(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsTRR)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * This attribute is only set if TRR is used and it measures time between
    ///      * asyncOpen on a channel and the time parsing of response if done.
    ///      * Thee time is measured in milliseconds.
    ///      */
    /// ```
    ///

    /// `readonly attribute double trrFetchDuration;`
    #[inline]
    pub unsafe fn GetTrrFetchDuration(&self, aTrrFetchDuration: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetTrrFetchDuration)(self, aTrrFetchDuration)
    }


    /// ```text
    /// /**
    ///      * This attribute is only set if TRR is used and it measures time between
    ///      * sending a request and the time response is received from the network.
    ///      * This time is similat to the time above, but exludes a time needed to
    ///      * make a connection and a time neededto parse results (this also does not
        ///      * include delays that may be introduce because parsing is perform on the main
        ///      * thread).
    ///      * Thee time is measured in milliseconds.
    ///      */
    /// ```
    ///

    /// `readonly attribute double trrFetchDurationNetworkOnly;`
    #[inline]
    pub unsafe fn GetTrrFetchDurationNetworkOnly(&self, aTrrFetchDurationNetworkOnly: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetTrrFetchDurationNetworkOnly)(self, aTrrFetchDurationNetworkOnly)
    }


    /// ```text
    /// /**
    ///      * The TRR mode this record is used.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long effectiveTRRMode;`
    #[inline]
    pub unsafe fn GetEffectiveTRRMode(&self, aEffectiveTRRMode: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEffectiveTRRMode)(self, aEffectiveTRRMode)
    }


}


