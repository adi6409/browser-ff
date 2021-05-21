//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteInput.idl
//


/// `interface nsIAutoCompleteInput : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteInput {
    vtable: *const nsIAutoCompleteInputVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteInput.
unsafe impl XpCom for nsIAutoCompleteInput {
    const IID: nsIID = nsID(0xb068e70f, 0xf82c, 0x4c12,
        [0xad, 0x87, 0x82, 0xe2, 0x71, 0xc5, 0xc1, 0x80]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteInput {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteInput.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteInputCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteInput`.
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteInputCoerce for nsIAutoCompleteInput {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self {
        v
    }
}

impl nsIAutoCompleteInput {
    /// Cast this `nsIAutoCompleteInput` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteInputCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteInput {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteInputCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteInput) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteInput
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteInputVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Element popupElement; */
    pub GetPopupElement: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aPopupElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsIAutoCompletePopup popup; */
    pub GetPopup: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aPopup: *mut*const nsIAutoCompletePopup) -> ::nserror::nsresult,

    /* readonly attribute nsIAutoCompleteController controller; */
    pub GetController: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aController: *mut *const nsIAutoCompleteController) -> ::nserror::nsresult,

    /* [can_run_script] attribute boolean popupOpen; */
    pub GetPopupOpen: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aPopupOpen: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] attribute boolean popupOpen; */
    pub SetPopupOpen: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aPopupOpen: bool) -> ::nserror::nsresult,

    /* attribute boolean disableAutoComplete; */
    pub GetDisableAutoComplete: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aDisableAutoComplete: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean disableAutoComplete; */
    pub SetDisableAutoComplete: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aDisableAutoComplete: bool) -> ::nserror::nsresult,

    /* attribute boolean completeDefaultIndex; */
    pub GetCompleteDefaultIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aCompleteDefaultIndex: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean completeDefaultIndex; */
    pub SetCompleteDefaultIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aCompleteDefaultIndex: bool) -> ::nserror::nsresult,

    /* attribute boolean completeSelectedIndex; */
    pub GetCompleteSelectedIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aCompleteSelectedIndex: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean completeSelectedIndex; */
    pub SetCompleteSelectedIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aCompleteSelectedIndex: bool) -> ::nserror::nsresult,

    /* attribute boolean forceComplete; */
    pub GetForceComplete: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aForceComplete: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean forceComplete; */
    pub SetForceComplete: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aForceComplete: bool) -> ::nserror::nsresult,

    /* attribute unsigned long minResultsForPopup; */
    pub GetMinResultsForPopup: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aMinResultsForPopup: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long minResultsForPopup; */
    pub SetMinResultsForPopup: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aMinResultsForPopup: u32) -> ::nserror::nsresult,

    /* attribute unsigned long maxRows; */
    pub GetMaxRows: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aMaxRows: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long maxRows; */
    pub SetMaxRows: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aMaxRows: u32) -> ::nserror::nsresult,

    /* attribute unsigned long timeout; */
    pub GetTimeout: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aTimeout: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long timeout; */
    pub SetTimeout: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aTimeout: u32) -> ::nserror::nsresult,

    /* attribute AString searchParam; */
    pub GetSearchParam: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aSearchParam: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchParam; */
    pub SetSearchParam: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aSearchParam: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long searchCount; */
    pub GetSearchCount: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aSearchCount: *mut u32) -> ::nserror::nsresult,

    /* ACString getSearchAt (in unsigned long index); */
    pub GetSearchAt: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, index: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AString textValue; */
    pub GetTextValue: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aTextValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString textValue; */
    pub SetTextValue: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aTextValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setTextValueWithReason (in AString aValue, in unsigned short aReason); */
    pub SetTextValueWithReason: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aValue: *const ::nsstring::nsAString, aReason: u16) -> ::nserror::nsresult,

    /* readonly attribute long selectionStart; */
    pub GetSelectionStart: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aSelectionStart: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long selectionEnd; */
    pub GetSelectionEnd: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aSelectionEnd: *mut i32) -> ::nserror::nsresult,

    /* void selectTextRange (in long startIndex, in long endIndex); */
    pub SelectTextRange: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, startIndex: i32, endIndex: i32) -> ::nserror::nsresult,

    /* void onSearchBegin (); */
    pub OnSearchBegin: unsafe extern "system" fn (this: *const nsIAutoCompleteInput) -> ::nserror::nsresult,

    /* void onSearchComplete (); */
    pub OnSearchComplete: unsafe extern "system" fn (this: *const nsIAutoCompleteInput) -> ::nserror::nsresult,

    /* boolean onTextEntered ([optional] in Event aEvent, [optional] in boolean itemWasSelected); */
    pub OnTextEntered: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aEvent: *const libc::c_void, itemWasSelected: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean onTextReverted (); */
    pub OnTextReverted: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean consumeRollupEvent; */
    pub GetConsumeRollupEvent: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aConsumeRollupEvent: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean inPrivateContext; */
    pub GetInPrivateContext: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aInPrivateContext: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean noRollupOnCaretMove; */
    pub GetNoRollupOnCaretMove: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aNoRollupOnCaretMove: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean noRollupOnEmptySearch; */
    pub GetNoRollupOnEmptySearch: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aNoRollupOnEmptySearch: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long userContextId; */
    pub GetUserContextId: unsafe extern "system" fn (this: *const nsIAutoCompleteInput, aUserContextId: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteInput {

    pub const TEXTVALUE_REASON_UNKNOWN: i64 = 0;


    pub const TEXTVALUE_REASON_COMPLETEDEFAULT: i64 = 1;


    pub const TEXTVALUE_REASON_COMPLETESELECTED: i64 = 2;


    pub const TEXTVALUE_REASON_REVERT: i64 = 3;


    pub const TEXTVALUE_REASON_ENTERMATCH: i64 = 4;


    /// `readonly attribute Element popupElement;`
    #[inline]
    pub unsafe fn GetPopupElement(&self, aPopupElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetPopupElement)(self, aPopupElement)
    }



    /// `readonly attribute nsIAutoCompletePopup popup;`
    #[inline]
    pub unsafe fn GetPopup(&self, aPopup: *mut*const nsIAutoCompletePopup) -> ::nserror::nsresult {
        ((*self.vtable).GetPopup)(self, aPopup)
    }



    /// `readonly attribute nsIAutoCompleteController controller;`
    #[inline]
    pub unsafe fn GetController(&self, aController: *mut *const nsIAutoCompleteController) -> ::nserror::nsresult {
        ((*self.vtable).GetController)(self, aController)
    }



    /// `[can_run_script] attribute boolean popupOpen;`
    #[inline]
    pub unsafe fn GetPopupOpen(&self, aPopupOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPopupOpen)(self, aPopupOpen)
    }



    /// `[can_run_script] attribute boolean popupOpen;`
    #[inline]
    pub unsafe fn SetPopupOpen(&self, aPopupOpen: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPopupOpen)(self, aPopupOpen)
    }



    /// `attribute boolean disableAutoComplete;`
    #[inline]
    pub unsafe fn GetDisableAutoComplete(&self, aDisableAutoComplete: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDisableAutoComplete)(self, aDisableAutoComplete)
    }



    /// `attribute boolean disableAutoComplete;`
    #[inline]
    pub unsafe fn SetDisableAutoComplete(&self, aDisableAutoComplete: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDisableAutoComplete)(self, aDisableAutoComplete)
    }



    /// `attribute boolean completeDefaultIndex;`
    #[inline]
    pub unsafe fn GetCompleteDefaultIndex(&self, aCompleteDefaultIndex: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCompleteDefaultIndex)(self, aCompleteDefaultIndex)
    }



    /// `attribute boolean completeDefaultIndex;`
    #[inline]
    pub unsafe fn SetCompleteDefaultIndex(&self, aCompleteDefaultIndex: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCompleteDefaultIndex)(self, aCompleteDefaultIndex)
    }



    /// `attribute boolean completeSelectedIndex;`
    #[inline]
    pub unsafe fn GetCompleteSelectedIndex(&self, aCompleteSelectedIndex: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCompleteSelectedIndex)(self, aCompleteSelectedIndex)
    }



    /// `attribute boolean completeSelectedIndex;`
    #[inline]
    pub unsafe fn SetCompleteSelectedIndex(&self, aCompleteSelectedIndex: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCompleteSelectedIndex)(self, aCompleteSelectedIndex)
    }



    /// `attribute boolean forceComplete;`
    #[inline]
    pub unsafe fn GetForceComplete(&self, aForceComplete: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetForceComplete)(self, aForceComplete)
    }



    /// `attribute boolean forceComplete;`
    #[inline]
    pub unsafe fn SetForceComplete(&self, aForceComplete: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetForceComplete)(self, aForceComplete)
    }



    /// `attribute unsigned long minResultsForPopup;`
    #[inline]
    pub unsafe fn GetMinResultsForPopup(&self, aMinResultsForPopup: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMinResultsForPopup)(self, aMinResultsForPopup)
    }



    /// `attribute unsigned long minResultsForPopup;`
    #[inline]
    pub unsafe fn SetMinResultsForPopup(&self, aMinResultsForPopup: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetMinResultsForPopup)(self, aMinResultsForPopup)
    }



    /// `attribute unsigned long maxRows;`
    #[inline]
    pub unsafe fn GetMaxRows(&self, aMaxRows: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxRows)(self, aMaxRows)
    }



    /// `attribute unsigned long maxRows;`
    #[inline]
    pub unsafe fn SetMaxRows(&self, aMaxRows: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxRows)(self, aMaxRows)
    }



    /// `attribute unsigned long timeout;`
    #[inline]
    pub unsafe fn GetTimeout(&self, aTimeout: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeout)(self, aTimeout)
    }



    /// `attribute unsigned long timeout;`
    #[inline]
    pub unsafe fn SetTimeout(&self, aTimeout: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetTimeout)(self, aTimeout)
    }



    /// `attribute AString searchParam;`
    #[inline]
    pub unsafe fn GetSearchParam(&self, aSearchParam: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchParam)(self, aSearchParam)
    }



    /// `attribute AString searchParam;`
    #[inline]
    pub unsafe fn SetSearchParam(&self, aSearchParam: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchParam)(self, aSearchParam)
    }



    /// `readonly attribute unsigned long searchCount;`
    #[inline]
    pub unsafe fn GetSearchCount(&self, aSearchCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchCount)(self, aSearchCount)
    }



    /// `ACString getSearchAt (in unsigned long index);`
    #[inline]
    pub unsafe fn GetSearchAt(&self, index: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchAt)(self, index, _retval)
    }



    /// `attribute AString textValue;`
    #[inline]
    pub unsafe fn GetTextValue(&self, aTextValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTextValue)(self, aTextValue)
    }



    /// `attribute AString textValue;`
    #[inline]
    pub unsafe fn SetTextValue(&self, aTextValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTextValue)(self, aTextValue)
    }



    /// `void setTextValueWithReason (in AString aValue, in unsigned short aReason);`
    #[inline]
    pub unsafe fn SetTextValueWithReason(&self, aValue: *const ::nsstring::nsAString, aReason: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetTextValueWithReason)(self, aValue, aReason)
    }



    /// `readonly attribute long selectionStart;`
    #[inline]
    pub unsafe fn GetSelectionStart(&self, aSelectionStart: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionStart)(self, aSelectionStart)
    }



    /// `readonly attribute long selectionEnd;`
    #[inline]
    pub unsafe fn GetSelectionEnd(&self, aSelectionEnd: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionEnd)(self, aSelectionEnd)
    }



    /// `void selectTextRange (in long startIndex, in long endIndex);`
    #[inline]
    pub unsafe fn SelectTextRange(&self, startIndex: i32, endIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SelectTextRange)(self, startIndex, endIndex)
    }



    /// `void onSearchBegin ();`
    #[inline]
    pub unsafe fn OnSearchBegin(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnSearchBegin)(self, )
    }



    /// `void onSearchComplete ();`
    #[inline]
    pub unsafe fn OnSearchComplete(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnSearchComplete)(self, )
    }



    /// `boolean onTextEntered ([optional] in Event aEvent, [optional] in boolean itemWasSelected);`
    #[inline]
    pub unsafe fn OnTextEntered(&self, aEvent: *const libc::c_void, itemWasSelected: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnTextEntered)(self, aEvent, itemWasSelected, _retval)
    }



    /// `boolean onTextReverted ();`
    #[inline]
    pub unsafe fn OnTextReverted(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnTextReverted)(self, _retval)
    }



    /// `readonly attribute boolean consumeRollupEvent;`
    #[inline]
    pub unsafe fn GetConsumeRollupEvent(&self, aConsumeRollupEvent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetConsumeRollupEvent)(self, aConsumeRollupEvent)
    }



    /// `readonly attribute boolean inPrivateContext;`
    #[inline]
    pub unsafe fn GetInPrivateContext(&self, aInPrivateContext: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInPrivateContext)(self, aInPrivateContext)
    }



    /// `readonly attribute boolean noRollupOnCaretMove;`
    #[inline]
    pub unsafe fn GetNoRollupOnCaretMove(&self, aNoRollupOnCaretMove: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNoRollupOnCaretMove)(self, aNoRollupOnCaretMove)
    }



    /// `readonly attribute boolean noRollupOnEmptySearch;`
    #[inline]
    pub unsafe fn GetNoRollupOnEmptySearch(&self, aNoRollupOnEmptySearch: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNoRollupOnEmptySearch)(self, aNoRollupOnEmptySearch)
    }


    /// ```text
    /// /**
    ///    * The userContextId of the current browser.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long userContextId;`
    #[inline]
    pub unsafe fn GetUserContextId(&self, aUserContextId: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetUserContextId)(self, aUserContextId)
    }


}


