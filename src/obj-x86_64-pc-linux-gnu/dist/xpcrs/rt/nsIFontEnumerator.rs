//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/gfx/src/nsIFontEnumerator.idl
//


/// `interface nsIFontEnumerator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFontEnumerator {
    vtable: *const nsIFontEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFontEnumerator.
unsafe impl XpCom for nsIFontEnumerator {
    const IID: nsIID = nsID(0x924d98d9, 0x3518, 0x4cb4,
        [0x87, 0x08, 0xc7, 0x4f, 0xe8, 0xe3, 0xec, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFontEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFontEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFontEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIFontEnumerator`.
    fn coerce_from(v: &nsIFontEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFontEnumeratorCoerce for nsIFontEnumerator {
    #[inline]
    fn coerce_from(v: &nsIFontEnumerator) -> &Self {
        v
    }
}

impl nsIFontEnumerator {
    /// Cast this `nsIFontEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFontEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFontEnumerator {
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
impl<T: nsISupportsCoerce> nsIFontEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFontEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFontEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFontEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Array<AString> EnumerateAllFonts (); */
    pub EnumerateAllFonts: unsafe extern "system" fn (this: *const nsIFontEnumerator, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* Array<AString> EnumerateFonts (in string aLangGroup, in string aGeneric); */
    pub EnumerateFonts: unsafe extern "system" fn (this: *const nsIFontEnumerator, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval EnumerateAllFontsAsync (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub EnumerateAllFontsAsync: *const ::libc::c_void,

    /* [implicit_jscontext] jsval EnumerateFontsAsync (in string aLangGroup, in string aGeneric); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub EnumerateFontsAsync: *const ::libc::c_void,

    /* void HaveFontFor (in string aLangGroup, [retval] out boolean aResult); */
    pub HaveFontFor: unsafe extern "system" fn (this: *const nsIFontEnumerator, aLangGroup: *const libc::c_char, aResult: *mut bool) -> ::nserror::nsresult,

    /* wstring getDefaultFont (in string aLangGroup, in string aGeneric); */
    pub GetDefaultFont: unsafe extern "system" fn (this: *const nsIFontEnumerator, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char, _retval: *mut *const i16) -> ::nserror::nsresult,

    /* boolean updateFontList (); */
    pub UpdateFontList: unsafe extern "system" fn (this: *const nsIFontEnumerator, _retval: *mut bool) -> ::nserror::nsresult,

    /* wstring getStandardFamilyName (in wstring aName); */
    pub GetStandardFamilyName: unsafe extern "system" fn (this: *const nsIFontEnumerator, aName: *const i16, _retval: *mut *const i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFontEnumerator {

    /// ```text
    /// /**
    ///    * Return a sorted array of the names of all installed fonts.
    ///    *
    ///    * @return array of names
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `Array<AString> EnumerateAllFonts ();`
    #[inline]
    pub unsafe fn EnumerateAllFonts(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateAllFonts)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a sorted array of names of fonts that support the given language
    ///    * group and are suitable for use as the given CSS generic font.
    ///    *
    ///    * @param  aLangGroup language group
    ///    * @param  aGeneric   CSS generic font
    ///    * @return            array of names
    ///    * @return void
    ///    */
    /// ```
    ///

    /// `Array<AString> EnumerateFonts (in string aLangGroup, in string aGeneric);`
    #[inline]
    pub unsafe fn EnumerateFonts(&self, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateFonts)(self, aLangGroup, aGeneric, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a promise that resolves to a sorted array of the names of all
    ///    * installed fonts.
    ///    *
    ///    * @return Promise that resolves to Array
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval EnumerateAllFontsAsync ();`
    const _EnumerateAllFontsAsync: () = ();

    /// ```text
    /// /**
    ///    * Return a promise that resolves to a sorted array of names of fonts
    ///    * that support the given language group and are suitable for use as the given
    ///    * CSS generic font.
    ///    *
    ///    * @param  aLangGroup language group
    ///    * @param  aGeneric   CSS generic font
    ///    * @return Promise that resolves to Array
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval EnumerateFontsAsync (in string aLangGroup, in string aGeneric);`
    const _EnumerateFontsAsync: () = ();

    /// ```text
    /// /**
    ///     @param  aLangGroup language group
    ///     @return bool do we have a font for this language group
    ///    */
    /// ```
    ///

    /// `void HaveFontFor (in string aLangGroup, [retval] out boolean aResult);`
    #[inline]
    pub unsafe fn HaveFontFor(&self, aLangGroup: *const libc::c_char, aResult: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HaveFontFor)(self, aLangGroup, aResult)
    }


    /// ```text
    /// /**
    ///    * @param  aLangGroup language group
    ///    * @param  aGeneric CSS generic font
    ///    * @return suggested default font for this language group and generic family
    ///    */
    /// ```
    ///

    /// `wstring getDefaultFont (in string aLangGroup, in string aGeneric);`
    #[inline]
    pub unsafe fn GetDefaultFont(&self, aLangGroup: *const libc::c_char, aGeneric: *const libc::c_char, _retval: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultFont)(self, aLangGroup, aGeneric, _retval)
    }


    /// ```text
    /// /**
    ///    * update the global font list
    ///    * return true if font list is changed
    ///    */
    /// ```
    ///

    /// `boolean updateFontList ();`
    #[inline]
    pub unsafe fn UpdateFontList(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).UpdateFontList)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * get the standard family name on the system from given family
    ///    * @param  aName family name which may be alias
    ///    * @return the standard family name on the system, if given name does not
    ///    *         exist, returns empty string
    ///    */
    /// ```
    ///

    /// `wstring getStandardFamilyName (in wstring aName);`
    #[inline]
    pub unsafe fn GetStandardFamilyName(&self, aName: *const i16, _retval: *mut *const i16) -> ::nserror::nsresult {
        ((*self.vtable).GetStandardFamilyName)(self, aName, _retval)
    }


}


