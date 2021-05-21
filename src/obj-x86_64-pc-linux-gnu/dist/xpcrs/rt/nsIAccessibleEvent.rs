//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleEvent.idl
//


/// `interface nsIAccessibleEvent : nsISupports`
///

/// ```text
/// /**
///  * An interface for accessibility events listened to
///  * by in-process accessibility clients, which can be used
///  * to find out how to get accessibility and DOM interfaces for
///  * the event and its target. To listen to in-process accessibility invents,
///  * make your object an nsIObserver, and listen for accessible-event by
///  * using code something like this:
///  *   nsCOMPtr<nsIObserverService> observerService =
///  *     do_GetService("@mozilla.org/observer-service;1", &rv);
///  *   if (NS_SUCCEEDED(rv))
///  *     rv = observerService->AddObserver(this, "accessible-event", PR_TRUE);
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleEvent {
    vtable: *const nsIAccessibleEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleEvent.
unsafe impl XpCom for nsIAccessibleEvent {
    const IID: nsIID = nsID(0x20c69a40, 0x6c2c, 0x42a3,
        [0xa5, 0x78, 0x6f, 0x44, 0x73, 0xaa, 0xb9, 0xdd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleEvent`.
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleEventCoerce for nsIAccessibleEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self {
        v
    }
}

impl nsIAccessibleEvent {
    /// Cast this `nsIAccessibleEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleEvent {
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
impl<T: nsISupportsCoerce> nsIAccessibleEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long eventType; */
    pub GetEventType: unsafe extern "system" fn (this: *const nsIAccessibleEvent, aEventType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible accessible; */
    pub GetAccessible: unsafe extern "system" fn (this: *const nsIAccessibleEvent, aAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleDocument accessibleDocument; */
    pub GetAccessibleDocument: unsafe extern "system" fn (this: *const nsIAccessibleEvent, aAccessibleDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult,

    /* readonly attribute Node DOMNode; */
    pub GetDOMNode: unsafe extern "system" fn (this: *const nsIAccessibleEvent, aDOMNode: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean isFromUserInput; */
    pub GetIsFromUserInput: unsafe extern "system" fn (this: *const nsIAccessibleEvent, aIsFromUserInput: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleEvent {
    /// ```text
    /// /**
    ///    * An object has been created.
    ///    */
    /// ```
    ///

    pub const EVENT_SHOW: i64 = 1;

    /// ```text
    /// /**
    ///    * An object has been destroyed.
    ///    */
    /// ```
    ///

    pub const EVENT_HIDE: i64 = 2;

    /// ```text
    /// /**
    ///    * An object's children have changed
    ///    */
    /// ```
    ///

    pub const EVENT_REORDER: i64 = 3;

    /// ```text
    /// /**
    ///    * The active descendant of a component has changed. The active descendant
    ///    * is used in objects with transient children.
    ///    */
    /// ```
    ///

    pub const EVENT_ACTIVE_DECENDENT_CHANGED: i64 = 4;

    /// ```text
    /// /**
    ///    * An object has received the keyboard focus.
    ///    */
    /// ```
    ///

    pub const EVENT_FOCUS: i64 = 5;

    /// ```text
    /// /**
    ///    * An object's state has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_STATE_CHANGE: i64 = 6;

    /// ```text
    /// /**
    ///    * An object has changed location, shape, or size.
    ///    */
    /// ```
    ///

    pub const EVENT_LOCATION_CHANGE: i64 = 7;

    /// ```text
    /// /**
    ///    * An object's Name property has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_NAME_CHANGE: i64 = 8;

    /// ```text
    /// /**
    ///    * An object's Description property has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_DESCRIPTION_CHANGE: i64 = 9;

    /// ```text
    /// /**
    ///    * An object's numeric Value has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_VALUE_CHANGE: i64 = 10;

    /// ```text
    /// /**
    ///    * An object's help has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_HELP_CHANGE: i64 = 11;

    /// ```text
    /// /**
    ///    * An object's default action has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_DEFACTION_CHANGE: i64 = 12;

    /// ```text
    /// /**
    ///    * An object's action has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_ACTION_CHANGE: i64 = 13;

    /// ```text
    /// /**
    ///    * An object's keyboard shortcut has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_ACCELERATOR_CHANGE: i64 = 14;

    /// ```text
    /// /**
    ///    * The selection within a container object has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_SELECTION: i64 = 15;

    /// ```text
    /// /**
    ///    * An item within a container object has been added to the selection.
    ///    */
    /// ```
    ///

    pub const EVENT_SELECTION_ADD: i64 = 16;

    /// ```text
    /// /**
    ///    * An item within a container object has been removed from the selection.
    ///    */
    /// ```
    ///

    pub const EVENT_SELECTION_REMOVE: i64 = 17;

    /// ```text
    /// /**
    ///    * Numerous selection changes have occurred within a container object.
    ///    */
    /// ```
    ///

    pub const EVENT_SELECTION_WITHIN: i64 = 18;

    /// ```text
    /// /**
    ///    * An alert has been generated. Server applications send this event when a
    ///    * user needs to know that a user interface element has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_ALERT: i64 = 19;

    /// ```text
    /// /**
    ///    * The foreground window has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_FOREGROUND: i64 = 20;

    /// ```text
    /// /**
    ///    * A menu item on the menu bar has been selected.
    ///    */
    /// ```
    ///

    pub const EVENT_MENU_START: i64 = 21;

    /// ```text
    /// /**
    ///    * A menu from the menu bar has been closed.
    ///    */
    /// ```
    ///

    pub const EVENT_MENU_END: i64 = 22;

    /// ```text
    /// /**
    ///    * A pop-up menu has been displayed.
    ///    */
    /// ```
    ///

    pub const EVENT_MENUPOPUP_START: i64 = 23;

    /// ```text
    /// /**
    ///    * A pop-up menu has been closed.
    ///    */
    /// ```
    ///

    pub const EVENT_MENUPOPUP_END: i64 = 24;

    /// ```text
    /// /**
    ///    * A window has received mouse capture.
    ///    */
    /// ```
    ///

    pub const EVENT_CAPTURE_START: i64 = 25;

    /// ```text
    /// /**
    ///    * A window has lost mouse capture.
    ///    */
    /// ```
    ///

    pub const EVENT_CAPTURE_END: i64 = 26;

    /// ```text
    /// /**
    ///    * A window is being moved or resized.
    ///    */
    /// ```
    ///

    pub const EVENT_MOVESIZE_START: i64 = 27;

    /// ```text
    /// /**
    ///   * The movement or resizing of a window has finished
    ///   */
    /// ```
    ///

    pub const EVENT_MOVESIZE_END: i64 = 28;

    /// ```text
    /// /**
    ///    * A window has entered context-sensitive Help mode
    ///    */
    /// ```
    ///

    pub const EVENT_CONTEXTHELP_START: i64 = 29;

    /// ```text
    /// /**
    ///    * A window has exited context-sensitive Help mode
    ///    */
    /// ```
    ///

    pub const EVENT_CONTEXTHELP_END: i64 = 30;

    /// ```text
    /// /**
    ///    * An application is about to enter drag-and-drop mode
    ///    */
    /// ```
    ///

    pub const EVENT_DRAGDROP_START: i64 = 31;

    /// ```text
    /// /**
    ///    * An application is about to exit drag-and-drop mode
    ///    */
    /// ```
    ///

    pub const EVENT_DRAGDROP_END: i64 = 32;

    /// ```text
    /// /**
    ///    * A dialog box has been displayed
    ///    */
    /// ```
    ///

    pub const EVENT_DIALOG_START: i64 = 33;

    /// ```text
    /// /**
    ///    * A dialog box has been closed
    ///    */
    /// ```
    ///

    pub const EVENT_DIALOG_END: i64 = 34;

    /// ```text
    /// /**
    ///    * Scrolling has started on a scroll bar
    ///    */
    /// ```
    ///

    pub const EVENT_SCROLLING_START: i64 = 35;

    /// ```text
    /// /**
    ///    * Scrolling has ended on a scroll bar
    ///    */
    /// ```
    ///

    pub const EVENT_SCROLLING_END: i64 = 36;

    /// ```text
    /// /**
    ///    * A window object is about to be minimized or maximized
    ///    */
    /// ```
    ///

    pub const EVENT_MINIMIZE_START: i64 = 37;

    /// ```text
    /// /**
    ///    * A window object has been minimized or maximized
    ///    */
    /// ```
    ///

    pub const EVENT_MINIMIZE_END: i64 = 38;

    /// ```text
    /// /**
    ///    * The loading of the document has completed.
    ///    */
    /// ```
    ///

    pub const EVENT_DOCUMENT_LOAD_COMPLETE: i64 = 39;

    /// ```text
    /// /**
    ///    * The document contents are being reloaded.
    ///    */
    /// ```
    ///

    pub const EVENT_DOCUMENT_RELOAD: i64 = 40;

    /// ```text
    /// /**
    ///    * The loading of the document was interrupted.
    ///    */
    /// ```
    ///

    pub const EVENT_DOCUMENT_LOAD_STOPPED: i64 = 41;

    /// ```text
    /// /**
    ///    * The document wide attributes of the document object have changed.
    ///    */
    /// ```
    ///

    pub const EVENT_DOCUMENT_ATTRIBUTES_CHANGED: i64 = 42;

    /// ```text
    /// /**
    ///    * The contents of the document have changed.
    ///    */
    /// ```
    ///

    pub const EVENT_DOCUMENT_CONTENT_CHANGED: i64 = 43;


    pub const EVENT_PROPERTY_CHANGED: i64 = 44;

    /// ```text
    /// /**
    ///    * A slide changed in a presentation document or a page boundary was
    ///    * crossed in a word processing document.
    ///    */
    /// ```
    ///

    pub const EVENT_PAGE_CHANGED: i64 = 45;

    /// ```text
    /// /**
    ///    * A text object's attributes changed.
    ///    * Also see EVENT_OBJECT_ATTRIBUTE_CHANGED.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_ATTRIBUTE_CHANGED: i64 = 46;

    /// ```text
    /// /**
    ///    * The caret has moved to a new position.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_CARET_MOVED: i64 = 47;

    /// ```text
    /// /**
    ///    * This event indicates general text changes, i.e. changes to text that is
    ///    * exposed through the IAccessibleText and IAccessibleEditableText interfaces.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_CHANGED: i64 = 48;

    /// ```text
    /// /**
    ///    * Text was inserted.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_INSERTED: i64 = 49;

    /// ```text
    /// /**
    ///    * Text was removed.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_REMOVED: i64 = 50;

    /// ```text
    /// /**
    ///    * Text was updated.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_UPDATED: i64 = 51;

    /// ```text
    /// /**
    ///    * The text selection changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_SELECTION_CHANGED: i64 = 52;

    /// ```text
    /// /**
    ///    * A visibile data event indicates the change of the visual appearance
    ///    * of an accessible object.  This includes for example most of the
    ///    * attributes available via the IAccessibleComponent interface.
    ///    */
    /// ```
    ///

    pub const EVENT_VISIBLE_DATA_CHANGED: i64 = 53;

    /// ```text
    /// /**
    ///    * The caret moved from one column to the next.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_COLUMN_CHANGED: i64 = 54;

    /// ```text
    /// /**
    ///    * The caret moved from one section to the next.
    ///    */
    /// ```
    ///

    pub const EVENT_SECTION_CHANGED: i64 = 55;

    /// ```text
    /// /**
    ///    * A table caption changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_CAPTION_CHANGED: i64 = 56;

    /// ```text
    /// /**
    ///    * A table's data changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_MODEL_CHANGED: i64 = 57;

    /// ```text
    /// /**
    ///    * A table's summary changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_SUMMARY_CHANGED: i64 = 58;

    /// ```text
    /// /**
    ///    * A table's row description changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_ROW_DESCRIPTION_CHANGED: i64 = 59;

    /// ```text
    /// /**
    ///    * A table's row header changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_ROW_HEADER_CHANGED: i64 = 60;


    pub const EVENT_TABLE_ROW_INSERT: i64 = 61;


    pub const EVENT_TABLE_ROW_DELETE: i64 = 62;


    pub const EVENT_TABLE_ROW_REORDER: i64 = 63;

    /// ```text
    /// /**
    ///    * A table's column description changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED: i64 = 64;

    /// ```text
    /// /**
    ///    * A table's column header changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_COLUMN_HEADER_CHANGED: i64 = 65;


    pub const EVENT_TABLE_COLUMN_INSERT: i64 = 66;


    pub const EVENT_TABLE_COLUMN_DELETE: i64 = 67;


    pub const EVENT_TABLE_COLUMN_REORDER: i64 = 68;


    pub const EVENT_WINDOW_ACTIVATE: i64 = 69;


    pub const EVENT_WINDOW_CREATE: i64 = 70;


    pub const EVENT_WINDOW_DEACTIVATE: i64 = 71;


    pub const EVENT_WINDOW_DESTROY: i64 = 72;


    pub const EVENT_WINDOW_MAXIMIZE: i64 = 73;


    pub const EVENT_WINDOW_MINIMIZE: i64 = 74;


    pub const EVENT_WINDOW_RESIZE: i64 = 75;


    pub const EVENT_WINDOW_RESTORE: i64 = 76;

    /// ```text
    /// /**
    ///    * The ending index of this link within the containing string has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERLINK_END_INDEX_CHANGED: i64 = 77;

    /// ```text
    /// /**
    ///    * The number of anchors assoicated with this hyperlink object has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED: i64 = 78;

    /// ```text
    /// /**
    ///    * The hyperlink selected state changed from selected to unselected or
    ///    * from unselected to selected.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERLINK_SELECTED_LINK_CHANGED: i64 = 79;

    /// ```text
    /// /**
    ///    * One of the links associated with the hypertext object has been activated.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERTEXT_LINK_ACTIVATED: i64 = 80;

    /// ```text
    /// /**
    ///    * One of the links associated with the hypertext object has been selected.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERTEXT_LINK_SELECTED: i64 = 81;

    /// ```text
    /// /**
    ///    * The starting index of this link within the containing string has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERLINK_START_INDEX_CHANGED: i64 = 82;

    /// ```text
    /// /**
    ///    * Focus has changed from one hypertext object to another, or focus moved
    ///    * from a non-hypertext object to a hypertext object, or focus moved from a
    ///    * hypertext object to a non-hypertext object.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERTEXT_CHANGED: i64 = 83;

    /// ```text
    /// /**
    ///    * The number of hyperlinks associated with a hypertext object changed.
    ///    */
    /// ```
    ///

    pub const EVENT_HYPERTEXT_NLINKS_CHANGED: i64 = 84;

    /// ```text
    /// /**
    ///    * An object's attributes changed. Also see EVENT_TEXT_ATTRIBUTE_CHANGED.
    ///    */
    /// ```
    ///

    pub const EVENT_OBJECT_ATTRIBUTE_CHANGED: i64 = 85;

    /// ```text
    /// /**
    ///    * A cursorable's virtual cursor has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_VIRTUALCURSOR_CHANGED: i64 = 86;

    /// ```text
    /// /**
    ///    * An object's text Value has changed.
    ///    */
    /// ```
    ///

    pub const EVENT_TEXT_VALUE_CHANGE: i64 = 87;

    /// ```text
    /// /**
    ///    * An accessible's viewport is scrolling.
    ///    */
    /// ```
    ///

    pub const EVENT_SCROLLING: i64 = 88;

    /// ```text
    /// /**
    ///    * An accessible is making an explicit announcement.
    ///    */
    /// ```
    ///

    pub const EVENT_ANNOUNCEMENT: i64 = 89;

    /// ```text
    /// /**
    ///    * A live region has been introduced. Mac only.
    ///    */
    /// ```
    ///

    pub const EVENT_LIVE_REGION_ADDED: i64 = 90;

    /// ```text
    /// /**
    ///    * A live region has been removed (aria-live attribute changed). Mac Only.
    ///    */
    /// ```
    ///

    pub const EVENT_LIVE_REGION_REMOVED: i64 = 91;

    /// ```text
    /// /**
    ///    * A style change has occured on a table that may cause it to
    ///    * change from data to layout table, or the reverse.
    ///    */
    /// ```
    ///

    pub const EVENT_TABLE_STYLING_CHANGED: i64 = 92;

    /// ```text
    /// /**
    ///    * Help make sure event map does not get out-of-line.
    ///    */
    /// ```
    ///

    pub const EVENT_LAST_ENTRY: i64 = 93;

    /// ```text
    /// /**
    ///    * The type of event, based on the enumerated event values
    ///    * defined in this interface.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long eventType;`
    #[inline]
    pub unsafe fn GetEventType(&self, aEventType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEventType)(self, aEventType)
    }


    /// ```text
    /// /**
    ///    * The nsIAccessible associated with the event.
    ///    * May return null if no accessible is available
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible accessible;`
    #[inline]
    pub unsafe fn GetAccessible(&self, aAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessible)(self, aAccessible)
    }


    /// ```text
    /// /**
    ///    * The nsIAccessibleDocument that the event target nsIAccessible
    ///    * resides in. This can be used to get the DOM window,
    ///    * the DOM document and the window handler, among other things.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleDocument accessibleDocument;`
    #[inline]
    pub unsafe fn GetAccessibleDocument(&self, aAccessibleDocument: *mut*const nsIAccessibleDocument) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessibleDocument)(self, aAccessibleDocument)
    }


    /// ```text
    /// /**
    ///    * The Node associated with the event
    ///    * May return null if accessible for event has been shut down
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
    ///    * Returns true if the event was caused by explicit user input,
    ///    * as opposed to purely originating from a timer or mouse movement
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isFromUserInput;`
    #[inline]
    pub unsafe fn GetIsFromUserInput(&self, aIsFromUserInput: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFromUserInput)(self, aIsFromUserInput)
    }


}


