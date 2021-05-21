//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleText.idl
//


/// `typedef int32_t  AccessibleTextBoundary;`
///


pub type AccessibleTextBoundary = i32;


/// `interface nsIAccessibleText : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleText {
    vtable: *const nsIAccessibleTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleText.
unsafe impl XpCom for nsIAccessibleText {
    const IID: nsIID = nsID(0xa4cc7576, 0x45bb, 0x44c5,
        [0xb3, 0x47, 0xd9, 0xcb, 0x3c, 0xa4, 0xde, 0x9f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleText {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleText.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTextCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleText`.
    fn coerce_from(v: &nsIAccessibleText) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTextCoerce for nsIAccessibleText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleText) -> &Self {
        v
    }
}

impl nsIAccessibleText {
    /// Cast this `nsIAccessibleText` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleText {
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
impl<T: nsISupportsCoerce> nsIAccessibleTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleText) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleText
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute long caretOffset; */
    pub GetCaretOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, aCaretOffset: *mut i32) -> ::nserror::nsresult,

    /* attribute long caretOffset; */
    pub SetCaretOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, aCaretOffset: i32) -> ::nserror::nsresult,

    /* readonly attribute long characterCount; */
    pub GetCharacterCount: unsafe extern "system" fn (this: *const nsIAccessibleText, aCharacterCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long selectionCount; */
    pub GetSelectionCount: unsafe extern "system" fn (this: *const nsIAccessibleText, aSelectionCount: *mut i32) -> ::nserror::nsresult,

    /* AString getText (in long startOffset, in long endOffset); */
    pub GetText: unsafe extern "system" fn (this: *const nsIAccessibleText, startOffset: i32, endOffset: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub GetTextAfterOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub GetTextAtOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
    pub GetTextBeforeOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* wchar getCharacterAtOffset (in long offset); */
    pub GetCharacterAtOffset: unsafe extern "system" fn (this: *const nsIAccessibleText, offset: i32, _retval: *mut i16) -> ::nserror::nsresult,

    /* nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset); */
    pub GetTextAttributes: unsafe extern "system" fn (this: *const nsIAccessibleText, includeDefAttrs: bool, offset: i32, rangeStartOffset: *mut i32, rangeEndOffset: *mut i32, _retval: *mut*const nsIPersistentProperties) -> ::nserror::nsresult,

    /* readonly attribute nsIPersistentProperties defaultTextAttributes; */
    pub GetDefaultTextAttributes: unsafe extern "system" fn (this: *const nsIAccessibleText, aDefaultTextAttributes: *mut*const nsIPersistentProperties) -> ::nserror::nsresult,

    /* void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    pub GetCharacterExtents: unsafe extern "system" fn (this: *const nsIAccessibleText, offset: i32, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32, coordType: u32) -> ::nserror::nsresult,

    /* void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
    pub GetRangeExtents: unsafe extern "system" fn (this: *const nsIAccessibleText, startOffset: i32, endOffset: i32, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32, coordType: u32) -> ::nserror::nsresult,

    /* long getOffsetAtPoint (in long x, in long y, in unsigned long coordType); */
    pub GetOffsetAtPoint: unsafe extern "system" fn (this: *const nsIAccessibleText, x: i32, y: i32, coordType: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset); */
    pub GetSelectionBounds: unsafe extern "system" fn (this: *const nsIAccessibleText, selectionNum: i32, startOffset: *mut i32, endOffset: *mut i32) -> ::nserror::nsresult,

    /* void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset); */
    pub SetSelectionBounds: unsafe extern "system" fn (this: *const nsIAccessibleText, selectionNum: i32, startOffset: i32, endOffset: i32) -> ::nserror::nsresult,

    /* void addSelection (in long startOffset, in long endOffset); */
    pub AddSelection: unsafe extern "system" fn (this: *const nsIAccessibleText, startOffset: i32, endOffset: i32) -> ::nserror::nsresult,

    /* void removeSelection (in long selectionNum); */
    pub RemoveSelection: unsafe extern "system" fn (this: *const nsIAccessibleText, selectionNum: i32) -> ::nserror::nsresult,

    /* void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType); */
    pub ScrollSubstringTo: unsafe extern "system" fn (this: *const nsIAccessibleText, startIndex: i32, endIndex: i32, scrollType: u32) -> ::nserror::nsresult,

    /* void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y); */
    pub ScrollSubstringToPoint: unsafe extern "system" fn (this: *const nsIAccessibleText, startIndex: i32, endIndex: i32, coordinateType: u32, x: i32, y: i32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleTextRange enclosingRange; */
    pub GetEnclosingRange: unsafe extern "system" fn (this: *const nsIAccessibleText, aEnclosingRange: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* readonly attribute nsIArray selectionRanges; */
    pub GetSelectionRanges: unsafe extern "system" fn (this: *const nsIAccessibleText, aSelectionRanges: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute nsIArray visibleRanges; */
    pub GetVisibleRanges: unsafe extern "system" fn (this: *const nsIAccessibleText, aVisibleRanges: *mut*const nsIArray) -> ::nserror::nsresult,

    /* nsIAccessibleTextRange getRangeByChild (in nsIAccessible child); */
    pub GetRangeByChild: unsafe extern "system" fn (this: *const nsIAccessibleText, child: *const nsIAccessible, _retval: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* nsIAccessibleTextRange getRangeAtPoint (in long x, in long y); */
    pub GetRangeAtPoint: unsafe extern "system" fn (this: *const nsIAccessibleText, x: i32, y: i32, _retval: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleText {

    pub const TEXT_OFFSET_END_OF_TEXT: i64 = -1;


    pub const TEXT_OFFSET_CARET: i64 = -2;


    pub const BOUNDARY_CHAR: i64 = 0;


    pub const BOUNDARY_WORD_START: i64 = 1;


    pub const BOUNDARY_WORD_END: i64 = 2;


    pub const BOUNDARY_SENTENCE_START: i64 = 3;


    pub const BOUNDARY_SENTENCE_END: i64 = 4;


    pub const BOUNDARY_LINE_START: i64 = 5;


    pub const BOUNDARY_LINE_END: i64 = 6;


    pub const BOUNDARY_PARAGRAPH: i64 = 7;

    /// ```text
    /// /**
    ///    * The current current caret offset.
    ///    * If set < 0 then caret will be placed  at the end of the text
    ///    */
    /// ```
    ///

    /// `attribute long caretOffset;`
    #[inline]
    pub unsafe fn GetCaretOffset(&self, aCaretOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCaretOffset)(self, aCaretOffset)
    }


    /// ```text
    /// /**
    ///    * The current current caret offset.
    ///    * If set < 0 then caret will be placed  at the end of the text
    ///    */
    /// ```
    ///

    /// `attribute long caretOffset;`
    #[inline]
    pub unsafe fn SetCaretOffset(&self, aCaretOffset: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetCaretOffset)(self, aCaretOffset)
    }



    /// `readonly attribute long characterCount;`
    #[inline]
    pub unsafe fn GetCharacterCount(&self, aCharacterCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCharacterCount)(self, aCharacterCount)
    }



