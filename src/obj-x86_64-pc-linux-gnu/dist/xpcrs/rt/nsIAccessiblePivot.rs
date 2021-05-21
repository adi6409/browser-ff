//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessiblePivot.idl
//


/// `typedef int16_t  TextBoundaryType;`
///


pub type TextBoundaryType = i16;


/// `typedef int16_t  PivotMoveReason;`
///


pub type PivotMoveReason = i16;


/// `interface nsIAccessiblePivot : nsISupports`
///

/// ```text
/// /**
///  * The pivot interface encapsulates a reference to a single place in an accessible
///  * subtree. The pivot is a point or a range in the accessible tree. This interface
///  * provides traversal methods to move the pivot to next/prev state that complies
///  * to a given rule.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessiblePivot {
    vtable: *const nsIAccessiblePivotVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessiblePivot.
unsafe impl XpCom for nsIAccessiblePivot {
    const IID: nsIID = nsID(0x81fe5144, 0x059b, 0x42db,
        [0xbd, 0x3a, 0xf6, 0xce, 0x31, 0x58, 0xd5, 0xe9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessiblePivot {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessiblePivot.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessiblePivotCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessiblePivot`.
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessiblePivotCoerce for nsIAccessiblePivot {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self {
        v
    }
}

impl nsIAccessiblePivot {
    /// Cast this `nsIAccessiblePivot` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessiblePivotCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessiblePivot {
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
impl<T: nsISupportsCoerce> nsIAccessiblePivotCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivot) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessiblePivot
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessiblePivotVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIAccessible position; */
    pub GetPosition: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aPosition: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* attribute nsIAccessible position; */
    pub SetPosition: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aPosition: *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible root; */
    pub GetRoot: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aRoot: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* attribute nsIAccessible modalRoot; */
    pub GetModalRoot: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aModalRoot: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* attribute nsIAccessible modalRoot; */
    pub SetModalRoot: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aModalRoot: *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute long startOffset; */
    pub GetStartOffset: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aStartOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long endOffset; */
    pub GetEndOffset: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aEndOffset: *mut i32) -> ::nserror::nsresult,

    /* [optional_argc] void setTextRange (in nsIAccessibleText aTextAccessible, in long aStartOffset, in long aEndOffset, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub SetTextRange: *const ::libc::c_void,

    /* [optional_argc] boolean moveNext (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MoveNext: *const ::libc::c_void,

    /* [optional_argc] boolean movePrevious (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MovePrevious: *const ::libc::c_void,

    /* [optional_argc] boolean moveFirst (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MoveFirst: *const ::libc::c_void,

    /* [optional_argc] boolean moveLast (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MoveLast: *const ::libc::c_void,

    /* [optional_argc] boolean moveNextByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MoveNextByText: *const ::libc::c_void,

    /* [optional_argc] boolean movePreviousByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MovePreviousByText: *const ::libc::c_void,

    /* [optional_argc] boolean moveToPoint (in nsIAccessibleTraversalRule aRule, in long aX, in long aY, in boolean aIgnoreNoMatch, [optional] in boolean aIsFromUserInput); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub MoveToPoint: *const ::libc::c_void,

    /* void addObserver (in nsIAccessiblePivotObserver aObserver); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aObserver: *const nsIAccessiblePivotObserver) -> ::nserror::nsresult,

    /* void removeObserver (in nsIAccessiblePivotObserver aObserver); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsIAccessiblePivot, aObserver: *const nsIAccessiblePivotObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessiblePivot {

    pub const NO_BOUNDARY: i64 = -1;


    pub const CHAR_BOUNDARY: i64 = 0;


    pub const WORD_BOUNDARY: i64 = 1;


    pub const LINE_BOUNDARY: i64 = 2;


    pub const ATTRIBUTE_RANGE_BOUNDARY: i64 = 3;


    pub const REASON_NONE: i64 = 0;


    pub const REASON_NEXT: i64 = 1;


    pub const REASON_PREV: i64 = 2;


    pub const REASON_FIRST: i64 = 3;


    pub const REASON_LAST: i64 = 4;


    pub const REASON_POINT: i64 = 5;

    /// ```text
    /// /**
    ///    * The accessible the pivot is currently pointed at.
    ///    */
    /// ```
    ///

    /// `attribute nsIAccessible position;`
    #[inline]
    pub unsafe fn GetPosition(&self, aPosition: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetPosition)(self, aPosition)
    }


    /// ```text
    /// /**
    ///    * The accessible the pivot is currently pointed at.
    ///    */
    /// ```
    ///

    /// `attribute nsIAccessible position;`
    #[inline]
    pub unsafe fn SetPosition(&self, aPosition: *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).SetPosition)(self, aPosition)
    }


