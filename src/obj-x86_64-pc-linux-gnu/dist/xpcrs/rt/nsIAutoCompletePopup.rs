//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompletePopup.idl
//


/// `interface nsIAutoCompletePopup : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompletePopup {
    vtable: *const nsIAutoCompletePopupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompletePopup.
unsafe impl XpCom for nsIAutoCompletePopup {
    const IID: nsIID = nsID(0xbd3c2662, 0xa988, 0x41ab,
        [0x8c, 0x94, 0xc1, 0x5e, 0xd0, 0xe6, 0xac, 0x7d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompletePopup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompletePopup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompletePopupCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompletePopup`.
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompletePopupCoerce for nsIAutoCompletePopup {
    #[inline]
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self {
        v
    }
}

impl nsIAutoCompletePopup {
    /// Cast this `nsIAutoCompletePopup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompletePopupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompletePopup {
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
impl<T: nsISupportsCoerce> nsIAutoCompletePopupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompletePopup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompletePopup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompletePopupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAutoCompleteInput input; */
    pub GetInput: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, aInput: *mut*const nsIAutoCompleteInput) -> ::nserror::nsresult,

    /* readonly attribute AString overrideValue; */
    pub GetOverrideValue: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, aOverrideValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute long selectedIndex; */
    pub GetSelectedIndex: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, aSelectedIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long selectedIndex; */
    pub SetSelectedIndex: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, aSelectedIndex: i32) -> ::nserror::nsresult,

    /* readonly attribute boolean popupOpen; */
    pub GetPopupOpen: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, aPopupOpen: *mut bool) -> ::nserror::nsresult,

    /* void openAutocompletePopup (in nsIAutoCompleteInput input, in Element element); */
    pub OpenAutocompletePopup: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, input: *const nsIAutoCompleteInput, element: *const libc::c_void) -> ::nserror::nsresult,

    /* void closePopup (); */
    pub ClosePopup: unsafe extern "system" fn (this: *const nsIAutoCompletePopup) -> ::nserror::nsresult,

    /* void invalidate (in unsigned short reason); */
    pub Invalidate: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, reason: u16) -> ::nserror::nsresult,

    /* void selectBy (in boolean reverse, in boolean page); */
    pub SelectBy: unsafe extern "system" fn (this: *const nsIAutoCompletePopup, reverse: bool, page: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompletePopup {

    pub const INVALIDATE_REASON_NEW_RESULT: i64 = 0;


    pub const INVALIDATE_REASON_DELETE: i64 = 1;


    /// `readonly attribute nsIAutoCompleteInput input;`
    #[inline]
    pub unsafe fn GetInput(&self, aInput: *mut*const nsIAutoCompleteInput) -> ::nserror::nsresult {
        ((*self.vtable).GetInput)(self, aInput)
    }



    /// `readonly attribute AString overrideValue;`
    #[inline]
    pub unsafe fn GetOverrideValue(&self, aOverrideValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOverrideValue)(self, aOverrideValue)
    }



    /// `attribute long selectedIndex;`
    #[inline]
    pub unsafe fn GetSelectedIndex(&self, aSelectedIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectedIndex)(self, aSelectedIndex)
    }



    /// `attribute long selectedIndex;`
    #[inline]
    pub unsafe fn SetSelectedIndex(&self, aSelectedIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectedIndex)(self, aSelectedIndex)
    }



    /// `readonly attribute boolean popupOpen;`
    #[inline]
    pub unsafe fn GetPopupOpen(&self, aPopupOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPopupOpen)(self, aPopupOpen)
    }



    /// `void openAutocompletePopup (in nsIAutoCompleteInput input, in Element element);`
    #[inline]
    pub unsafe fn OpenAutocompletePopup(&self, input: *const nsIAutoCompleteInput, element: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).OpenAutocompletePopup)(self, input, element)
    }



    /// `void closePopup ();`
    #[inline]
    pub unsafe fn ClosePopup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClosePopup)(self, )
    }



    /// `void invalidate (in unsigned short reason);`
    #[inline]
    pub unsafe fn Invalidate(&self, reason: u16) -> ::nserror::nsresult {
        ((*self.vtable).Invalidate)(self, reason)
    }



    /// `void selectBy (in boolean reverse, in boolean page);`
    #[inline]
    pub unsafe fn SelectBy(&self, reverse: bool, page: bool) -> ::nserror::nsresult {
        ((*self.vtable).SelectBy)(self, reverse, page)
    }


}


