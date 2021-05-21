//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/xul/tree/nsITreeSelection.idl
//


/// `interface nsITreeSelection : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITreeSelection {
    vtable: *const nsITreeSelectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITreeSelection.
unsafe impl XpCom for nsITreeSelection {
    const IID: nsIID = nsID(0xab6fe746, 0x300b, 0x4ab4,
        [0xab, 0xb9, 0x1c, 0x0e, 0x39, 0x77, 0x87, 0x4c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITreeSelection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITreeSelection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITreeSelectionCoerce {
    /// Cheaply cast a value of this type from a `nsITreeSelection`.
    fn coerce_from(v: &nsITreeSelection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITreeSelectionCoerce for nsITreeSelection {
    #[inline]
    fn coerce_from(v: &nsITreeSelection) -> &Self {
        v
    }
}

impl nsITreeSelection {
    /// Cast this `nsITreeSelection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITreeSelectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITreeSelection {
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
impl<T: nsISupportsCoerce> nsITreeSelectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITreeSelection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITreeSelection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITreeSelectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute XULTreeElement tree; */
    pub GetTree: unsafe extern "system" fn (this: *const nsITreeSelection, aTree: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute XULTreeElement tree; */
    pub SetTree: unsafe extern "system" fn (this: *const nsITreeSelection, aTree: *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean single; */
    pub GetSingle: unsafe extern "system" fn (this: *const nsITreeSelection, aSingle: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long count; */
    pub GetCount: unsafe extern "system" fn (this: *const nsITreeSelection, aCount: *mut i32) -> ::nserror::nsresult,

    /* boolean isSelected (in long index); */
    pub IsSelected: unsafe extern "system" fn (this: *const nsITreeSelection, index: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* void select (in long index); */
    pub Select: unsafe extern "system" fn (this: *const nsITreeSelection, index: i32) -> ::nserror::nsresult,

    /* void timedSelect (in long index, in long delay); */
    pub TimedSelect: unsafe extern "system" fn (this: *const nsITreeSelection, index: i32, delay: i32) -> ::nserror::nsresult,

    /* void toggleSelect (in long index); */
    pub ToggleSelect: unsafe extern "system" fn (this: *const nsITreeSelection, index: i32) -> ::nserror::nsresult,

    /* void rangedSelect (in long startIndex, in long endIndex, in boolean augment); */
    pub RangedSelect: unsafe extern "system" fn (this: *const nsITreeSelection, startIndex: i32, endIndex: i32, augment: bool) -> ::nserror::nsresult,

    /* void clearRange (in long startIndex, in long endIndex); */
    pub ClearRange: unsafe extern "system" fn (this: *const nsITreeSelection, startIndex: i32, endIndex: i32) -> ::nserror::nsresult,

    /* void clearSelection (); */
    pub ClearSelection: unsafe extern "system" fn (this: *const nsITreeSelection) -> ::nserror::nsresult,

    /* void selectAll (); */
    pub SelectAll: unsafe extern "system" fn (this: *const nsITreeSelection) -> ::nserror::nsresult,

    /* long getRangeCount (); */
    pub GetRangeCount: unsafe extern "system" fn (this: *const nsITreeSelection, _retval: *mut i32) -> ::nserror::nsresult,

    /* void getRangeAt (in long i, out long min, out long max); */
    pub GetRangeAt: unsafe extern "system" fn (this: *const nsITreeSelection, i: i32, min: *mut i32, max: *mut i32) -> ::nserror::nsresult,

    /* void invalidateSelection (); */
    pub InvalidateSelection: unsafe extern "system" fn (this: *const nsITreeSelection) -> ::nserror::nsresult,

    /* void adjustSelection (in long index, in long count); */
    pub AdjustSelection: unsafe extern "system" fn (this: *const nsITreeSelection, index: i32, count: i32) -> ::nserror::nsresult,

    /* attribute boolean selectEventsSuppressed; */
    pub GetSelectEventsSuppressed: unsafe extern "system" fn (this: *const nsITreeSelection, aSelectEventsSuppressed: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean selectEventsSuppressed; */
    pub SetSelectEventsSuppressed: unsafe extern "system" fn (this: *const nsITreeSelection, aSelectEventsSuppressed: bool) -> ::nserror::nsresult,

    /* attribute long currentIndex; */
    pub GetCurrentIndex: unsafe extern "system" fn (this: *const nsITreeSelection, aCurrentIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long currentIndex; */
    pub SetCurrentIndex: unsafe extern "system" fn (this: *const nsITreeSelection, aCurrentIndex: i32) -> ::nserror::nsresult,

    /* readonly attribute long shiftSelectPivot; */
    pub GetShiftSelectPivot: unsafe extern "system" fn (this: *const nsITreeSelection, aShiftSelectPivot: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITreeSelection {

    /// ```text
    /// /**
    ///    * The tree widget for this selection.
    ///    */
    /// ```
    ///

    /// `attribute XULTreeElement tree;`
    #[inline]
    pub unsafe fn GetTree(&self, aTree: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetTree)(self, aTree)
    }


    /// ```text
    /// /**
    ///    * The tree widget for this selection.
    ///    */
    /// ```
    ///

    /// `attribute XULTreeElement tree;`
    #[inline]
    pub unsafe fn SetTree(&self, aTree: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetTree)(self, aTree)
    }


    /// ```text
    /// /**
    ///    * This attribute is a boolean indicating single selection.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean single;`
    #[inline]
    pub unsafe fn GetSingle(&self, aSingle: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSingle)(self, aSingle)
    }


    /// ```text
    /// /**
    ///    * The number of rows currently selected in this tree.
    ///    */
    /// ```
    ///

    /// `readonly attribute long count;`
    #[inline]
    pub unsafe fn GetCount(&self, aCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCount)(self, aCount)
    }


    /// ```text
    /// /**
    ///    * Indicates whether or not the row at the specified index is
    ///    * part of the selection.
    ///    */
    /// ```
    ///

    /// `boolean isSelected (in long index);`
    #[inline]
    pub unsafe fn IsSelected(&self, index: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSelected)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Deselect all rows and select the row at the specified index.
    ///    */
    /// ```
    ///

    /// `void select (in long index);`
    #[inline]
    pub unsafe fn Select(&self, index: i32) -> ::nserror::nsresult {
        ((*self.vtable).Select)(self, index)
    }


    /// ```text
    /// /**
    ///    * Perform a timed select.
    ///    */
    /// ```
    ///

    /// `void timedSelect (in long index, in long delay);`
    #[inline]
    pub unsafe fn TimedSelect(&self, index: i32, delay: i32) -> ::nserror::nsresult {
        ((*self.vtable).TimedSelect)(self, index, delay)
    }


    /// ```text
    /// /**
    ///    * Toggle the selection state of the row at the specified index.
    ///    */
    /// ```
    ///

    /// `void toggleSelect (in long index);`
    #[inline]
    pub unsafe fn ToggleSelect(&self, index: i32) -> ::nserror::nsresult {
        ((*self.vtable).ToggleSelect)(self, index)
    }


    /// ```text
    /// /**
    ///    * Select the range specified by the indices.  If augment is true,
    ///    * then we add the range to the selection without clearing out anything
    ///    * else.  If augment is false, everything is cleared except for the specified range.
    ///    */
    /// ```
    ///

    /// `void rangedSelect (in long startIndex, in long endIndex, in boolean augment);`
    #[inline]
    pub unsafe fn RangedSelect(&self, startIndex: i32, endIndex: i32, augment: bool) -> ::nserror::nsresult {
        ((*self.vtable).RangedSelect)(self, startIndex, endIndex, augment)
    }


    /// ```text
    /// /**
    ///    * Clears the range.
    ///    */
    /// ```
    ///

    /// `void clearRange (in long startIndex, in long endIndex);`
    #[inline]
    pub unsafe fn ClearRange(&self, startIndex: i32, endIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).ClearRange)(self, startIndex, endIndex)
    }


    /// ```text
    /// /**
    ///    * Clears the selection.
    ///    */
    /// ```
    ///

    /// `void clearSelection ();`
    #[inline]
    pub unsafe fn ClearSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearSelection)(self, )
    }


    /// ```text
    /// /**
    ///    * Selects all rows.
    ///    */
    /// ```
    ///

    /// `void selectAll ();`
    #[inline]
    pub unsafe fn SelectAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectAll)(self, )
    }


    /// ```text
    /// /**
    ///    * Iterate the selection using these methods.
    ///    */
    /// ```
    ///

    /// `long getRangeCount ();`
    #[inline]
    pub unsafe fn GetRangeCount(&self, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRangeCount)(self, _retval)
    }



    /// `void getRangeAt (in long i, out long min, out long max);`
    #[inline]
    pub unsafe fn GetRangeAt(&self, i: i32, min: *mut i32, max: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRangeAt)(self, i, min, max)
    }


    /// ```text
    /// /**
    ///    * Can be used to invalidate the selection.
    ///    */
    /// ```
    ///

    /// `void invalidateSelection ();`
    #[inline]
    pub unsafe fn InvalidateSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InvalidateSelection)(self, )
    }


    /// ```text
    /// /**
    ///    * Called when the row count changes to adjust selection indices.
    ///    */
    /// ```
    ///

    /// `void adjustSelection (in long index, in long count);`
    #[inline]
    pub unsafe fn AdjustSelection(&self, index: i32, count: i32) -> ::nserror::nsresult {
        ((*self.vtable).AdjustSelection)(self, index, count)
    }


    /// ```text
    /// /**
    ///    * This attribute is a boolean indicating whether or not the
    ///    * "select" event should fire when the selection is changed using
    ///    * one of our methods.  A view can use this to temporarily suppress
    ///    * the selection while manipulating all of the indices, e.g., on
    ///    * a sort.
    ///    * Note: setting this attribute to false will fire a select event.
    ///    */
    /// ```
    ///

    /// `attribute boolean selectEventsSuppressed;`
    #[inline]
    pub unsafe fn GetSelectEventsSuppressed(&self, aSelectEventsSuppressed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectEventsSuppressed)(self, aSelectEventsSuppressed)
    }


    /// ```text
    /// /**
    ///    * This attribute is a boolean indicating whether or not the
    ///    * "select" event should fire when the selection is changed using
    ///    * one of our methods.  A view can use this to temporarily suppress
    ///    * the selection while manipulating all of the indices, e.g., on
    ///    * a sort.
    ///    * Note: setting this attribute to false will fire a select event.
    ///    */
    /// ```
    ///

    /// `attribute boolean selectEventsSuppressed;`
    #[inline]
    pub unsafe fn SetSelectEventsSuppressed(&self, aSelectEventsSuppressed: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSelectEventsSuppressed)(self, aSelectEventsSuppressed)
    }


    /// ```text
    /// /**
    ///    * The current item (the one that gets a focus rect in addition to being
        ///    * selected).
    ///    */
    /// ```
    ///

    /// `attribute long currentIndex;`
    #[inline]
    pub unsafe fn GetCurrentIndex(&self, aCurrentIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentIndex)(self, aCurrentIndex)
    }


    /// ```text
    /// /**
    ///    * The current item (the one that gets a focus rect in addition to being
        ///    * selected).
    ///    */
    /// ```
    ///

    /// `attribute long currentIndex;`
    #[inline]
    pub unsafe fn SetCurrentIndex(&self, aCurrentIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentIndex)(self, aCurrentIndex)
    }


    /// ```text
    /// /**
    ///    * The selection "pivot".  This is the first item the user selected as
    ///    * part of a ranged select.
    ///    */
    /// ```
    ///

    /// `readonly attribute long shiftSelectPivot;`
    #[inline]
    pub unsafe fn GetShiftSelectPivot(&self, aShiftSelectPivot: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetShiftSelectPivot)(self, aShiftSelectPivot)
    }


}


