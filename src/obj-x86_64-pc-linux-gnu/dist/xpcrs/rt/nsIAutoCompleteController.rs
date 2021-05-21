//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteController.idl
//


/// `interface nsIAutoCompleteController : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteController {
    vtable: *const nsIAutoCompleteControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteController.
unsafe impl XpCom for nsIAutoCompleteController {
    const IID: nsIID = nsID(0xff9f8465, 0x204a, 0x47a6,
        [0xb3, 0xc9, 0x06, 0x28, 0xb3, 0x85, 0x66, 0x84]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteControllerCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteController`.
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteControllerCoerce for nsIAutoCompleteController {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self {
        v
    }
}

impl nsIAutoCompleteController {
    /// Cast this `nsIAutoCompleteController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteController {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [setter_can_run_script] attribute nsIAutoCompleteInput input; */
    pub GetInput: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aInput: *mut*const nsIAutoCompleteInput) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute nsIAutoCompleteInput input; */
    pub SetInput: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aInput: *const nsIAutoCompleteInput) -> ::nserror::nsresult,

    /* readonly attribute unsigned short searchStatus; */
    pub GetSearchStatus: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aSearchStatus: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned long matchCount; */
    pub GetMatchCount: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aMatchCount: *mut u32) -> ::nserror::nsresult,

    /* [can_run_script] void startSearch (in AString searchString); */
    pub StartSearch: unsafe extern "system" fn (this: *const nsIAutoCompleteController, searchString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void stopSearch (); */
    pub StopSearch: unsafe extern "system" fn (this: *const nsIAutoCompleteController) -> ::nserror::nsresult,

    /* [can_run_script] boolean handleText (); */
    pub HandleText: unsafe extern "system" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] boolean handleEnter (in boolean aIsPopupSelection, [optional] in Event aEvent); */
    pub HandleEnter: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aIsPopupSelection: bool, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] boolean handleEscape (); */
    pub HandleEscape: unsafe extern "system" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void handleStartComposition (); */
    pub HandleStartComposition: unsafe extern "system" fn (this: *const nsIAutoCompleteController) -> ::nserror::nsresult,

    /* void handleEndComposition (); */
    pub HandleEndComposition: unsafe extern "system" fn (this: *const nsIAutoCompleteController) -> ::nserror::nsresult,

    /* [can_run_script] void handleTab (); */
    pub HandleTab: unsafe extern "system" fn (this: *const nsIAutoCompleteController) -> ::nserror::nsresult,

    /* [can_run_script] boolean handleKeyNavigation (in unsigned long key); */
    pub HandleKeyNavigation: unsafe extern "system" fn (this: *const nsIAutoCompleteController, key: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] boolean handleDelete (); */
    pub HandleDelete: unsafe extern "system" fn (this: *const nsIAutoCompleteController, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getValueAt (in long index); */
    pub GetValueAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getLabelAt (in long index); */
    pub GetLabelAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getCommentAt (in long index); */
    pub GetCommentAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getStyleAt (in long index); */
    pub GetStyleAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getImageAt (in long index); */
    pub GetImageAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getFinalCompleteValueAt (in long index); */
    pub GetFinalCompleteValueAt: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchString; */
    pub GetSearchString: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchString; */
    pub SetSearchString: unsafe extern "system" fn (this: *const nsIAutoCompleteController, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setInitiallySelectedIndex (in long index); */
    pub SetInitiallySelectedIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteController, index: i32) -> ::nserror::nsresult,

    /* [can_run_script] void resetInternalState (); */
    pub ResetInternalState: unsafe extern "system" fn (this: *const nsIAutoCompleteController) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteController {

    pub const STATUS_NONE: i64 = 1;


    pub const STATUS_SEARCHING: i64 = 2;


    pub const STATUS_COMPLETE_NO_MATCH: i64 = 3;


    pub const STATUS_COMPLETE_MATCH: i64 = 4;


    /// `[setter_can_run_script] attribute nsIAutoCompleteInput input;`
    #[inline]
    pub unsafe fn GetInput(&self, aInput: *mut*const nsIAutoCompleteInput) -> ::nserror::nsresult {
        ((*self.vtable).GetInput)(self, aInput)
    }



    /// `[setter_can_run_script] attribute nsIAutoCompleteInput input;`
    #[inline]
    pub unsafe fn SetInput(&self, aInput: *const nsIAutoCompleteInput) -> ::nserror::nsresult {
        ((*self.vtable).SetInput)(self, aInput)
    }



    /// `readonly attribute unsigned short searchStatus;`
    #[inline]
    pub unsafe fn GetSearchStatus(&self, aSearchStatus: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchStatus)(self, aSearchStatus)
    }



    /// `readonly attribute unsigned long matchCount;`
    #[inline]
    pub unsafe fn GetMatchCount(&self, aMatchCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchCount)(self, aMatchCount)
    }



    /// `[can_run_script] void startSearch (in AString searchString);`
    #[inline]
    pub unsafe fn StartSearch(&self, searchString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).StartSearch)(self, searchString)
    }



    /// `[can_run_script] void stopSearch ();`
    #[inline]
    pub unsafe fn StopSearch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopSearch)(self, )
    }



    /// `[can_run_script] boolean handleText ();`
    #[inline]
    pub unsafe fn HandleText(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleText)(self, _retval)
    }



    /// `[can_run_script] boolean handleEnter (in boolean aIsPopupSelection, [optional] in Event aEvent);`
    #[inline]
    pub unsafe fn HandleEnter(&self, aIsPopupSelection: bool, aEvent: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleEnter)(self, aIsPopupSelection, aEvent, _retval)
    }



    /// `[can_run_script] boolean handleEscape ();`
    #[inline]
    pub unsafe fn HandleEscape(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleEscape)(self, _retval)
    }



    /// `[can_run_script] void handleStartComposition ();`
    #[inline]
    pub unsafe fn HandleStartComposition(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).HandleStartComposition)(self, )
    }



    /// `void handleEndComposition ();`
    #[inline]
    pub unsafe fn HandleEndComposition(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).HandleEndComposition)(self, )
    }



    /// `[can_run_script] void handleTab ();`
    #[inline]
    pub unsafe fn HandleTab(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).HandleTab)(self, )
    }



    /// `[can_run_script] boolean handleKeyNavigation (in unsigned long key);`
    #[inline]
    pub unsafe fn HandleKeyNavigation(&self, key: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleKeyNavigation)(self, key, _retval)
    }



    /// `[can_run_script] boolean handleDelete ();`
    #[inline]
    pub unsafe fn HandleDelete(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleDelete)(self, _retval)
    }



    /// `AString getValueAt (in long index);`
    #[inline]
    pub unsafe fn GetValueAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValueAt)(self, index, _retval)
    }



    /// `AString getLabelAt (in long index);`
    #[inline]
    pub unsafe fn GetLabelAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLabelAt)(self, index, _retval)
    }



    /// `AString getCommentAt (in long index);`
    #[inline]
    pub unsafe fn GetCommentAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCommentAt)(self, index, _retval)
    }



    /// `AString getStyleAt (in long index);`
    #[inline]
    pub unsafe fn GetStyleAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStyleAt)(self, index, _retval)
    }



    /// `AString getImageAt (in long index);`
    #[inline]
    pub unsafe fn GetImageAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetImageAt)(self, index, _retval)
    }



    /// `AString getFinalCompleteValueAt (in long index);`
    #[inline]
    pub unsafe fn GetFinalCompleteValueAt(&self, index: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFinalCompleteValueAt)(self, index, _retval)
    }



    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn GetSearchString(&self, aSearchString: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchString)(self, aSearchString)
    }



    /// `attribute AString searchString;`
    #[inline]
    pub unsafe fn SetSearchString(&self, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchString)(self, aSearchString)
    }



    /// `void setInitiallySelectedIndex (in long index);`
    #[inline]
    pub unsafe fn SetInitiallySelectedIndex(&self, index: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetInitiallySelectedIndex)(self, index)
    }



    /// `[can_run_script] void resetInternalState ();`
    #[inline]
    pub unsafe fn ResetInternalState(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetInternalState)(self, )
    }


}


