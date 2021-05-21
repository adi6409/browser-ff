//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITransferable.idl
//


/// `interface nsIFlavorDataProvider : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFlavorDataProvider {
    vtable: *const nsIFlavorDataProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFlavorDataProvider.
unsafe impl XpCom for nsIFlavorDataProvider {
    const IID: nsIID = nsID(0x7e225e5f, 0x711c, 0x11d7,
        [0x9f, 0xae, 0x00, 0x03, 0x93, 0x63, 0x65, 0x92]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFlavorDataProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFlavorDataProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFlavorDataProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIFlavorDataProvider`.
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFlavorDataProviderCoerce for nsIFlavorDataProvider {
    #[inline]
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self {
        v
    }
}

impl nsIFlavorDataProvider {
    /// Cast this `nsIFlavorDataProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFlavorDataProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFlavorDataProvider {
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
impl<T: nsISupportsCoerce> nsIFlavorDataProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFlavorDataProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFlavorDataProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFlavorDataProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData); */
    pub GetFlavorData: unsafe extern "system" fn (this: *const nsIFlavorDataProvider, aTransferable: *const nsITransferable, aFlavor: *const libc::c_char, aData: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFlavorDataProvider {

    /// ```text
    /// /**
    ///     * Retrieve the data from this data provider.
    ///     *
    ///     * @param  aTransferable (in parameter) the transferable we're being called for.
    ///     * @param  aFlavor (in parameter) the flavor of data to retrieve
    ///     * @param  aData the data. Some variant of class in nsISupportsPrimitives.idl
    ///     */
    /// ```
    ///

    /// `void getFlavorData (in nsITransferable aTransferable, in string aFlavor, out nsISupports aData);`
    #[inline]
    pub unsafe fn GetFlavorData(&self, aTransferable: *const nsITransferable, aFlavor: *const libc::c_char, aData: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetFlavorData)(self, aTransferable, aFlavor, aData)
    }


}


/// `interface nsITransferable : nsISupports`
///

/// ```text
/// /**
///   * nsIFlavorDataProvider allows a flavor to 'promise' data later,
///   * supplying the data lazily.
///   *
///   * To use it, call setTransferData, passing the flavor string and
///   * a nsIFlavorDataProvider QI'd to nsISupports.
///   *
///   * When someone calls getTransferData later, if the data size is
///   * stored as 0, the nsISupports will be QI'd to nsIFlavorDataProvider,
///   * and its getFlavorData called.
///   *
///   */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransferable {
    vtable: *const nsITransferableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransferable.
unsafe impl XpCom for nsITransferable {
    const IID: nsIID = nsID(0x97e0c418, 0x1c1e, 0x4106,
        [0xba, 0xd1, 0x9f, 0xcb, 0x11, 0xdf, 0xf2, 0xfe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransferable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransferable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransferableCoerce {
    /// Cheaply cast a value of this type from a `nsITransferable`.
    fn coerce_from(v: &nsITransferable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransferableCoerce for nsITransferable {
    #[inline]
    fn coerce_from(v: &nsITransferable) -> &Self {
        v
    }
}

impl nsITransferable {
    /// Cast this `nsITransferable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransferableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransferable {
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
impl<T: nsISupportsCoerce> nsITransferableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransferable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransferable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransferableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsILoadContext aContext); */
    pub Init: unsafe extern "system" fn (this: *const nsITransferable, aContext: *const nsILoadContext) -> ::nserror::nsresult,

    /* Array<ACString> flavorsTransferableCanExport (); */
    pub FlavorsTransferableCanExport: unsafe extern "system" fn (this: *const nsITransferable, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [must_use] void getTransferData (in string aFlavor, out nsISupports aData); */
    pub GetTransferData: unsafe extern "system" fn (this: *const nsITransferable, aFlavor: *const libc::c_char, aData: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void getAnyTransferData (out ACString aFlavor, out nsISupports aData); */
    pub GetAnyTransferData: unsafe extern "system" fn (this: *const nsITransferable, aFlavor: *mut ::nsstring::nsACString, aData: *mut *const nsISupports) -> ::nserror::nsresult,

    /* Array<ACString> flavorsTransferableCanImport (); */
    pub FlavorsTransferableCanImport: unsafe extern "system" fn (this: *const nsITransferable, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void setTransferData (in string aFlavor, in nsISupports aData); */
    pub SetTransferData: unsafe extern "system" fn (this: *const nsITransferable, aFlavor: *const libc::c_char, aData: *const nsISupports) -> ::nserror::nsresult,

    /* void addDataFlavor (in string aDataFlavor); */
    pub AddDataFlavor: unsafe extern "system" fn (this: *const nsITransferable, aDataFlavor: *const libc::c_char) -> ::nserror::nsresult,

    /* void removeDataFlavor (in string aDataFlavor); */
    pub RemoveDataFlavor: unsafe extern "system" fn (this: *const nsITransferable, aDataFlavor: *const libc::c_char) -> ::nserror::nsresult,

    /* attribute nsIFormatConverter converter; */
    pub GetConverter: unsafe extern "system" fn (this: *const nsITransferable, aConverter: *mut *const nsIFormatConverter) -> ::nserror::nsresult,

    /* attribute nsIFormatConverter converter; */
    pub SetConverter: unsafe extern "system" fn (this: *const nsITransferable, aConverter: *const nsIFormatConverter) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] attribute boolean isPrivateData; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIsPrivateData: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute boolean isPrivateData; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetIsPrivateData: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsIPrincipal requestingPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetRequestingPrincipal: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsIPrincipal requestingPrincipal; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetRequestingPrincipal: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsContentPolicyType contentPolicyType; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetContentPolicyType: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsContentPolicyType contentPolicyType; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetContentPolicyType: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsICookieJarSettings cookieJarSettings; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetCookieJarSettings: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsICookieJarSettings cookieJarSettings; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetCookieJarSettings: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransferable {

    /// ```text
    /// /**
    ///    * Initializes a transferable object.  This should be called on all
    ///    * transferable objects.  Failure to do so will result in fatal assertions in
    ///    * debug builds.
    ///    *
    ///    * The load context is used to track whether the transferable is storing privacy-
    ///    * sensitive information.
    ///    *
    ///    * To get the appropriate load context in Javascript callers, one needs to get
    ///    * to the document that the transferable corresponds to, and then get the load
    ///    * context from the document like this:
    ///    *
    ///    * var loadContext = doc.defaultView.docShell
    ///    *                                  .QueryInterface(Ci.nsILoadContext);
    ///    *
    ///    * In C++ callers, if you have the corresponding document, you can just call
    ///    * Document::GetLoadContext to get to the load context object.
    ///    *
    ///    * @param aContext the load context associated with the transferable object.
    ///    *        This can be set to null if a load context is not available.
    ///    */
    /// ```
    ///

    /// `void init (in nsILoadContext aContext);`
    #[inline]
    pub unsafe fn Init(&self, aContext: *const nsILoadContext) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aContext)
    }


    /// ```text
    /// /**
    ///     * Computes a list of flavors that the transferable can export, either
    ///     * through intrinsic knowledge or output data converters.
    ///     */
    /// ```
    ///

    /// `Array<ACString> flavorsTransferableCanExport ();`
    #[inline]
    pub unsafe fn FlavorsTransferableCanExport(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).FlavorsTransferableCanExport)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Given a flavor retrieve the data.
    ///     *
    ///     * @param  aFlavor (in parameter) the flavor of data to retrieve
    ///     * @param  aData the data. Some variant of class in nsISupportsPrimitives.idl
    ///     */
    /// ```
    ///

    /// `[must_use] void getTransferData (in string aFlavor, out nsISupports aData);`
    #[inline]
    pub unsafe fn GetTransferData(&self, aFlavor: *const libc::c_char, aData: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetTransferData)(self, aFlavor, aData)
    }


    /// ```text
    /// /**
    ///     * Returns the first flavor which has data.
    ///     *
    ///     * @param  aFlavor (out parameter) the flavor of data that was retrieved
    ///     * @param  aData the data. Some variant of class in nsISupportsPrimitives.idl
    ///     */
    /// ```
    ///

    /// `void getAnyTransferData (out ACString aFlavor, out nsISupports aData);`
    #[inline]
    pub unsafe fn GetAnyTransferData(&self, aFlavor: *mut ::nsstring::nsACString, aData: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetAnyTransferData)(self, aFlavor, aData)
    }


    /// ```text
    /// /**
    ///     * Computes a list of flavors that the transferable can
    ///     * accept into it, either through intrinsic knowledge or input data converters.
    ///     *
    ///     */
    /// ```
    ///

    /// `Array<ACString> flavorsTransferableCanImport ();`
    #[inline]
    pub unsafe fn FlavorsTransferableCanImport(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).FlavorsTransferableCanImport)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Sets the data in the transferable with the specified flavor. The transferable
    ///     * will maintain its own copy the data, so it is not necessary to do that beforehand.
    ///     *
    ///     * @param  aFlavor the flavor of data that is being set
    ///     * @param  aData the data, either some variant of class in nsISupportsPrimitives.idl,
    ///     *         an nsIFile, or an nsIFlavorDataProvider (see above)
    ///     */
    /// ```
    ///

    /// `void setTransferData (in string aFlavor, in nsISupports aData);`
    #[inline]
    pub unsafe fn SetTransferData(&self, aFlavor: *const libc::c_char, aData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetTransferData)(self, aFlavor, aData)
    }


    /// ```text
    /// /**
    ///     * Add the data flavor, indicating that this transferable
    ///     * can receive this type of flavor
    ///     *
    ///     * @param  aDataFlavor a new data flavor to handle
    ///     */
    /// ```
    ///

    /// `void addDataFlavor (in string aDataFlavor);`
    #[inline]
    pub unsafe fn AddDataFlavor(&self, aDataFlavor: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).AddDataFlavor)(self, aDataFlavor)
    }


