//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessible.idl
//


/// `interface nsIAccessible : nsISupports`
///

/// ```text
/// /**
///  * A cross-platform interface that supports platform-specific
///  * accessibility APIs like MSAA and ATK. Contains the sum of what's needed
///  * to support IAccessible as well as ATK's generic accessibility objects.
///  * Can also be used by in-process accessibility clients to get information
///  * about objects in the accessible tree. The accessible tree is a subset of
///  * nodes in the DOM tree -- such as documents, focusable elements and text.
///  * Mozilla creates the implementations of nsIAccessible on demand.
///  * See http://www.mozilla.org/projects/ui/accessibility for more information.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessible {
    vtable: *const nsIAccessibleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessible.
unsafe impl XpCom for nsIAccessible {
    const IID: nsIID = nsID(0xde2869d9, 0x563c, 0x4943,
        [0x99, 0x6b, 0x31, 0xa4, 0xda, 0xa4, 0xd0, 0x97]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessible {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessible.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessible`.
    fn coerce_from(v: &nsIAccessible) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleCoerce for nsIAccessible {
    #[inline]
    fn coerce_from(v: &nsIAccessible) -> &Self {
        v
    }
}

impl nsIAccessible {
    /// Cast this `nsIAccessible` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessible {
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
impl<T: nsISupportsCoerce> nsIAccessibleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessible) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessible
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIAccessible parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsIAccessible, aParent: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible nextSibling; */
    pub GetNextSibling: unsafe extern "system" fn (this: *const nsIAccessible, aNextSibling: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible previousSibling; */
    pub GetPreviousSibling: unsafe extern "system" fn (this: *const nsIAccessible, aPreviousSibling: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible firstChild; */
    pub GetFirstChild: unsafe extern "system" fn (this: *const nsIAccessible, aFirstChild: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible lastChild; */
    pub GetLastChild: unsafe extern "system" fn (this: *const nsIAccessible, aLastChild: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIArray children; */
    pub GetChildren: unsafe extern "system" fn (this: *const nsIAccessible, aChildren: *mut *const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute long childCount; */
    pub GetChildCount: unsafe extern "system" fn (this: *const nsIAccessible, aChildCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long indexInParent; */
    pub GetIndexInParent: unsafe extern "system" fn (this: *const nsIAccessible, aIndexInParent: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long long uniqueID; */
    pub GetUniqueID: unsafe extern "system" fn (this: *const nsIAccessible, aUniqueID: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute Node DOMNode; */
    pub GetDOMNode: unsafe extern "system" fn (this: *const nsIAccessible, aDOMNode: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIAccessible, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleDocument document; */
    pub GetDocument: unsafe extern "system" fn (this: *const nsIAccessible, aDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleDocument rootDocument; */
    pub GetRootDocument: unsafe extern "system" fn (this: *const nsIAccessible, aRootDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult,

    /* readonly attribute AString language; */
    pub GetLanguage: unsafe extern "system" fn (this: *const nsIAccessible, aLanguage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIAccessible, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIAccessible, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsIAccessible, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString accessKey; */
    pub GetAccessKey: unsafe extern "system" fn (this: *const nsIAccessible, aAccessKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString keyboardShortcut; */
    pub GetKeyboardShortcut: unsafe extern "system" fn (this: *const nsIAccessible, aKeyboardShortcut: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long role; */
    pub GetRole: unsafe extern "system" fn (this: *const nsIAccessible, aRole: *mut u32) -> ::nserror::nsresult,

    /* void getState (out unsigned long aState, out unsigned long aExtraState); */
    pub GetState: unsafe extern "system" fn (this: *const nsIAccessible, aState: *mut u32, aExtraState: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString help; */
    pub GetHelp: unsafe extern "system" fn (this: *const nsIAccessible, aHelp: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible focusedChild; */
    pub GetFocusedChild: unsafe extern "system" fn (this: *const nsIAccessible, aFocusedChild: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIPersistentProperties attributes; */
    pub GetAttributes: unsafe extern "system" fn (this: *const nsIAccessible, aAttributes: *mut*const nsIPersistentProperties) -> ::nserror::nsresult,

    /* readonly attribute nsISupports nativeInterface; */
    pub GetNativeInterface: unsafe extern "system" fn (this: *const nsIAccessible, aNativeInterface: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup); */
    pub GroupPosition: unsafe extern "system" fn (this: *const nsIAccessible, aGroupLevel: *mut i32, aSimilarItemsInGroup: *mut i32, aPositionInGroup: *mut i32) -> ::nserror::nsresult,

    /* nsIAccessible getChildAtPoint (in long x, in long y); */
    pub GetChildAtPoint: unsafe extern "system" fn (this: *const nsIAccessible, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessible getDeepestChildAtPoint (in long x, in long y); */
    pub GetDeepestChildAtPoint: unsafe extern "system" fn (this: *const nsIAccessible, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessible getDeepestChildAtPointInProcess (in long x, in long y); */
    pub GetDeepestChildAtPointInProcess: unsafe extern "system" fn (this: *const nsIAccessible, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessible getChildAt (in long aChildIndex); */
    pub GetChildAt: unsafe extern "system" fn (this: *const nsIAccessible, aChildIndex: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult,

    /* nsIAccessibleRelation getRelationByType (in unsigned long aRelationType); */
    pub GetRelationByType: unsafe extern "system" fn (this: *const nsIAccessible, aRelationType: u32, _retval: *mut*const nsIAccessibleRelation) -> ::nserror::nsresult,

    /* nsIArray getRelations (); */
    pub GetRelations: unsafe extern "system" fn (this: *const nsIAccessible, _retval: *mut *const nsIArray) -> ::nserror::nsresult,

    /* void getBounds (out long x, out long y, out long width, out long height); */
    pub GetBounds: unsafe extern "system" fn (this: *const nsIAccessible, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void getBoundsInCSSPixels (out long aX, out long aY, out long aWidth, out long aHeight); */
    pub GetBoundsInCSSPixels: unsafe extern "system" fn (this: *const nsIAccessible, aX: *mut i32, aY: *mut i32, aWidth: *mut i32, aHeight: *mut i32) -> ::nserror::nsresult,

    /* void setSelected (in boolean isSelected); */
    pub SetSelected: unsafe extern "system" fn (this: *const nsIAccessible, isSelected: bool) -> ::nserror::nsresult,

    /* void takeSelection (); */
    pub TakeSelection: unsafe extern "system" fn (this: *const nsIAccessible) -> ::nserror::nsresult,

    /* void takeFocus (); */
    pub TakeFocus: unsafe extern "system" fn (this: *const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute uint8_t actionCount; */
    pub GetActionCount: unsafe extern "system" fn (this: *const nsIAccessible, aActionCount: *mut uint8_t) -> ::nserror::nsresult,

    /* AString getActionName (in uint8_t index); */
    pub GetActionName: unsafe extern "system" fn (this: *const nsIAccessible, index: uint8_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getActionDescription (in uint8_t aIndex); */
    pub GetActionDescription: unsafe extern "system" fn (this: *const nsIAccessible, aIndex: uint8_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void doAction (in uint8_t index); */
    pub DoAction: unsafe extern "system" fn (this: *const nsIAccessible, index: uint8_t) -> ::nserror::nsresult,

    /* [can_run_script] void scrollTo (in unsigned long aScrollType); */
    pub ScrollTo: unsafe extern "system" fn (this: *const nsIAccessible, aScrollType: u32) -> ::nserror::nsresult,

    /* void scrollToPoint (in unsigned long coordinateType, in long x, in long y); */
    pub ScrollToPoint: unsafe extern "system" fn (this: *const nsIAccessible, coordinateType: u32, x: i32, y: i32) -> ::nserror::nsresult,

    /* void announce (in AString announcement, in unsigned short priority); */
    pub Announce: unsafe extern "system" fn (this: *const nsIAccessible, announcement: *const ::nsstring::nsAString, priority: u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessible {

    /// ```text
    /// /**
    ///    * Parent node in accessible tree.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }


    /// ```text
    /// /**
    ///    * Next sibling in accessible tree
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible nextSibling;`
    #[inline]
    pub unsafe fn GetNextSibling(&self, aNextSibling: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetNextSibling)(self, aNextSibling)
    }


    /// ```text
    /// /**
    ///    * Previous sibling in accessible tree
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible previousSibling;`
    #[inline]
    pub unsafe fn GetPreviousSibling(&self, aPreviousSibling: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetPreviousSibling)(self, aPreviousSibling)
    }


    /// ```text
    /// /**
    ///    * First child in accessible tree
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible firstChild;`
    #[inline]
    pub unsafe fn GetFirstChild(&self, aFirstChild: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetFirstChild)(self, aFirstChild)
    }


    /// ```text
    /// /**
    ///    * Last child in accessible tree
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible lastChild;`
    #[inline]
    pub unsafe fn GetLastChild(&self, aLastChild: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetLastChild)(self, aLastChild)
    }


    /// ```text
    /// /**
    ///    * Array of all this element's children.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray children;`
    #[inline]
    pub unsafe fn GetChildren(&self, aChildren: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetChildren)(self, aChildren)
    }


    /// ```text
    /// /**
    ///    * Number of accessible children
    ///    */
    /// ```
    ///

    /// `readonly attribute long childCount;`
    #[inline]
    pub unsafe fn GetChildCount(&self, aChildCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetChildCount)(self, aChildCount)
    }


