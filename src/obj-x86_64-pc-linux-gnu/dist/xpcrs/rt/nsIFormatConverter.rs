//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFormatConverter.idl
//


/// `interface nsIFormatConverter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormatConverter {
    vtable: *const nsIFormatConverterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormatConverter.
unsafe impl XpCom for nsIFormatConverter {
    const IID: nsIID = nsID(0x948a0023, 0xe3a7, 0x11d2,
        [0x96, 0xcf, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormatConverter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormatConverter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormatConverterCoerce {
    /// Cheaply cast a value of this type from a `nsIFormatConverter`.
    fn coerce_from(v: &nsIFormatConverter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormatConverterCoerce for nsIFormatConverter {
    #[inline]
    fn coerce_from(v: &nsIFormatConverter) -> &Self {
        v
    }
}

impl nsIFormatConverter {
    /// Cast this `nsIFormatConverter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormatConverterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormatConverter {
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
impl<T: nsISupportsCoerce> nsIFormatConverterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormatConverter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormatConverter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormatConverterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Array<ACString> getInputDataFlavors (); */
    pub GetInputDataFlavors: unsafe extern "system" fn (this: *const nsIFormatConverter, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* Array<ACString> getOutputDataFlavors (); */
    pub GetOutputDataFlavors: unsafe extern "system" fn (this: *const nsIFormatConverter, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
    pub CanConvert: unsafe extern "system" fn (this: *const nsIFormatConverter, aFromDataFlavor: *const libc::c_char, aToDataFlavor: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in string aToDataFlavor, out nsISupports aToData); */
    pub Convert: unsafe extern "system" fn (this: *const nsIFormatConverter, aFromDataFlavor: *const libc::c_char, aFromData: *const nsISupports, aToDataFlavor: *const libc::c_char, aToData: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormatConverter {

    /// ```text
    /// /**
    ///     * Get the list of the "input" data flavors (mime types as nsISupportsCString),
    ///     * in otherwords, the flavors that this converter can convert "from" (the
        ///     * incoming data to the converter).
    ///     */
    /// ```
    ///

    /// `Array<ACString> getInputDataFlavors ();`
    #[inline]
    pub unsafe fn GetInputDataFlavors(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetInputDataFlavors)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Get the list of the "output" data flavors (mime types as nsISupportsCString),
    ///     * in otherwords, the flavors that this converter can convert "to" (the
        ///     * outgoing data to the converter).
    ///     *
    ///     * @param  aDataFlavorList fills list with supported flavors
    ///     */
    /// ```
    ///

    /// `Array<ACString> getOutputDataFlavors ();`
    #[inline]
    pub unsafe fn GetOutputDataFlavors(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetOutputDataFlavors)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Determines whether a conversion from one flavor to another is supported
    ///     *
    ///     * @param  aFromFormatConverter flavor to convert from
    ///     * @param  aFromFormatConverter flavor to convert to
    ///     */
    /// ```
    ///

    /// `boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor);`
    #[inline]
    pub unsafe fn CanConvert(&self, aFromDataFlavor: *const libc::c_char, aToDataFlavor: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanConvert)(self, aFromDataFlavor, aToDataFlavor, _retval)
    }


    /// ```text
    /// /**
    ///     * Converts from one flavor to another.
    ///     *
    ///     * @param  aFromFormatConverter flavor to convert from
    ///     * @param  aFromFormatConverter flavor to convert to (destination own the memory)
    ///     * @returns returns NS_OK if it was converted
    ///     */
    /// ```
    ///

    /// `void convert (in string aFromDataFlavor, in nsISupports aFromData, in string aToDataFlavor, out nsISupports aToData);`
    #[inline]
    pub unsafe fn Convert(&self, aFromDataFlavor: *const libc::c_char, aFromData: *const nsISupports, aToDataFlavor: *const libc::c_char, aToData: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Convert)(self, aFromDataFlavor, aFromData, aToDataFlavor, aToData)
    }


}