    /// ```text
    /// /**
    ///     * Removes the data flavor matching the given one (string compare) and the data
    ///     * that goes along with it.
    ///     *
    ///     * @param  aDataFlavor a data flavor to remove
    ///     */
    /// ```
    ///

    /// `void removeDataFlavor (in string aDataFlavor);`
    #[inline]
    pub unsafe fn RemoveDataFlavor(&self, aDataFlavor: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDataFlavor)(self, aDataFlavor)
    }



    /// `attribute nsIFormatConverter converter;`
    #[inline]
    pub unsafe fn GetConverter(&self, aConverter: *mut *const nsIFormatConverter) -> ::nserror::nsresult {
        ((*self.vtable).GetConverter)(self, aConverter)
    }



    /// `attribute nsIFormatConverter converter;`
    #[inline]
    pub unsafe fn SetConverter(&self, aConverter: *const nsIFormatConverter) -> ::nserror::nsresult {
        ((*self.vtable).SetConverter)(self, aConverter)
    }


    /// ```text
    /// /**
    ///    * Use of the SetIsPrivateData() method generated by isPrivateData attribute should
    ///    * be avoided as much as possible because the value set may not reflect the status
    ///    * of the context in which the transferable was created.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute boolean isPrivateData;`
    const _GetIsPrivateData: () = ();

    /// ```text
    /// /**
    ///    * Use of the SetIsPrivateData() method generated by isPrivateData attribute should
    ///    * be avoided as much as possible because the value set may not reflect the status
    ///    * of the context in which the transferable was created.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute boolean isPrivateData;`
    const _SetIsPrivateData: () = ();

    /// ```text
    /// /**
    ///    * The principal of the source dom node this transferable was
    ///    * created from and the contentPolicyType for the transferable.
    ///    * Note, currently only used on Windows for network principal and
    ///    * contentPolicyType information in clipboard and drag operations.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsIPrincipal requestingPrincipal;`
    const _GetRequestingPrincipal: () = ();

    /// ```text
    /// /**
    ///    * The principal of the source dom node this transferable was
    ///    * created from and the contentPolicyType for the transferable.
    ///    * Note, currently only used on Windows for network principal and
    ///    * contentPolicyType information in clipboard and drag operations.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsIPrincipal requestingPrincipal;`
    const _SetRequestingPrincipal: () = ();


    /// `[nostdcall,notxpcom] attribute nsContentPolicyType contentPolicyType;`
    const _GetContentPolicyType: () = ();


    /// `[nostdcall,notxpcom] attribute nsContentPolicyType contentPolicyType;`
    const _SetContentPolicyType: () = ();

    /// ```text
    /// /**
    ///    * The cookieJarSettings of the source dom node this transferable was created
    ///    * from.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsICookieJarSettings cookieJarSettings;`
    const _GetCookieJarSettings: () = ();

    /// ```text
    /// /**
    ///    * The cookieJarSettings of the source dom node this transferable was created
    ///    * from.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsICookieJarSettings cookieJarSettings;`
    const _SetCookieJarSettings: () = ();

}


