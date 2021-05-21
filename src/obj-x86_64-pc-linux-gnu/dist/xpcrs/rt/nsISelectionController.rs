//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionController.idl
//


/// `interface nsISelectionController : nsISelectionDisplay`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISelectionController {
    vtable: *const nsISelectionControllerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISelectionController.
unsafe impl XpCom for nsISelectionController {
    const IID: nsIID = nsID(0x3801c9d4, 0x8e69, 0x4bfc,
        [0x9e, 0xdb, 0xb5, 0x82, 0x78, 0x62, 0x1f, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISelectionController {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISelectionController.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISelectionControllerCoerce {
    /// Cheaply cast a value of this type from a `nsISelectionController`.
    fn coerce_from(v: &nsISelectionController) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISelectionControllerCoerce for nsISelectionController {
    #[inline]
    fn coerce_from(v: &nsISelectionController) -> &Self {
        v
    }
}

impl nsISelectionController {
    /// Cast this `nsISelectionController` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISelectionControllerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISelectionController {
    type Target = nsISelectionDisplay;
    #[inline]
    fn deref(&self) -> &nsISelectionDisplay {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISelectionDisplayCoerce> nsISelectionControllerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISelectionController) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISelectionController
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISelectionControllerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISelectionDisplayVTable,

    /* void setDisplaySelection (in short toggle); */
    pub SetDisplaySelection: unsafe extern "system" fn (this: *const nsISelectionController, toggle: i16) -> ::nserror::nsresult,

    /* short getDisplaySelection (); */
    pub GetDisplaySelection: unsafe extern "system" fn (this: *const nsISelectionController, _retval: *mut i16) -> ::nserror::nsresult,

    /* [binaryname(GetSelectionFromScript)] Selection getSelection (in short type); */
    pub GetSelectionFromScript: unsafe extern "system" fn (this: *const nsISelectionController, type_: i16, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [binaryname(GetSelection),noscript,nostdcall,notxpcom] Selection getDOMSelection (in short aType); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetSelection: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void selectionWillTakeFocus (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SelectionWillTakeFocus: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void selectionWillLoseFocus (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SelectionWillLoseFocus: *const ::libc::c_void,

    /* void scrollSelectionIntoView (in short type, in short region, in short flags); */
    pub ScrollSelectionIntoView: unsafe extern "system" fn (this: *const nsISelectionController, type_: i16, region: i16, flags: i16) -> ::nserror::nsresult,

    /* void repaintSelection (in short type); */
    pub RepaintSelection: unsafe extern "system" fn (this: *const nsISelectionController, type_: i16) -> ::nserror::nsresult,

    /* void setCaretEnabled (in boolean enabled); */
    pub SetCaretEnabled: unsafe extern "system" fn (this: *const nsISelectionController, enabled: bool) -> ::nserror::nsresult,

    /* void setCaretReadOnly (in boolean readOnly); */
    pub SetCaretReadOnly: unsafe extern "system" fn (this: *const nsISelectionController, readOnly: bool) -> ::nserror::nsresult,

    /* boolean getCaretEnabled (); */
    pub GetCaretEnabled: unsafe extern "system" fn (this: *const nsISelectionController, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean caretVisible; */
    pub GetCaretVisible: unsafe extern "system" fn (this: *const nsISelectionController, aCaretVisible: *mut bool) -> ::nserror::nsresult,

    /* void setCaretVisibilityDuringSelection (in boolean visibility); */
    pub SetCaretVisibilityDuringSelection: unsafe extern "system" fn (this: *const nsISelectionController, visibility: bool) -> ::nserror::nsresult,

    /* void characterMove (in boolean forward, in boolean extend); */
    pub CharacterMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* void physicalMove (in short direction, in short amount, in boolean extend); */
    pub PhysicalMove: unsafe extern "system" fn (this: *const nsISelectionController, direction: i16, amount: i16, extend: bool) -> ::nserror::nsresult,

    /* void wordMove (in boolean forward, in boolean extend); */
    pub WordMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* void lineMove (in boolean forward, in boolean extend); */
    pub LineMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* void intraLineMove (in boolean forward, in boolean extend); */
    pub IntraLineMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* [can_run_script] void pageMove (in boolean forward, in boolean extend); */
    pub PageMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* void completeScroll (in boolean forward); */
    pub CompleteScroll: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool) -> ::nserror::nsresult,

    /* [can_run_script] void completeMove (in boolean forward, in boolean extend); */
    pub CompleteMove: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool, extend: bool) -> ::nserror::nsresult,

    /* void scrollPage (in boolean forward); */
    pub ScrollPage: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool) -> ::nserror::nsresult,

    /* void scrollLine (in boolean forward); */
    pub ScrollLine: unsafe extern "system" fn (this: *const nsISelectionController, forward: bool) -> ::nserror::nsresult,

    /* void scrollCharacter (in boolean right); */
    pub ScrollCharacter: unsafe extern "system" fn (this: *const nsISelectionController, right: bool) -> ::nserror::nsresult,

    /* boolean checkVisibility (in Node node, in short startOffset, in short endOffset); */
    pub CheckVisibility: unsafe extern "system" fn (this: *const nsISelectionController, node: *const libc::c_void, startOffset: i16, endOffset: i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub CheckVisibilityContent: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISelectionController {

    pub const SELECTION_NONE: i64 = 0;


    pub const SELECTION_NORMAL: i64 = 1;


    pub const SELECTION_SPELLCHECK: i64 = 2;


    pub const SELECTION_IME_RAWINPUT: i64 = 3;


    pub const SELECTION_IME_SELECTEDRAWTEXT: i64 = 4;


    pub const SELECTION_IME_CONVERTEDTEXT: i64 = 5;


    pub const SELECTION_IME_SELECTEDCONVERTEDTEXT: i64 = 6;


    pub const SELECTION_ACCESSIBILITY: i64 = 7;


    pub const SELECTION_FIND: i64 = 8;


    pub const SELECTION_URLSECONDARY: i64 = 9;


    pub const SELECTION_URLSTRIKEOUT: i64 = 10;


    pub const NUM_SELECTIONTYPES: i64 = 11;


    pub const SELECTION_ANCHOR_REGION: i64 = 0;


    pub const SELECTION_FOCUS_REGION: i64 = 1;


    pub const SELECTION_WHOLE_SELECTION: i64 = 2;


    pub const NUM_SELECTION_REGIONS: i64 = 3;


    pub const SELECTION_OFF: i64 = 0;


    pub const SELECTION_HIDDEN: i64 = 1;


    pub const SELECTION_ON: i64 = 2;


    pub const SELECTION_DISABLED: i64 = 3;


    pub const SELECTION_ATTENTION: i64 = 4;


    pub const SCROLL_SYNCHRONOUS: i64 = 2;


    pub const SCROLL_FIRST_ANCESTOR_ONLY: i64 = 4;


    pub const SCROLL_CENTER_VERTICALLY: i64 = 16;


    pub const SCROLL_OVERFLOW_HIDDEN: i64 = 32;


    pub const SCROLL_FOR_CARET_MOVE: i64 = 64;

    /// ```text
    /// /**
    ///     * nsFrameSelection::PhysicalMove depends on the ordering of these values;
    ///     * do not change without checking there!
    ///     */
    /// ```
    ///

    pub const MOVE_LEFT: i64 = 0;


    pub const MOVE_RIGHT: i64 = 1;


    pub const MOVE_UP: i64 = 2;


    pub const MOVE_DOWN: i64 = 3;

    /// ```text
    /// /**
    ///    * SetDisplaySelection will set the display mode for the selection. OFF,ON,DISABLED
    ///    */
    /// ```
    ///

    /// `void setDisplaySelection (in short toggle);`
    #[inline]
    pub unsafe fn SetDisplaySelection(&self, toggle: i16) -> ::nserror::nsresult {
        ((*self.vtable).SetDisplaySelection)(self, toggle)
    }


    /// ```text
    /// /**
    ///    * GetDisplaySelection will get the display mode for the selection. OFF,ON,DISABLED
    ///    */
    /// ```
    ///

    /// `short getDisplaySelection ();`
    #[inline]
    pub unsafe fn GetDisplaySelection(&self, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplaySelection)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * GetSelection will return the selection that the presentation
    ///    *  shell may implement.
    ///    *
    ///    * @param aType This will hold the type of selection.  This value must be one
    ///    *              of RawSelectionType values.
    ///    * @param _return will hold the return value
    ///    */
    /// ```
    ///

    /// `[binaryname(GetSelectionFromScript)] Selection getSelection (in short type);`
    #[inline]
    pub unsafe fn GetSelectionFromScript(&self, type_: i16, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionFromScript)(self, type_, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the selection object corresponding to a selection type.
    ///    */
    /// ```
    ///

    /// `[binaryname(GetSelection),noscript,nostdcall,notxpcom] Selection getDOMSelection (in short aType);`
    const _GetSelection: () = ();

    /// ```text
    /// /**
    ///     * Called when the selection controller should take the focus.
    ///     *
    ///     * This will take care to hide the previously-focused selection, show this
    ///     * selection, and repaint both.
    ///     */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void selectionWillTakeFocus ();`
    const _SelectionWillTakeFocus: () = ();

    /// ```text
    /// /**
    ///     * Called when the selection controller has lost the focus.
    ///     *
    ///     * This will take care to hide and repaint the selection.
    ///     */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void selectionWillLoseFocus ();`
    const _SelectionWillLoseFocus: () = ();

    /// ```text
    /// /**
    ///    * ScrollSelectionIntoView scrolls a region of the selection,
    ///    * so that it is visible in the scrolled view.
    ///    *
    ///    * @param aType the selection to scroll into view.  This value must be one
    ///    *              of RawSelectionType values.
    ///    * @param aRegion the region inside the selection to scroll into view. //SelectionRegion
    ///    * @param aFlags the scroll flags.  Valid bits include:
    ///    * SCROLL_SYNCHRONOUS: when set, scrolls the selection into view
    ///    * before returning. If not set, posts a request which is processed
    ///    * at some point after the method returns.
    ///    * SCROLL_FIRST_ANCESTOR_ONLY: if set, only the first ancestor will be scrolled
    ///    * into view.
    ///    * SCROLL_OVERFLOW_HIDDEN: if set, scrolls even if the overflow is specified
    ///    * as hidden.
    ///    * SCROLL_FOR_CARET_MOVE: set to indicate whether scrolling is in response
    ///    * to the caret being moved. Does not affect behavior (used for telemetry
        ///    * purposes only).
    ///    *
    ///    * Note that if isSynchronous is true, then this might flush the pending
    ///    * reflow. It's dangerous for some objects. See bug 418470 comment 12.
    ///    */
    /// ```
    ///

    /// `void scrollSelectionIntoView (in short type, in short region, in short flags);`
    #[inline]
    pub unsafe fn ScrollSelectionIntoView(&self, type_: i16, region: i16, flags: i16) -> ::nserror::nsresult {
        ((*self.vtable).ScrollSelectionIntoView)(self, type_, region, flags)
    }


    /// ```text
    /// /**
    ///    * RepaintSelection repaints the selection specified by aType.
    ///    *
    ///    * @param aType specifies the selection to repaint.
    ///    */
    /// ```
    ///

    /// `void repaintSelection (in short type);`
    #[inline]
    pub unsafe fn RepaintSelection(&self, type_: i16) -> ::nserror::nsresult {
        ((*self.vtable).RepaintSelection)(self, type_)
    }


    /// ```text
    /// /**
    ///    * Set the caret as enabled or disabled. An enabled caret will
    ///    * draw or blink when made visible. A disabled caret will never show up.
    ///    * Can be called any time.
    ///    * @param aEnable PR_TRUE to enable caret.  PR_FALSE to disable.
    ///    * @return always NS_OK
    ///    */
    /// ```
    ///

    /// `void setCaretEnabled (in boolean enabled);`
    #[inline]
    pub unsafe fn SetCaretEnabled(&self, enabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCaretEnabled)(self, enabled)
    }


    /// ```text
    /// /**
    ///    * Set the caret readonly or not. An readonly caret will
    ///    * draw but not blink when made visible.
    ///    * @param aReadOnly PR_TRUE to enable caret.  PR_FALSE to disable.
    ///    * @return always NS_OK
    ///    */
    /// ```
    ///

    /// `void setCaretReadOnly (in boolean readOnly);`
    #[inline]
    pub unsafe fn SetCaretReadOnly(&self, readOnly: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCaretReadOnly)(self, readOnly)
    }


    /// ```text
    /// /**
    ///    * Gets the current state of the caret.
    ///    * @param aEnabled  [OUT] set to the current caret state, as set by SetCaretEnabled
    ///    * @return   if aOutEnabled==null, returns NS_ERROR_INVALID_ARG
    ///    *           else NS_OK
    ///    */
    /// ```
    ///

    /// `boolean getCaretEnabled ();`
    #[inline]
    pub unsafe fn GetCaretEnabled(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCaretEnabled)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * This is true if the caret is enabled, visible, and currently blinking.
    ///     * This is still true when the caret is enabled, visible, but in its "off"
    ///     * blink cycle.
    ///     */
    /// ```
    ///

    /// `readonly attribute boolean caretVisible;`
    #[inline]
    pub unsafe fn GetCaretVisible(&self, aCaretVisible: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCaretVisible)(self, aCaretVisible)
    }


    /// ```text
    /// /**
    ///    * Show the caret even in selections. By default the caret is hidden unless the
    ///    * selection is collapsed. Use this function to show the caret even in selections.
    ///    * @param aVisibility PR_TRUE to show the caret in selections.  PR_FALSE to hide.
    ///    * @return always NS_OK
    ///    */
    /// ```
    ///

    /// `void setCaretVisibilityDuringSelection (in boolean visibility);`
    #[inline]
    pub unsafe fn SetCaretVisibilityDuringSelection(&self, visibility: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCaretVisibilityDuringSelection)(self, visibility)
    }


    /// ```text
    /// /** CharacterMove will move the selection one character forward/backward in the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `void characterMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn CharacterMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).CharacterMove)(self, forward, extend)
    }


    /// ```text
    /// /** PhysicalMove will move the selection one "unit" in a given direction
    ///    *  within the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aDirection
    ///    *  @param aAmount    character/line; word/lineBoundary
    ///    *  @param aExtend    should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `void physicalMove (in short direction, in short amount, in boolean extend);`
    #[inline]
    pub unsafe fn PhysicalMove(&self, direction: i16, amount: i16, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).PhysicalMove)(self, direction, amount, extend)
    }


    /// ```text
    /// /** WordMove will move the selection one word forward/backward in the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `void wordMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn WordMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).WordMove)(self, forward, extend)
    }


    /// ```text
    /// /** LineMove will move the selection one line forward/backward in the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `void lineMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn LineMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).LineMove)(self, forward, extend)
    }