    /// ```text
    /// /**
    ///    * The root of the subtree in which the pivot traverses.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible root;`
    #[inline]
    pub unsafe fn GetRoot(&self, aRoot: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetRoot)(self, aRoot)
    }


    /// ```text
    /// /**
    ///    * The temporary modal root to which traversal is limited to.
    ///    */
    /// ```
    ///

    /// `attribute nsIAccessible modalRoot;`
    #[inline]
    pub unsafe fn GetModalRoot(&self, aModalRoot: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetModalRoot)(self, aModalRoot)
    }


    /// ```text
    /// /**
    ///    * The temporary modal root to which traversal is limited to.
    ///    */
    /// ```
    ///

    /// `attribute nsIAccessible modalRoot;`
    #[inline]
    pub unsafe fn SetModalRoot(&self, aModalRoot: *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).SetModalRoot)(self, aModalRoot)
    }


    /// ```text
    /// /**
    ///    * The start offset of the text range the pivot points at, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long startOffset;`
    #[inline]
    pub unsafe fn GetStartOffset(&self, aStartOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetStartOffset)(self, aStartOffset)
    }


    /// ```text
    /// /**
    ///    * The end offset of the text range the pivot points at, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long endOffset;`
    #[inline]
    pub unsafe fn GetEndOffset(&self, aEndOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetEndOffset)(self, aEndOffset)
    }


    /// ```text
    /// /**
    ///    * Set the pivot's text range in a text accessible.
    ///    *
    ///    * @param aTextAccessible  [in] the text accessible that contains the desired
    ///    *                           range.
    ///    * @param aStartOffset     [in] the start offset to set.
    ///    * @param aEndOffset       [in] the end offset to set.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @throws NS_ERROR_INVALID_ARG when the offset exceeds the accessible's
    ///    *   character count.
    ///    */
    /// ```
    ///

    /// `[optional_argc] void setTextRange (in nsIAccessibleText aTextAccessible, in long aStartOffset, in long aEndOffset, [optional] in boolean aIsFromUserInput);`
    const _SetTextRange: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to next object, from current position or given anchor,
    ///    * complying to given traversal rule.
    ///    *
    ///    * @param aRule            [in] traversal rule to use.
    ///    * @param aAnchor          [in] accessible to start search from, if not provided,
    ///    *                           current position will be used.
    ///    * @param aIncludeStart    [in] include anchor accessible in search.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if there are no new nodes to traverse to.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean moveNext (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput);`
    const _MoveNext: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to previous object, from current position or given anchor,
    ///    * complying to given traversal rule.
    ///    *
    ///    * @param aRule            [in] traversal rule to use.
    ///    * @param aAnchor          [in] accessible to start search from, if not provided,
    ///    *                           current position will be used.
    ///    * @param aIncludeStart    [in] include anchor accessible in search.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if there are no new nodes to traverse to.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean movePrevious (in nsIAccessibleTraversalRule aRule, [optional] in nsIAccessible aAnchor, [optional] in boolean aIncludeStart, [optional] in boolean aIsFromUserInput);`
    const _MovePrevious: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to first object in subtree complying to given traversal rule.
    ///    *
    ///    * @param aRule            [in] traversal rule to use.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if there are no new nodes to traverse to.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean moveFirst (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput);`
    const _MoveFirst: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to last object in subtree complying to given traversal rule.
    ///    *
    ///    * @param aRule            [in] traversal rule to use.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean moveLast (in nsIAccessibleTraversalRule aRule, [optional] in boolean aIsFromUserInput);`
    const _MoveLast: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to next text range.
    ///    *
    ///    * @param aBoundary        [in] type of boundary for next text range,
    ///    *                           character, word, etc.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if there are is no more text.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean moveNextByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput);`
    const _MoveNextByText: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to previous text range.
    ///    *
    ///    * @param aBoundary        [in] type of boundary for next text range,
    ///    *                           character, word, etc.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if there are is no more text.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean movePreviousByText (in TextBoundaryType aBoundary, [optional] in boolean aIsFromUserInput);`
    const _MovePreviousByText: () = ();

    /// ```text
    /// /**
    ///    * Move pivot to given coordinate in screen pixels.
    ///    *
    ///    * @param aRule            [in]  raversal rule to use.
    ///    * @param aX               [in]  screen's x coordinate
    ///    * @param aY               [in]  screen's y coordinate
    ///    * @param aIgnoreNoMatch   [in]  don't unset position if no object was found
    ///    *                           at point.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    * @return true on success, false if the pivot has not been moved.
    ///    */
    /// ```
    ///

    /// `[optional_argc] boolean moveToPoint (in nsIAccessibleTraversalRule aRule, in long aX, in long aY, in boolean aIgnoreNoMatch, [optional] in boolean aIsFromUserInput);`
    const _MoveToPoint: () = ();

    /// ```text
    /// /**
    ///    * Add an observer for pivot changes.
    ///    *
    ///    * @param aObserver [in] the observer object to be notified of pivot changes.
    ///    */
    /// ```
    ///

    /// `void addObserver (in nsIAccessiblePivotObserver aObserver);`
    #[inline]
    pub unsafe fn AddObserver(&self, aObserver: *const nsIAccessiblePivotObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * Remove an observer for pivot changes.
    ///    *
    ///    * @param aObserver [in] the observer object to remove from being notified.
    ///    */
    /// ```
    ///

    /// `void removeObserver (in nsIAccessiblePivotObserver aObserver);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, aObserver: *const nsIAccessiblePivotObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, aObserver)
    }


}


