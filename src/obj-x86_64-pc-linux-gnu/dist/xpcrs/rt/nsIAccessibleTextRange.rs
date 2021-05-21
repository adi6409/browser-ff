//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextRange.idl
//


/// `interface nsIAccessibleTextRange : nsISupports`
///

/// ```text
/// /**
///  * A range representing a piece of text in the document.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTextRange {
    vtable: *const nsIAccessibleTextRangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTextRange.
unsafe impl XpCom for nsIAccessibleTextRange {
    const IID: nsIID = nsID(0xc4515623, 0x55f9, 0x4543,
        [0xa3, 0xd5, 0xc1, 0xe9, 0xaf, 0xa5, 0x88, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTextRange {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTextRange.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTextRangeCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTextRange`.
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTextRangeCoerce for nsIAccessibleTextRange {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self {
        v
    }
}

impl nsIAccessibleTextRange {
    /// Cast this `nsIAccessibleTextRange` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTextRangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTextRange {
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
impl<T: nsISupportsCoerce> nsIAccessibleTextRangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextRange) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTextRange
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTextRangeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessibleText startContainer; */
    pub GetStartContainer: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aStartContainer: *mut*const nsIAccessibleText) -> ::nserror::nsresult,

    /* readonly attribute long startOffset; */
    pub GetStartOffset: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aStartOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleText endContainer; */
    pub GetEndContainer: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aEndContainer: *mut*const nsIAccessibleText) -> ::nserror::nsresult,

    /* readonly attribute long endOffset; */
    pub GetEndOffset: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aEndOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible container; */
    pub GetContainer: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aContainer: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIArray embeddedChildren; */
    pub GetEmbeddedChildren: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aEmbeddedChildren: *mut*const nsIArray) -> ::nserror::nsresult,

    /* boolean compare (in nsIAccessibleTextRange aOtherRange); */
    pub Compare: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aOtherRange: *const nsIAccessibleTextRange, _retval: *mut bool) -> ::nserror::nsresult,

    /* long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint); */
    pub CompareEndPoints: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aEndPoint: u32, aOtherRange: *const nsIAccessibleTextRange, aOtherRangeEndPoint: u32, _retval: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AString text; */
    pub GetText: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray bounds; */
    pub GetBounds: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aBounds: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void move (in unsigned long aUnit, in long aCount); */
    pub Move: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aUnit: u32, aCount: i32) -> ::nserror::nsresult,

    /* void moveStart (in unsigned long aUnit, in long aCount); */
    pub MoveStart: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aUnit: u32, aCount: i32) -> ::nserror::nsresult,

    /* void moveEnd (in unsigned long aUnit, in long aCount); */
    pub MoveEnd: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aUnit: u32, aCount: i32) -> ::nserror::nsresult,

    /* void normalize (in unsigned long aUnit); */
    pub Normalize: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aUnit: u32) -> ::nserror::nsresult,

    /* boolean crop (in nsIAccessible aContainer); */
    pub Crop: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aContainer: *const nsIAccessible, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase); */
    pub FindText: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aText: *const ::nsstring::nsAString, aIsBackward: bool, aIsIgnoreCase: bool, _retval: *mut *const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward); */
    pub FindAttr: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aAttr: u32, aValue: *const nsIVariant, aIsBackward: bool, _retval: *mut *const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* void addToSelection (); */
    pub AddToSelection: unsafe extern "system" fn (this: *const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* void removeFromSelection (); */
    pub RemoveFromSelection: unsafe extern "system" fn (this: *const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* void select (); */
    pub Select: unsafe extern "system" fn (this: *const nsIAccessibleTextRange) -> ::nserror::nsresult,

    /* void scrollIntoView (in unsigned long aHow); */
    pub ScrollIntoView: unsafe extern "system" fn (this: *const nsIAccessibleTextRange, aHow: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTextRange {
    /// ```text
    /// /**
    ///    * The two endpoints of the range (starting and ending).
    ///    */
    /// ```
    ///

    pub const EndPoint_Start: i64 = 1;


    pub const EndPoint_End: i64 = 2;


    pub const FormatUnit: i64 = 0;


    pub const WordUnit: i64 = 1;


    pub const LineUnit: i64 = 2;


    pub const ParagraphUnit: i64 = 3;


    pub const PageUnit: i64 = 4;


    pub const DocumentUnit: i64 = 5;

    /// ```text
    /// /**
    ///    * Text attributes. Used in conjunction with findAttrs().
    ///    */
    /// ```
    ///

    pub const AnimationStyleAttr: i64 = 0;


    pub const AnnotationObjectsAttr: i64 = 1;


    pub const AnnotationTypesAttr: i64 = 2;


    pub const BackgroundColorAttr: i64 = 3;


    pub const BulletStyleAttr: i64 = 4;


    pub const CapStyleAttr: i64 = 5;


    pub const CaretBidiModeAttr: i64 = 6;


    pub const CaretPositionAttr: i64 = 7;


    pub const CultureAttr: i64 = 8;


    pub const FontNameAttr: i64 = 9;


    pub const FontSizeAttr: i64 = 10;


    pub const FontWeightAttr: i64 = 11;


    pub const ForegroundColorAttr: i64 = 12;


    pub const HorizontalTextAlignmentAttr: i64 = 13;


    pub const IndentationFirstLineAttr: i64 = 14;


    pub const IndentationLeadingAttr: i64 = 15;


    pub const IndentationTrailingAttr: i64 = 16;


    pub const IsActiveAttr: i64 = 17;


    pub const IsHiddenAttr: i64 = 18;


    pub const IsItalicAttr: i64 = 19;


    pub const IsReadOnlyAttr: i64 = 20;


    pub const IsSubscriptAttr: i64 = 21;


    pub const IsSuperscriptAttr: i64 = 22;


    pub const LinkAttr: i64 = 23;


    pub const MarginBottomAttr: i64 = 24;


    pub const MarginLeadingAttr: i64 = 25;


    pub const MarginTopAttr: i64 = 26;


    pub const MarginTrailingAttr: i64 = 27;


    pub const OutlineStylesAttr: i64 = 28;


    pub const OverlineColorAttr: i64 = 29;


    pub const OverlineStyleAttr: i64 = 30;


    pub const SelectionActiveEndAttr: i64 = 31;


    pub const StrikethroughColorAttr: i64 = 32;


    pub const StrikethroughStyleAttr: i64 = 33;


    pub const StyleIdAttr: i64 = 34;


    pub const StyleNameAttr: i64 = 35;


    pub const TabsAttr: i64 = 36;


    pub const TextFlowDirectionsAttr: i64 = 37;


    pub const UnderlineColorAttr: i64 = 38;


    pub const UnderlineStyleAttr: i64 = 39;


    pub const AlignToTop: i64 = 0;


    pub const AlignToBottom: i64 = 1;


    /// `readonly attribute nsIAccessibleText startContainer;`
    #[inline]
    pub unsafe fn GetStartContainer(&self, aStartContainer: *mut*const nsIAccessibleText) -> ::nserror::nsresult {
        ((*self.vtable).GetStartContainer)(self, aStartContainer)
    }



    /// `readonly attribute long startOffset;`
    #[inline]
    pub unsafe fn GetStartOffset(&self, aStartOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetStartOffset)(self, aStartOffset)
    }



    /// `readonly attribute nsIAccessibleText endContainer;`
    #[inline]
    pub unsafe fn GetEndContainer(&self, aEndContainer: *mut*const nsIAccessibleText) -> ::nserror::nsresult {
        ((*self.vtable).GetEndContainer)(self, aEndContainer)
    }



    /// `readonly attribute long endOffset;`
    #[inline]
    pub unsafe fn GetEndOffset(&self, aEndOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetEndOffset)(self, aEndOffset)
    }


    /// ```text
    /// /**
    ///    * Return an accessible containing the whole range
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible container;`
    #[inline]
    pub unsafe fn GetContainer(&self, aContainer: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetContainer)(self, aContainer)
    }


    /// ```text
    /// /**
    ///    * Return embedded children within the range.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray embeddedChildren;`
    #[inline]
    pub unsafe fn GetEmbeddedChildren(&self, aEmbeddedChildren: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetEmbeddedChildren)(self, aEmbeddedChildren)
    }