    /// ```text
    /// /**
    ///    * The 0-based index of this accessible in its parent's list of children,
    ///    * or -1 if this accessible does not have a parent.
    ///    */
    /// ```
    ///

    /// `readonly attribute long indexInParent;`
    #[inline]
    pub unsafe fn GetIndexInParent(&self, aIndexInParent: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetIndexInParent)(self, aIndexInParent)
    }


    /// ```text
    /// /**
    ///    * The unique identifier of the accessible. ID is only guaranteed to be unique
    ///    * per document (Windows IDs are unique even across documents, but that is
        ///    * Windows specific and not exposed to core).
    ///    */
    /// ```
    ///

    /// `readonly attribute long long uniqueID;`
    #[inline]
    pub unsafe fn GetUniqueID(&self, aUniqueID: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetUniqueID)(self, aUniqueID)
    }


    /// ```text
    /// /**
    ///    * The DOM node this nsIAccessible is associated with.
    ///    */
    /// ```
    ///

    /// `readonly attribute Node DOMNode;`
    #[inline]
    pub unsafe fn GetDOMNode(&self, aDOMNode: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDOMNode)(self, aDOMNode)
    }


    /// ```text
    /// /**
    ///     * For remote accessibles the id of the related DOM node.
    ///     */
    /// ```
    ///

    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }


    /// ```text
    /// /**
    ///    * The document accessible that this access node resides in.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleDocument document;`
    #[inline]
    pub unsafe fn GetDocument(&self, aDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult {
        ((*self.vtable).GetDocument)(self, aDocument)
    }


    /// ```text
    /// /**
    ///    * The root document accessible that this access node resides in.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleDocument rootDocument;`
    #[inline]
    pub unsafe fn GetRootDocument(&self, aRootDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult {
        ((*self.vtable).GetRootDocument)(self, aRootDocument)
    }


    /// ```text
    /// /**
    ///    * The language for the current DOM node, e.g. en, de, etc.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString language;`
    #[inline]
    pub unsafe fn GetLanguage(&self, aLanguage: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLanguage)(self, aLanguage)
    }


    /// ```text
    /// /**
    ///    * Accessible name -- the main text equivalent for this node. The name is
    ///    * specified by ARIA or by native markup. Example of ARIA markup is
    ///    * aria-labelledby attribute placed on element of this accessible. Example
    ///    * of native markup is HTML label linked with HTML element of this accessible.
    ///    *
    ///    * Value can be string or null. A null value indicates that AT may attempt to
    ///    * compute the name. Any string value, including the empty string, should be
    ///    * considered author-intentional, and respected.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Accessible value -- a number or a secondary text equivalent for this node
    ///    * Widgets that use role attribute can force a value using the valuenow attribute
    ///    */
    /// ```
    ///

    /// `readonly attribute AString value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


    /// ```text
    /// /**
    ///    * Accessible description -- long text associated with this node
    ///    */
    /// ```
    ///

    /// `readonly attribute AString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///    * Provides localized string of accesskey name, such as Alt+D.
    ///    * The modifier may be affected by user and platform preferences.
    ///    * Usually alt+letter, or just the letter alone for menu items.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString accessKey;`
    #[inline]
    pub unsafe fn GetAccessKey(&self, aAccessKey: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessKey)(self, aAccessKey)
    }


    /// ```text
    /// /**
    ///    * Provides localized string of global keyboard accelerator for default
    ///    * action, such as Ctrl+O for Open file
    ///    */
    /// ```
    ///

    /// `readonly attribute AString keyboardShortcut;`
    #[inline]
    pub unsafe fn GetKeyboardShortcut(&self, aKeyboardShortcut: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetKeyboardShortcut)(self, aKeyboardShortcut)
    }


    /// ```text
    /// /**
    ///    * Enumerated accessible role (see the constants defined in nsIAccessibleRole).
    ///    *
    ///    * @note  The values might depend on platform because of variations. Widgets
    ///    *        can use ARIA role attribute to force the final role.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long role;`
    #[inline]
    pub unsafe fn GetRole(&self, aRole: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRole)(self, aRole)
    }


    /// ```text
    /// /**
    ///    * Accessible states -- bit fields which describe boolean properties of node.
    ///    * Many states are only valid given a certain role attribute that supports
    ///    * them.
    ///    *
    ///    * @param aState - the first bit field (see nsIAccessibleStates::STATE_*
        ///    *                 constants)
    ///    * @param aExtraState - the second bit field
    ///    *                      (see nsIAccessibleStates::EXT_STATE_* constants)
    ///    */
    /// ```
    ///

    /// `void getState (out unsigned long aState, out unsigned long aExtraState);`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u32, aExtraState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState, aExtraState)
    }


    /// ```text
    /// /**
    ///    * Help text associated with node
    ///    *
    ///    * @note As of now, this just returns empty string.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString help;`
    #[inline]
    pub unsafe fn GetHelp(&self, aHelp: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHelp)(self, aHelp)
    }


    /// ```text
    /// /**
    ///    * Focused accessible child of node
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible focusedChild;`
    #[inline]
    pub unsafe fn GetFocusedChild(&self, aFocusedChild: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedChild)(self, aFocusedChild)
    }


    /// ```text
    /// /**
    ///    * Attributes of accessible
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPersistentProperties attributes;`
    #[inline]
    pub unsafe fn GetAttributes(&self, aAttributes: *mut*const nsIPersistentProperties) -> ::nserror::nsresult {
        ((*self.vtable).GetAttributes)(self, aAttributes)
    }


    /// ```text
    /// /**
    ///    * Platform specific interface for accessible
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISupports nativeInterface;`
    #[inline]
    pub unsafe fn GetNativeInterface(&self, aNativeInterface: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetNativeInterface)(self, aNativeInterface)
    }


    /// ```text
    /// /**
    ///    * Returns grouping information. Used for tree items, list items, tab panel
    ///    * labels, radio buttons, etc. Also used for collectons of non-text objects.
    ///    *
    ///    * @param groupLevel - 1-based, similar to ARIA 'level' property
    ///    * @param similarItemsInGroup - 1-based, similar to ARIA 'setsize' property,
    ///    *                              inclusive of the current item
    ///    * @param positionInGroup - 1-based, similar to ARIA 'posinset' property
    ///    */
    /// ```
    ///

    /// `void groupPosition (out long aGroupLevel, out long aSimilarItemsInGroup, out long aPositionInGroup);`
    #[inline]
    pub unsafe fn GroupPosition(&self, aGroupLevel: *mut i32, aSimilarItemsInGroup: *mut i32, aPositionInGroup: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GroupPosition)(self, aGroupLevel, aSimilarItemsInGroup, aPositionInGroup)
    }


    /// ```text
    /// /**
    ///    * Accessible child which contains the coordinate at (x, y) in screen pixels.
    ///    * If the point is in the current accessible but not in a child, the
    ///    * current accessible will be returned.
    ///    * If the point is in neither the current accessible or a child, then
    ///    * null will be returned.
    ///    *
    ///    * @param x  screen's x coordinate
    ///    * @param y  screen's y coordinate
    ///    * @return   the direct accessible child containing the given point
    ///    */
    /// ```
    ///

    /// `nsIAccessible getChildAtPoint (in long x, in long y);`
    #[inline]
    pub unsafe fn GetChildAtPoint(&self, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetChildAtPoint)(self, x, y, _retval)
    }


    /// ```text
    /// /**
    ///    * Deepest accessible child which contains the coordinate at (x, y) in screen
    ///    * pixels. If the point is in the current accessible but not in a child, the
    ///    * current accessible will be returned. If the point is in neither the current
    ///    * accessible or a child, then null will be returned.
    ///    *
    ///    * @param x  screen's x coordinate
    ///    * @param y  screen's y coordinate
    ///    * @return   the deepest accessible child containing the given point
    ///    */
    /// ```
    ///

    /// `nsIAccessible getDeepestChildAtPoint (in long x, in long y);`
    #[inline]
    pub unsafe fn GetDeepestChildAtPoint(&self, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetDeepestChildAtPoint)(self, x, y, _retval)
    }


    /// ```text
    /// /**
    ///    * Like GetDeepestChildAtPoint, but restricted to the current process.
    ///    * If the point is within a remote document, the accessible for the browser
    ///    * element containing that document will be returned; i.e. this will not
    ///    * descend into the document. If called on an accessible inside a remote
    ///    * document, this will fail.
    ///    *
    ///    * @param x  screen's x coordinate
    ///    * @param y  screen's y coordinate
    ///    * @return   the deepest accessible child in this process containing the given
    ///    *           point
    ///    */
    /// ```
    ///

    /// `nsIAccessible getDeepestChildAtPointInProcess (in long x, in long y);`
    #[inline]
    pub unsafe fn GetDeepestChildAtPointInProcess(&self, x: i32, y: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetDeepestChildAtPointInProcess)(self, x, y, _retval)
    }


    /// ```text
    /// /**
    ///    * Nth accessible child using zero-based index or last child if index less than zero
    ///    */
    /// ```
    ///

    /// `nsIAccessible getChildAt (in long aChildIndex);`
    #[inline]
    pub unsafe fn GetChildAt(&self, aChildIndex: i32, _retval: *mut *const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetChildAt)(self, aChildIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Return accessible relation by the given relation type (see.
        ///    * constants defined in nsIAccessibleRelation).
    ///    */
    /// ```
    ///

    /// `nsIAccessibleRelation getRelationByType (in unsigned long aRelationType);`
    #[inline]
    pub unsafe fn GetRelationByType(&self, aRelationType: u32, _retval: *mut*const nsIAccessibleRelation) -> ::nserror::nsresult {
        ((*self.vtable).GetRelationByType)(self, aRelationType, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns multiple accessible relations for this object.
    ///    */
    /// ```
    ///

    /// `nsIArray getRelations ();`
    #[inline]
    pub unsafe fn GetRelations(&self, _retval: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetRelations)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Return accessible's x and y coordinates relative to the screen and
    ///    * accessible's width and height in Dev pixels.
    ///    */
    /// ```
    ///

    /// `void getBounds (out long x, out long y, out long width, out long height);`
    #[inline]
    pub unsafe fn GetBounds(&self, x: *mut i32, y: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetBounds)(self, x, y, width, height)
    }


    /// ```text
    /// /**
    ///    * Return accessible's x and y coordinates relative to the screen and
    ///    * accessible's width and height in CSS pixels.
    ///    */
    /// ```
    ///

    /// `void getBoundsInCSSPixels (out long aX, out long aY, out long aWidth, out long aHeight);`
    #[inline]
    pub unsafe fn GetBoundsInCSSPixels(&self, aX: *mut i32, aY: *mut i32, aWidth: *mut i32, aHeight: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetBoundsInCSSPixels)(self, aX, aY, aWidth, aHeight)
    }


    /// ```text
    /// /**
    ///    * Add or remove this accessible to the current selection
    ///    */
    /// ```
    ///

    /// `void setSelected (in boolean isSelected);`
    #[inline]
    pub unsafe fn SetSelected(&self, isSelected: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSelected)(self, isSelected)
    }


    /// ```text
    /// /**
    ///    * Select this accessible node only
    ///    */
    /// ```
    ///

    /// `void takeSelection ();`
    #[inline]
    pub unsafe fn TakeSelection(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TakeSelection)(self, )
    }


    /// ```text
    /// /**
    ///    * Focus this accessible node,
    ///    * The state STATE_FOCUSABLE indicates whether this node is normally focusable.
    ///    * It is the callers responsibility to determine whether this node is focusable.
    ///    * accTakeFocus on a node that is not normally focusable (such as a table),
    ///    * will still set focus on that node, although normally that will not be visually
    ///    * indicated in most style sheets.
    ///    */
    /// ```
    ///

    /// `void takeFocus ();`
    #[inline]
    pub unsafe fn TakeFocus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TakeFocus)(self, )
    }


    /// ```text
    /// /**
    ///    * The number of accessible actions associated with this accessible
    ///    */
    /// ```
    ///

    /// `readonly attribute uint8_t actionCount;`
    #[inline]
    pub unsafe fn GetActionCount(&self, aActionCount: *mut uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).GetActionCount)(self, aActionCount)
    }


    /// ```text
    /// /**
    ///    * The name of the accessible action at the given zero-based index
    ///    */
    /// ```
    ///

    /// `AString getActionName (in uint8_t index);`
    #[inline]
    pub unsafe fn GetActionName(&self, index: uint8_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetActionName)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * The description of the accessible action at the given zero-based index
    ///    */
    /// ```
    ///

    /// `AString getActionDescription (in uint8_t aIndex);`
    #[inline]
    pub unsafe fn GetActionDescription(&self, aIndex: uint8_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetActionDescription)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Perform the accessible action at the given zero-based index
    ///    * Action number 0 is the default action
    ///    */
    /// ```
    ///

    /// `void doAction (in uint8_t index);`
    #[inline]
    pub unsafe fn DoAction(&self, index: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).DoAction)(self, index)
    }


    /// ```text
    /// /**
    ///    * Makes an object visible on screen.
    ///    *
    ///    * @param scrollType - defines where the object should be placed on
    ///    *                     the screen (see nsIAccessibleScrollType for
        ///    *                     available constants).
    ///    */
    /// ```
    ///

    /// `[can_run_script] void scrollTo (in unsigned long aScrollType);`
    #[inline]
    pub unsafe fn ScrollTo(&self, aScrollType: u32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollTo)(self, aScrollType)
    }


    /// ```text
    /// /**
    ///    * Moves the top left of an object to a specified location.
    ///    *
    ///    * @param coordinateType [in] - specifies whether the coordinates are relative to
    ///    *                         the screen or the parent object (for available
        ///    *                         constants refer to nsIAccessibleCoordinateType)
    ///    * @param x [in] - defines the x coordinate
    ///    * @param y [in] - defines the y coordinate
    ///   */
    /// ```
    ///

    /// `void scrollToPoint (in unsigned long coordinateType, in long x, in long y);`
    #[inline]
    pub unsafe fn ScrollToPoint(&self, coordinateType: u32, x: i32, y: i32) -> ::nserror::nsresult {
        ((*self.vtable).ScrollToPoint)(self, coordinateType, x, y)
    }


    /// ```text
    /// /**
    ///    * Dispatches an ANNOUNCEMENT event with this accessible as target.
    ///    *
    ///    * @param announcement [in] - string to use in announcement.
    ///    * @param priority [in] - priority for announcement, could be
    ///    *                      nsIAccessibleAnnouncementEvent.POLITE or
    ///    *                      nsIAccessibleAnnouncementEvent.ASSERTIVE.
    ///    */
    /// ```
    ///

    /// `void announce (in AString announcement, in unsigned short priority);`
    #[inline]
    pub unsafe fn Announce(&self, announcement: *const ::nsstring::nsAString, priority: u16) -> ::nserror::nsresult {
        ((*self.vtable).Announce)(self, announcement, priority)
    }


}


