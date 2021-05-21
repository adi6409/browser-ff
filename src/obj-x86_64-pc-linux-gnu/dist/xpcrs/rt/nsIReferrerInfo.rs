//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/security/nsIReferrerInfo.idl
//


/// `interface nsIReferrerInfo : nsISerializable`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIReferrerInfo {
    vtable: *const nsIReferrerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIReferrerInfo.
unsafe impl XpCom for nsIReferrerInfo {
    const IID: nsIID = nsID(0x081cdc36, 0xf2e2, 0x4f94,
        [0x87, 0xbf, 0x78, 0x57, 0x8f, 0x06, 0xf1, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIReferrerInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIReferrerInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIReferrerInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIReferrerInfo`.
    fn coerce_from(v: &nsIReferrerInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIReferrerInfoCoerce for nsIReferrerInfo {
    #[inline]
    fn coerce_from(v: &nsIReferrerInfo) -> &Self {
        v
    }
}

impl nsIReferrerInfo {
    /// Cast this `nsIReferrerInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIReferrerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIReferrerInfo {
    type Target = nsISerializable;
    #[inline]
    fn deref(&self) -> &nsISerializable {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISerializableCoerce> nsIReferrerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIReferrerInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIReferrerInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIReferrerInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISerializableVTable,

    /* [infallible] readonly attribute nsIURI originalReferrer; */
    pub GetOriginalReferrer: unsafe extern "system" fn (this: *const nsIReferrerInfo, aOriginalReferrer: *mut*const nsIURI) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute nsIReferrerInfo_ReferrerPolicyIDL referrerPolicy; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetReferrerPolicy: *const ::libc::c_void,

    /* [binaryname(ReferrerPolicy),noscript,nostdcall,notxpcom] ReferrerPolicy binaryReferrerPolicy (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ReferrerPolicy: *const ::libc::c_void,

    /* ACString getReferrerPolicyString (); */
    pub GetReferrerPolicyString: unsafe extern "system" fn (this: *const nsIReferrerInfo, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean sendReferrer; */
    pub GetSendReferrer: unsafe extern "system" fn (this: *const nsIReferrerInfo, aSendReferrer: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString computedReferrerSpec; */
    pub GetComputedReferrerSpec: unsafe extern "system" fn (this: *const nsIReferrerInfo, aComputedReferrerSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use,noscript,nostdcall,notxpcom] URIRef GetComputedReferrer (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetComputedReferrer: *const ::libc::c_void,

    /* boolean equals (in nsIReferrerInfo other); */
    pub Equals: unsafe extern "system" fn (this: *const nsIReferrerInfo, other: *const nsIReferrerInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* void init (in nsIReferrerInfo_ReferrerPolicyIDL aReferrerPolicy, [optional] in boolean aSendReferrer, [optional] in nsIURI aOriginalReferrer); */
    pub Init: unsafe extern "system" fn (this: *const nsIReferrerInfo, aReferrerPolicy:  u8, aSendReferrer: bool, aOriginalReferrer: *const nsIURI) -> ::nserror::nsresult,

    /* void initWithDocument ([const] in Document aDocument); */
    pub InitWithDocument: unsafe extern "system" fn (this: *const nsIReferrerInfo, aDocument: *const libc::c_void) -> ::nserror::nsresult,

    /* void initWithElement ([const] in Element aNode); */
    pub InitWithElement: unsafe extern "system" fn (this: *const nsIReferrerInfo, aNode: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIReferrerInfo {

    /// ```text
    /// /**
    ///    * The original referrer URI which indicates the full referrer before applying
    ///    * referrer policy
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute nsIURI originalReferrer;`
    #[inline]
    pub unsafe fn GetOriginalReferrer(&self, aOriginalReferrer: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetOriginalReferrer)(self, aOriginalReferrer)
    }


    /// ```text
    /// /**
    ///    * Referrer policy which is applied to the referrer
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute nsIReferrerInfo_ReferrerPolicyIDL referrerPolicy;`
    const _GetReferrerPolicy: () = ();

    /// ```text
    /// /**
    ///    * C++ friendly version of referrerPolicy getter
    ///    */
    /// ```
    ///

    /// `[binaryname(ReferrerPolicy),noscript,nostdcall,notxpcom] ReferrerPolicy binaryReferrerPolicy ();`
    const _ReferrerPolicy: () = ();

    /// ```text
    /// /**
    ///    * Get referrer policy as string
    ///    */
    /// ```
    ///

    /// `ACString getReferrerPolicyString ();`
    #[inline]
    pub unsafe fn GetReferrerPolicyString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerPolicyString)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Indicates if the referrer should not be sent or not even when it's available.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean sendReferrer;`
    #[inline]
    pub unsafe fn GetSendReferrer(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSendReferrer)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///   * Indicates if the referrer should not be sent or not even when it's available.
    ///   */
    /// ```
    ///

    /// `readonly attribute AString computedReferrerSpec;`
    #[inline]
    pub unsafe fn GetComputedReferrerSpec(&self, aComputedReferrerSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetComputedReferrerSpec)(self, aComputedReferrerSpec)
    }


    /// ```text
    /// /**
    ///    * Get the computed referrer, if one has been set. The computed referrer is
    ///    * the original referrer manipulated by the referrer-policy. Use the result of
    ///    * this function as the actual referrer value for the channel.
    ///    */
    /// ```
    ///

    /// `[must_use,noscript,nostdcall,notxpcom] URIRef GetComputedReferrer ();`
    const _GetComputedReferrer: () = ();

    /// ```text
    /// /**
    ///    * Returns whether the other referrerInfo is equivalent to this referrerInfo.
    ///    */
    /// ```
    ///

    /// `boolean equals (in nsIReferrerInfo other);`
    #[inline]
    pub unsafe fn Equals(&self, other: *const nsIReferrerInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///    * Initialize method to create ReferrerInfo object from JS
    ///    * @param aReferrerPolicy referrer policy of the created object
    ///    * @param aSendReferrer sendReferrer of the created object, defaults to false
    ///    * @param aOriginalReferrer the original referrer, defaults to null.
    ///    */
    /// ```
    ///

    /// `void init (in nsIReferrerInfo_ReferrerPolicyIDL aReferrerPolicy, [optional] in boolean aSendReferrer, [optional] in nsIURI aOriginalReferrer);`
    #[inline]
    pub unsafe fn Init(&self, aReferrerPolicy:  u8, aSendReferrer: bool, aOriginalReferrer: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aReferrerPolicy, aSendReferrer, aOriginalReferrer)
    }


    /// ```text
    /// /**
    ///    * Initialize with a given document.
    ///    * @param aDocument the document to init referrerInfo object
    ///    */
    /// ```
    ///

    /// `void initWithDocument ([const] in Document aDocument);`
    #[inline]
    pub unsafe fn InitWithDocument(&self, aDocument: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).InitWithDocument)(self, aDocument)
    }


    /// ```text
    /// /**
    ///    * Initialize with a given node. It you are working with node which supports
    ///    * referrerpolicy attribute: <a>, <img>, <area>, <script>, <iframe>, please
    ///    * try to use this init instead of initWithDocument, because referrer policy
    ///    * from rel and attribute has a higher priority.
    ///    * @param aNode the element to init referrerInfo object
    ///    */
    /// ```
    ///

    /// `void initWithElement ([const] in Element aNode);`
    #[inline]
    pub unsafe fn InitWithElement(&self, aNode: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).InitWithElement)(self, aNode)
    }


}


