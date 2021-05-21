//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteResult.idl
//


/// `interface nsIAutoCompleteResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteResult {
    vtable: *const nsIAutoCompleteResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteResult.
unsafe impl XpCom for nsIAutoCompleteResult {
    const IID: nsIID = nsID(0x9203c031, 0xc4e7, 0x4537,
        [0xa4, 0xec, 0x81, 0x44, 0x3d, 0x62, 0x3d, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteResultCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteResult`.
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteResultCoerce for nsIAutoCompleteResult {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self {
        v
    }
}

impl nsIAutoCompleteResult {
    /// Cast this `nsIAutoCompleteResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteResult {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString searchString; */
    pub GetSearchString: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short searchResult; */
    pub GetSearchResult: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, aSearchResult: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute long defaultIndex; */
    pub GetDefaultIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, aDefaultIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AString errorDescription; */
    pub GetErrorDescription: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, aErrorDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long matchCount; */
    pub GetMatchCount: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, aMatchCount: *mut u32) -> ::nserror::nsresult,

    /* AString getValueAt (in long index); */
    pub GetValueAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getLabelAt (in long index); */
    pub GetLabelAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getCommentAt (in long index); */
    pub GetCommentAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getStyleAt (in long index); */
    pub GetStyleAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getImageAt (in long index); */
    pub GetImageAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getFinalCompleteValueAt (in long index); */
    pub GetFinalCompleteValueAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeValueAt (in long rowIndex); */
    pub RemoveValueAt: unsafe extern "system" fn (this: *const nsIAutoCompleteResult, rowIndex: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteResult {
    /// ```text
    /// /**
    ///    * Possible values for the searchResult attribute
    ///    */
    /// ```
    ///

    pub const RESULT_IGNORED: i64 = 1;


    pub const RESULT_FAILURE: i64 = 2;


    pub const RESULT_NOMATCH: i64 = 3;


    pub const RESULT_SUCCESS: i64 = 4;


    pub const RESULT_NOMATCH_ONGOING: i64 = 5;


    pub const RESULT_SUCCESS_ONGOING: i64 = 6;

    /// ```text
    /// /**
    ///    * The original search string
    ///    */
    /// ```
    ///

    /// `readonly attribute AString searchString;`
    #[inline]
    pub unsafe fn GetSearchString(&self, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchString)(self, aSearchString)
    }


    /// ```text
    /// /**
    ///    * The result of the search
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned short searchResult;`
    #[inline]
    pub unsafe fn GetSearchResult(&self, aSearchResult: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchResult)(self, aSearchResult)
    }


    /// ```text
    /// /**
    ///    * Index of the default item that should be entered if none is selected
    ///    */
    /// ```
    ///

    /// `readonly attribute long defaultIndex;`
    #[inline]
    pub unsafe fn GetDefaultIndex(&self, aDefaultIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultIndex)(self, aDefaultIndex)
    }


    /// ```text
    /// /**
    ///    * A string describing the cause of a search failure
    ///    */
    /// ```
    ///

    /// `readonly attribute AString errorDescription;`
    #[inline]
    pub unsafe fn GetErrorDescription(&self, aErrorDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorDescription)(self, aErrorDescription)
    }


    /// ```text
    /// /**
    ///    * The number of matches
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long matchCount;`
    #[inline]
    pub unsafe fn GetMatchCount(&self, aMatchCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchCount)(self, aMatchCount)
    }


    /// ```text
    /// /**
    ///    * Get the value of the result at the given index
    ///    */
    /// ```
    ///

    /// `AString getValueAt (in long index);`
    #[inline]
    pub unsafe fn GetValueAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValueAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * This returns the string that is displayed in the dropdown
    ///    */
    /// ```
    ///

    /// `AString getLabelAt (in long index);`
    #[inline]
    pub unsafe fn GetLabelAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabelAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the comment of the result at the given index
    ///    */
    /// ```
    ///

    /// `AString getCommentAt (in long index);`
    #[inline]
    pub unsafe fn GetCommentAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCommentAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the style hint for the result at the given index
    ///    */
    /// ```
    ///

    /// `AString getStyleAt (in long index);`
    #[inline]
    pub unsafe fn GetStyleAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStyleAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the image of the result at the given index
    ///    */
    /// ```
    ///

    /// `AString getImageAt (in long index);`
    #[inline]
    pub unsafe fn GetImageAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetImageAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the final value that should be completed when the user confirms
    ///    * the match at the given index.
    ///    */
    /// ```
    ///

    /// `AString getFinalCompleteValueAt (in long index);`
    #[inline]
    pub unsafe fn GetFinalCompleteValueAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFinalCompleteValueAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove the value at the given index from the autocomplete results.
    ///    */
    /// ```
    ///

    /// `void removeValueAt (in long rowIndex);`
    #[inline]
    pub unsafe fn RemoveValueAt(&self, rowIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveValueAt)(self, rowIndex)
    }


}