/// `interface nsIAccessiblePivotObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessiblePivotObserver {
    vtable: *const nsIAccessiblePivotObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessiblePivotObserver.
unsafe impl XpCom for nsIAccessiblePivotObserver {
    const IID: nsIID = nsID(0x6006e502, 0x3861, 0x49bd,
        [0xab, 0xa1, 0xfa, 0x6d, 0x2e, 0x74, 0xe2, 0x37]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessiblePivotObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessiblePivotObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessiblePivotObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessiblePivotObserver`.
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessiblePivotObserverCoerce for nsIAccessiblePivotObserver {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self {
        v
    }
}

impl nsIAccessiblePivotObserver {
    /// Cast this `nsIAccessiblePivotObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessiblePivotObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessiblePivotObserver {
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
impl<T: nsISupportsCoerce> nsIAccessiblePivotObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessiblePivotObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessiblePivotObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessiblePivotObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in nsIAccessible aNewAccessible, in long aNewStart, in long aNewEnd, in PivotMoveReason aReason, in TextBoundaryType aBoundaryType, in boolean aIsFromUserInput); */
    pub OnPivotChanged: unsafe extern "system" fn (this: *const nsIAccessiblePivotObserver, aPivot: *const nsIAccessiblePivot, aOldAccessible: *const nsIAccessible, aOldStart: i32, aOldEnd: i32, aNewAccessible: *const nsIAccessible, aNewStart: i32, aNewEnd: i32, aReason: PivotMoveReason, aBoundaryType: TextBoundaryType, aIsFromUserInput: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessiblePivotObserver {

    /// ```text
    /// /**
    ///  * An observer interface for pivot changes.
    ///  */
    /// /**
    ///    * Called when the pivot changes.
    ///    *
    ///    * @param aPivot           [in] the pivot that has changed.
    ///    * @param aOldAccessible   [in] the old pivot position before the change,
    ///    *                           or null.
    ///    * @param aOldStart        [in] the old start offset, or -1.
    ///    * @param aOldEnd          [in] the old end offset, or -1.
    ///    * @param aReason          [in] the reason for the pivot change.
    ///    * @param aIsFromUserInput [in] the pivot changed because of direct user input
    ///    *                           (default is true).
    ///    */
    /// ```
    ///

    /// `void onPivotChanged (in nsIAccessiblePivot aPivot, in nsIAccessible aOldAccessible, in long aOldStart, in long aOldEnd, in nsIAccessible aNewAccessible, in long aNewStart, in long aNewEnd, in PivotMoveReason aReason, in TextBoundaryType aBoundaryType, in boolean aIsFromUserInput);`
    #[inline]
    pub unsafe fn OnPivotChanged(&self, aPivot: *const nsIAccessiblePivot, aOldAccessible: *const nsIAccessible, aOldStart: i32, aOldEnd: i32, aNewAccessible: *const nsIAccessible, aNewStart: i32, aNewEnd: i32, aReason: PivotMoveReason, aBoundaryType: TextBoundaryType, aIsFromUserInput: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnPivotChanged)(self, aPivot, aOldAccessible, aOldStart, aOldEnd, aNewAccessible, aNewStart, aNewEnd, aReason, aBoundaryType, aIsFromUserInput)
    }


}


/// `interface nsIAccessibleTraversalRule : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTraversalRule {
    vtable: *const nsIAccessibleTraversalRuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTraversalRule.
unsafe impl XpCom for nsIAccessibleTraversalRule {
    const IID: nsIID = nsID(0xe197460d, 0x1eff, 0x4247,
        [0xb4, 0xbb, 0xa4, 0x3b, 0xe1, 0x84, 0x0d, 0xae]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTraversalRule {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTraversalRule.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTraversalRuleCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTraversalRule`.
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTraversalRuleCoerce for nsIAccessibleTraversalRule {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self {
        v
    }
}

impl nsIAccessibleTraversalRule {
    /// Cast this `nsIAccessibleTraversalRule` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTraversalRuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTraversalRule {
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
impl<T: nsISupportsCoerce> nsIAccessibleTraversalRuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTraversalRule) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTraversalRule
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTraversalRuleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long preFilter; */
    pub GetPreFilter: unsafe extern "system" fn (this: *const nsIAccessibleTraversalRule, aPreFilter: *mut u32) -> ::nserror::nsresult,

    /* Array<unsigned long> getMatchRoles (); */
    pub GetMatchRoles: unsafe extern "system" fn (this: *const nsIAccessibleTraversalRule, _retval: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult,

    /* unsigned short match (in nsIAccessible aAccessible); */
    pub Match: unsafe extern "system" fn (this: *const nsIAccessibleTraversalRule, aAccessible: *const nsIAccessible, _retval: *mut u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTraversalRule {

    pub const FILTER_IGNORE: i64 = 0;


    pub const FILTER_MATCH: i64 = 1;


    pub const FILTER_IGNORE_SUBTREE: i64 = 2;


    pub const PREFILTER_INVISIBLE: i64 = 1;


    pub const PREFILTER_OFFSCREEN: i64 = 2;


    pub const PREFILTER_NOT_FOCUSABLE: i64 = 4;


    pub const PREFILTER_TRANSPARENT: i64 = 8;


    pub const PREFILTER_PLATFORM_PRUNED: i64 = 16;

    /// ```text
    /// /**
    ///    * Pre-filter bitfield to filter out obviously ignorable nodes and lighten
    ///    * the load on match().
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long preFilter;`
    #[inline]
    pub unsafe fn GetPreFilter(&self, aPreFilter: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPreFilter)(self, aPreFilter)
    }


    /// ```text
    /// /**
    ///    * Retrieve a list of roles that the traversal rule should test for. Any node
    ///    * with a role not in this list will automatically be ignored. An empty list
    ///    * will match all roles. It should be assumed that this method is called once
    ///    * at the start of a traversal, so changing the method's return result after
    ///    * that would have no affect.
    ///    *
    ///    * @return an array of the roles to match.
    ///    */
    /// ```
    ///

    /// `Array<unsigned long> getMatchRoles ();`
    #[inline]
    pub unsafe fn GetMatchRoles(&self, _retval: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult {
        ((*self.vtable).GetMatchRoles)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Determines if a given accessible is to be accepted in our traversal rule
    ///    *
    ///    * @param aAccessible [in] accessible to examine.
    ///    * @return a bitfield of FILTER_MATCH and FILTER_IGNORE_SUBTREE.
    ///    */
    /// ```
    ///

    /// `unsigned short match (in nsIAccessible aAccessible);`
    #[inline]
    pub unsafe fn Match(&self, aAccessible: *const nsIAccessible, _retval: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).Match)(self, aAccessible, _retval)
    }


}