    /// `readonly attribute long selectionCount;`
    #[inline]
    pub unsafe fn GetSelectionCount(&self, aSelectionCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionCount)(self, aSelectionCount)
    }


    /// ```text
    /// /**
    ///       * String methods may need to return multibyte-encoded strings,
    ///       * since some locales can't be encoded using 16-bit chars.
    ///       * So the methods below might return UTF-16 strings, or they could
    ///       * return "string" values which are UTF-8.
    ///       */
    /// ```
    ///

    /// `AString getText (in long startOffset, in long endOffset);`
    #[inline]
    pub unsafe fn GetText(&self, startOffset: i32, endOffset: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetText)(self, startOffset, endOffset, _retval)
    }



    /// `AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset);`
    #[inline]
    pub unsafe fn GetTextAfterOffset(&self, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTextAfterOffset)(self, offset, boundaryType, startOffset, endOffset, _retval)
    }



    /// `AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset);`
    #[inline]
    pub unsafe fn GetTextAtOffset(&self, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTextAtOffset)(self, offset, boundaryType, startOffset, endOffset, _retval)
    }



    /// `AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset);`
    #[inline]
    pub unsafe fn GetTextBeforeOffset(&self, offset: i32, boundaryType: AccessibleTextBoundary, startOffset: *mut i32, endOffset: *mut i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTextBeforeOffset)(self, offset, boundaryType, startOffset, endOffset, _retval)
    }


    /// ```text
    /// /**
    ///       * It would be better to return an unsigned long here,
    ///       * to allow unicode chars > 16 bits
    ///       */
    /// ```
    ///

    /// `wchar getCharacterAtOffset (in long offset);`
    #[inline]
    pub unsafe fn GetCharacterAtOffset(&self, offset: i32, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetCharacterAtOffset)(self, offset, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the accessible start/end offsets around the given offset,
    ///    * return the text attributes for this range of text.
    ///    *
    ///    * @param  includeDefAttrs   [in] points whether text attributes applied to
    ///    *                           the entire accessible should be included or not.
    ///    * @param  offset            [in] text offset
    ///    * @param  rangeStartOffset  [out] start offset of the range of text
    ///    * @param  rangeEndOffset    [out] end offset of the range of text
    ///    */
    /// ```
    ///

    /// `nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset);`
    #[inline]
    pub unsafe fn GetTextAttributes(&self, includeDefAttrs: bool, offset: i32, rangeStartOffset: *mut i32, rangeEndOffset: *mut i32, _retval: *mut*const nsIPersistentProperties) -> ::nserror::nsresult {
        ((*self.vtable).GetTextAttributes)(self, includeDefAttrs, offset, rangeStartOffset, rangeEndOffset, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the text attributes that apply to the entire accessible.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPersistentProperties defaultTextAttributes;`
    #[inline]
    pub unsafe fn GetDefaultTextAttributes(&self, aDefaultTextAttributes: *mut*const nsIPersistentProperties) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultTextAttributes)(self, aDefaultTextAttributes)
    }


    /// ```text
    /// /**
    ///    * Returns the bounding box of the specified position.
    ///    *
    ///    * The virtual character after the last character of the represented text,
    ///    * i.e. the one at position length is a special case. It represents the
    ///    * current input position and will therefore typically be queried by AT more
    ///    * often than other positions. Because it does not represent an existing
    ///    * character its bounding box is defined in relation to preceding characters.
    ///    * It should be roughly equivalent to the bounding box of some character when
    ///    * inserted at the end of the text. Its height typically being the maximal
    ///    * height of all the characters in the text or the height of the preceding
    ///    * character, its width being at least one pixel so that the bounding box is
    ///    * not degenerate.
    ///    *
    ///    * @param offset - Index of the character for which to return its bounding
    ///    *                  box. The valid range is 0..length.
    ///    * @param x - X coordinate of the bounding box of the referenced character.
    ///    * @param y - Y coordinate of the bounding box of the referenced character.
    ///    * @param width - Width of the bounding box of the referenced character.
    ///    * @param height - Height of the bounding box of the referenced character.
    ///    * @param coordType - Specifies if the coordinates are relative to the screen
    ///    *                    or to the parent window (see constants declared in
        ///    *                    nsIAccessibleCoordinateType).
    ///   */
    /// ```
    ///

    /// `void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType);`
    #[inline]
    pub unsafe fn GetCharacterExtents(&self, offset: i32, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32, coordType: u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCharacterExtents)(self, offset, x, y, width, height, coordType)
    }



    /// `void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType);`
    #[inline]
    pub unsafe fn GetRangeExtents(&self, startOffset: i32, endOffset: i32, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32, coordType: u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRangeExtents)(self, startOffset, endOffset, x, y, width, height, coordType)
    }


    /// ```text
    /// /**
    ///    * Get the text offset at the given point, or return -1
    ///    * if no character exists at that point
    ///    *
    ///    * @param x - The position's x value for which to look up the index of the
    ///    *            character that is rendered on to the display at that point.
    ///    * @param y - The position's y value for which to look up the index of the
    ///    *            character that is rendered on to the display at that point.
    ///    * @param coordType - Screen coordinates or window coordinates (see constants
        ///    *                    declared in nsIAccessibleCoordinateType).
    ///    * @return offset - Index of the character under the given point or -1 if
    ///    *                  the point is invalid or there is no character under
    ///    *                  the point.
    ///    */
    /// ```
    ///

    /// `long getOffsetAtPoint (in long x, in long y, in unsigned long coordType);`
    #[inline]
    pub unsafe fn GetOffsetAtPoint(&self, x: i32, y: i32, coordType: u32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetOffsetAtPoint)(self, x, y, coordType, _retval)
    }



    /// `void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset);`
    #[inline]
    pub unsafe fn GetSelectionBounds(&self, selectionNum: i32, startOffset: *mut i32, endOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionBounds)(self, selectionNum, startOffset, endOffset)
    }


    /// ```text
    /// /**
    ///    * Set the bounds for the given selection range.
    ///    * A reverse range where the start offset is larger than the end offset is
    ///    * acceptable. The caretOffset will be set to the endOffset argument.
    ///    */
    /// ```
    ///

    /// `void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset);`
    #[inline]
    pub unsafe fn SetSelectionBounds(&self, selectionNum: i32, startOffset: i32, endOffset: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectionBounds)(self, selectionNum, startOffset, endOffset)
    }



    /// `void addSelection (in long startOffset, in long endOffset);`
    #[inline]
    pub unsafe fn AddSelection(&self, startOffset: i32, endOffset: i32) -> ::nserror::nsresult {
        ((*self.vtable).AddSelection)(self, startOffset, endOffset)
    }



    /// `void removeSelection (in long selectionNum);`
    #[inline]
    pub unsafe fn RemoveSelection(&self, selectionNum: i32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveSelection)(self, selectionNum)
    }


    /// ```text
    /// /**
    ///    * Makes a specific part of string visible on screen.
    ///    *
    ///    * @param startIndex  0-based character offset
    ///    * @param endIndex    0-based character offset - the offset of the
    ///    *                    character just past the last character of the
    ///    *                    string
    ///    * @param scrollType  defines how to scroll (see nsIAccessibleScrollType for
        ///    *                    available constants)
    ///    */
    /// ```
    ///

    /// `void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType);`
    #[inline]
    pub unsafe fn ScrollSubstringTo(&self, startIndex: i32, endIndex: i32, scrollType: u32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollSubstringTo)(self, startIndex, endIndex, scrollType)
    }


    /// ```text
    /// /**
    ///    * Moves the top left of a substring to a specified location.
    ///    *
    ///    * @param startIndex      0-based character offset
    ///    * @param endIndex        0-based character offset - the offset of the
    ///    *                        character just past the last character of
    ///    *                        the string
    ///    * @param coordinateType  specifies the coordinates origin (for available
        ///    *                        constants refer to nsIAccessibleCoordinateType)
    ///    * @param x               defines the x coordinate
    ///    * @param y               defines the y coordinate
    ///    */
    /// ```
    ///

    /// `void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y);`
    #[inline]
    pub unsafe fn ScrollSubstringToPoint(&self, startIndex: i32, endIndex: i32, coordinateType: u32, x: i32, y: i32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollSubstringToPoint)(self, startIndex, endIndex, coordinateType, x, y)
    }


    /// ```text
    /// /**
    ///    * Return a range that encloses this text control or otherwise the document
    ///    * this text accessible belongs to.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleTextRange enclosingRange;`
    #[inline]
    pub unsafe fn GetEnclosingRange(&self, aEnclosingRange: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult {
        ((*self.vtable).GetEnclosingRange)(self, aEnclosingRange)
    }


    /// ```text
    /// /**
    ///    * Return an array of disjoint ranges for selected text within the text control
    ///    * or otherwise the document this accessible belongs to.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray selectionRanges;`
    #[inline]
    pub unsafe fn GetSelectionRanges(&self, aSelectionRanges: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionRanges)(self, aSelectionRanges)
    }


    /// ```text
    /// /**
    ///    * Return an array of disjoint ranges of visible text within the text control
    ///    * or otherwise the document this accessible belongs to.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray visibleRanges;`
    #[inline]
    pub unsafe fn GetVisibleRanges(&self, aVisibleRanges: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetVisibleRanges)(self, aVisibleRanges)
    }


    /// ```text
    /// /**
    ///    * Return a range containing the given accessible.
    ///    */
    /// ```
    ///

    /// `nsIAccessibleTextRange getRangeByChild (in nsIAccessible child);`
    #[inline]
    pub unsafe fn GetRangeByChild(&self, child: *const nsIAccessible, _retval: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult {
        ((*self.vtable).GetRangeByChild)(self, child, _retval)
    }


    /// ```text
    /// /**
    ///    * Return a range containing an accessible at the given point.
    ///    */
    /// ```
    ///

    /// `nsIAccessibleTextRange getRangeAtPoint (in long x, in long y);`
    #[inline]
    pub unsafe fn GetRangeAtPoint(&self, x: i32, y: i32, _retval: *mut*const nsIAccessibleTextRange) -> ::nserror::nsresult {
        ((*self.vtable).GetRangeAtPoint)(self, x, y, _retval)
    }


}