    /// ```text
    /// /**
    ///    * Return true if this range has the same end points of the given range.
    ///    */
    /// ```
    ///

    /// `boolean compare (in nsIAccessibleTextRange aOtherRange);`
    #[inline]
    pub unsafe fn Compare(&self, aOtherRange: *const nsIAccessibleTextRange, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Compare)(self, aOtherRange, _retval)
    }


    /// ```text
    /// /**
    ///    * Compare this and given ranges end points.
    ///    *
    ///    * @return -1/0/1 if this range end point is before/equal/after the given
    ///    *          range end point.
    ///    */
    /// ```
    ///

    /// `long compareEndPoints (in unsigned long aEndPoint, in nsIAccessibleTextRange aOtherRange, in unsigned long aOtherRangeEndPoint);`
    #[inline]
    pub unsafe fn CompareEndPoints(&self, aEndPoint: u32, aOtherRange: *const nsIAccessibleTextRange, aOtherRangeEndPoint: u32, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).CompareEndPoints)(self, aEndPoint, aOtherRange, aOtherRangeEndPoint, _retval)
    }


    /// ```text
    /// /**
    ///    * Return text within the range.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString text;`
    #[inline]
    pub unsafe fn GetText(&self, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetText)(self, aText)
    }


    /// ```text
    /// /**
    ///    * Return list of rects of the range.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray bounds;`
    #[inline]
    pub unsafe fn GetBounds(&self, aBounds: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetBounds)(self, aBounds)
    }


    /// ```text
    /// /**
    ///    *  Move the boundary(ies) by the given number of the unit.
    ///    */
    /// ```
    ///

    /// `void move (in unsigned long aUnit, in long aCount);`
    #[inline]
    pub unsafe fn Move(&self, aUnit: u32, aCount: i32) -> ::nserror::nsresult {
        ((*self.vtable).Move)(self, aUnit, aCount)
    }



    /// `void moveStart (in unsigned long aUnit, in long aCount);`
    #[inline]
    pub unsafe fn MoveStart(&self, aUnit: u32, aCount: i32) -> ::nserror::nsresult {
        ((*self.vtable).MoveStart)(self, aUnit, aCount)
    }



    /// `void moveEnd (in unsigned long aUnit, in long aCount);`
    #[inline]
    pub unsafe fn MoveEnd(&self, aUnit: u32, aCount: i32) -> ::nserror::nsresult {
        ((*self.vtable).MoveEnd)(self, aUnit, aCount)
    }


    /// ```text
    /// /**
    ///    * Normalize the range to the closest unit of the given type.
    ///    */
    /// ```
    ///

    /// `void normalize (in unsigned long aUnit);`
    #[inline]
    pub unsafe fn Normalize(&self, aUnit: u32) -> ::nserror::nsresult {
        ((*self.vtable).Normalize)(self, aUnit)
    }


    /// ```text
    /// /**
    ///    * Crops the range by the given accessible element.
    ///    */
    /// ```
    ///

    /// `boolean crop (in nsIAccessible aContainer);`
    #[inline]
    pub unsafe fn Crop(&self, aContainer: *const nsIAccessible, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Crop)(self, aContainer, _retval)
    }


    /// ```text
    /// /**
    ///    * Return range enclosing the found text.
    ///    */
    /// ```
    ///

    /// `nsIAccessibleTextRange findText (in AString aText, in boolean aIsBackward, in boolean aIsIgnoreCase);`
    #[inline]
    pub unsafe fn FindText(&self, aText: *const ::nsstring::nsAString, aIsBackward: bool, aIsIgnoreCase: bool, _retval: *mut *const nsIAccessibleTextRange) -> ::nserror::nsresult {
        ((*self.vtable).FindText)(self, aText, aIsBackward, aIsIgnoreCase, _retval)
    }


    /// ```text
    /// /**
    ///    * Return range enslosing the text having requested attribute.
    ///    */
    /// ```
    ///

    /// `nsIAccessibleTextRange findAttr (in unsigned long aAttr, in nsIVariant aValue, in boolean aIsBackward);`
    #[inline]
    pub unsafe fn FindAttr(&self, aAttr: u32, aValue: *const nsIVariant, aIsBackward: bool, _retval: *mut *const nsIAccessibleTextRange) -> ::nserror::nsresult {
        ((*self.vtable).FindAttr)(self, aAttr, aValue, aIsBackward, _retval)
    }


    /// ```text
    /// /**
    ///    * Add/remove the text range from selection.
    ///    */
    /// ```
    ///

    /// `void addToSelection ();`
    #[inline]
    pub unsafe fn AddToSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AddToSelection)(self, )
    }



    /// `void removeFromSelection ();`
    #[inline]
    pub unsafe fn RemoveFromSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveFromSelection)(self, )
    }



    /// `void select ();`
    #[inline]
    pub unsafe fn Select(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Select)(self, )
    }


    /// ```text
    /// /**
    ///    * Scroll the range into view.
    ///    */
    /// ```
    ///

    /// `void scrollIntoView (in unsigned long aHow);`
    #[inline]
    pub unsafe fn ScrollIntoView(&self, aHow: u32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollIntoView)(self, aHow)
    }


}