    /// ```text
    /// /** IntraLineMove will move the selection to the front of the line or end of the line
    ///    *  in the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `void intraLineMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn IntraLineMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).IntraLineMove)(self, forward, extend)
    }


    /// ```text
    /// /** PageMove will move the selection one page forward/backward in the document.
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `[can_run_script] void pageMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn PageMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).PageMove)(self, forward, extend)
    }


    /// ```text
    /// /** CompleteScroll will move page view to the top or bottom of the document
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    */
    /// ```
    ///

    /// `void completeScroll (in boolean forward);`
    #[inline]
    pub unsafe fn CompleteScroll(&self, forward: bool) -> ::nserror::nsresult {
        ((*self.vtable).CompleteScroll)(self, forward)
    }


    /// ```text
    /// /** CompleteMove will move page view to the top or bottom of the document
    ///    *  this will also have the effect of collapsing the selection if the aExtend = PR_FALSE
    ///    *  the "point" of selection that is extended is considered the "focus" point.
    ///    *  or the last point adjusted by the selection.
    ///    *  @param aForward forward or backward if PR_FALSE
    ///    *  @param aExtend  should it collapse the selection of extend it?
    ///    */
    /// ```
    ///

    /// `[can_run_script] void completeMove (in boolean forward, in boolean extend);`
    #[inline]
    pub unsafe fn CompleteMove(&self, forward: bool, extend: bool) -> ::nserror::nsresult {
        ((*self.vtable).CompleteMove)(self, forward, extend)
    }


    /// ```text
    /// /** ScrollPage will scroll the page without affecting the selection.
    ///    *  @param aForward scroll forward or backwards in selection
    ///    */
    /// ```
    ///

    /// `void scrollPage (in boolean forward);`
    #[inline]
    pub unsafe fn ScrollPage(&self, forward: bool) -> ::nserror::nsresult {
        ((*self.vtable).ScrollPage)(self, forward)
    }


    /// ```text
    /// /** ScrollLine will scroll line up or down dependent on the boolean
    ///    *  @param aForward scroll forward or backwards in selection
    ///    */
    /// ```
    ///

    /// `void scrollLine (in boolean forward);`
    #[inline]
    pub unsafe fn ScrollLine(&self, forward: bool) -> ::nserror::nsresult {
        ((*self.vtable).ScrollLine)(self, forward)
    }


    /// ```text
    /// /** ScrollCharacter will scroll right or left dependent on the boolean
    ///    *  @param aRight if true will scroll right. if not will scroll left.
    ///    */
    /// ```
    ///

    /// `void scrollCharacter (in boolean right);`
    #[inline]
    pub unsafe fn ScrollCharacter(&self, right: bool) -> ::nserror::nsresult {
        ((*self.vtable).ScrollCharacter)(self, right)
    }


    /// ```text
    /// /** CheckVisibility will return true if textnode and offsets are actually rendered
    ///    *  in the current precontext.
    ///    *  @param aNode textNode to test
    ///    *  @param aStartOffset  offset in dom to first char of textnode to test
    ///    *  @param aEndOffset    offset in dom to last char of textnode to test
    ///    *  @param aReturnBool   boolean returned TRUE if visible FALSE if not
    ///    */
    /// ```
    ///

    /// `boolean checkVisibility (in Node node, in short startOffset, in short endOffset);`
    #[inline]
    pub unsafe fn CheckVisibility(&self, node: *const libc::c_void, startOffset: i16, endOffset: i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckVisibility)(self, node, startOffset, endOffset, _retval)
    }



    /// `[noscript,nostdcall] boolean checkVisibilityContent (in nsIContent node, in short startOffset, in short endOffset);`
    const _CheckVisibilityContent: () = ();

}