/// `interface nsINativeTreeSelection : nsITreeSelection`
///

/// ```text
/// /**
///  * The following interface is not scriptable and MUST NEVER BE MADE scriptable.
///  * Native treeselections implement it, and we use this to check whether a
///  * treeselection is native (and therefore suitable for use by untrusted content).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeTreeSelection {
    vtable: *const nsINativeTreeSelectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeTreeSelection.
unsafe impl XpCom for nsINativeTreeSelection {
    const IID: nsIID = nsID(0x1bd59678, 0x5cb3, 0x4316,
        [0xb2, 0x46, 0x31, 0xa9, 0x1b, 0x19, 0xaa, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeTreeSelection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeTreeSelection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeTreeSelectionCoerce {
    /// Cheaply cast a value of this type from a `nsINativeTreeSelection`.
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeTreeSelectionCoerce for nsINativeTreeSelection {
    #[inline]
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self {
        v
    }
}

impl nsINativeTreeSelection {
    /// Cast this `nsINativeTreeSelection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeTreeSelectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeTreeSelection {
    type Target = nsITreeSelection;
    #[inline]
    fn deref(&self) -> &nsITreeSelection {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsITreeSelectionCoerce> nsINativeTreeSelectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeTreeSelection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeTreeSelection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeTreeSelectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsITreeSelectionVTable,

    /* [noscript] void ensureNative (); */
    pub EnsureNative: unsafe extern "system" fn (this: *const nsINativeTreeSelection) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeTreeSelection {


    /// `[noscript] void ensureNative ();`
    #[inline]
    pub unsafe fn EnsureNative(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnsureNative)(self, )
    }


}


