//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsITextInputProcessorCallback.idl
//


/// `interface nsITextInputProcessorNotification : nsISupports`
///

/// ```text
/// /**
///  * nsITextInputProcessorNotification stores the type of notification to IME and
///  * its detail.  See each explanation of attribute for the detail.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITextInputProcessorNotification {
    vtable: *const nsITextInputProcessorNotificationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITextInputProcessorNotification.
unsafe impl XpCom for nsITextInputProcessorNotification {
    const IID: nsIID = nsID(0xc0ce1add, 0x82bb, 0x45ab,
        [0xb9, 0x9a, 0x42, 0xcf, 0xba, 0x7f, 0xd5, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITextInputProcessorNotification {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITextInputProcessorNotification.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITextInputProcessorNotificationCoerce {
    /// Cheaply cast a value of this type from a `nsITextInputProcessorNotification`.
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITextInputProcessorNotificationCoerce for nsITextInputProcessorNotification {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self {
        v
    }
}

impl nsITextInputProcessorNotification {
    /// Cast this `nsITextInputProcessorNotification` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITextInputProcessorNotificationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITextInputProcessorNotification {
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
impl<T: nsISupportsCoerce> nsITextInputProcessorNotificationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorNotification) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITextInputProcessorNotification
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITextInputProcessorNotificationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long offset; */
    pub GetOffset: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aOffset: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString text; */
    pub GetText: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean collapsed; */
    pub GetCollapsed: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aCollapsed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute uint32_t length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aLength: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean reversed; */
    pub GetReversed: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aReversed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute ACString writingMode; */
    pub GetWritingMode: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aWritingMode: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean causedByComposition; */
    pub GetCausedByComposition: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aCausedByComposition: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean causedBySelectionEvent; */
    pub GetCausedBySelectionEvent: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aCausedBySelectionEvent: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean occurredDuringComposition; */
    pub GetOccurredDuringComposition: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aOccurredDuringComposition: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long removedLength; */
    pub GetRemovedLength: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aRemovedLength: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long addedLength; */
    pub GetAddedLength: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aAddedLength: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean causedOnlyByComposition; */
    pub GetCausedOnlyByComposition: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aCausedOnlyByComposition: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean includingChangesDuringComposition; */
    pub GetIncludingChangesDuringComposition: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aIncludingChangesDuringComposition: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean includingChangesWithoutComposition; */
    pub GetIncludingChangesWithoutComposition: unsafe extern "system" fn (this: *const nsITextInputProcessorNotification, aIncludingChangesWithoutComposition: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITextInputProcessorNotification {

    /// ```text
    /// /**
    ///    * type attribute represents what's notified or requested.  Value must be
    ///    * one of following values:
    ///    *
    ///    * "request-to-commit"  (required to be handled)
    ///    *   This is requested when Gecko believes that active composition should be
    ///    *   committed.  nsITextInputProcessorCallback::onNotify() has to handle this
    ///    *   notification.
    ///    *
    ///    * "request-to-cancel" (required to be handled)
    ///    *   This is requested when Gecko believes that active composition should be
    ///    *   canceled.  I.e., composition should be committed with empty string.
    ///    *   nsITextInputProcessorCallback::onNotify() has to handle this
    ///    *   notification.
    ///    *
    ///    * "notify-end-input-transaction" (optional)
    ///    *   This is notified when the callback is detached from
    ///    *   nsITextInputProcessor.  I.e., the TextInputProcessor lost the rights
    ///    *   to input text and needs to call .beginInputTransaction() before next
    ///    *   input.
    ///    *
    ///    * "notify-focus" (optional)
    ///    *   This is notified when an editable editor gets focus and Gecko starts
    ///    *   to observe changes in the content. E.g., selection changes.
    ///    *   IME shouldn't change DOM tree, focus nor something when this is notified.
    ///    *
    ///    * "notify-blur" (optional)
    ///    *   This is notified when an editable editor loses focus and Gecko stops
    ///    *   observing the changes in the content.
    ///    *
    ///    * "notify-text-change" (optional)
    ///    *   This is notified when text in the focused editor is modified.
    ///    *   Some attributes below are available to retrieve the detail.
    ///    *   IME shouldn't change DOM tree, focus nor something when this is notified.
    ///    *   Note that when there is no chance to notify you of some text changes
    ///    *   safely, this represents all changes as a change.
    ///    *
    ///    * "notify-selection-change" (optional)
    ///    *   This is notified when selection in the focused editor is changed.
    ///    *   Some attributes below are available to retrieve the detail.
    ///    *   IME shouldn't change DOM tree, focus nor something when this is notified.
    ///    *   Note that when there was no chance to notify you of this safely, this
    ///    *   represents the latest selection change.
    ///    *
    ///    * "notify-position-change" (optional)
    ///    *   This is notified when layout is changed in the editor or the window
    ///    *   is moved.
    ///    *   IME shouldn't change DOM tree, focus nor something when this is notified.
    ///    *   Note that when there was no chance to notify you of this safely, this
    ///    *   represents the latest layout change.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * Be careful, line breakers in the editor are treated as native line
    ///    * breakers.  I.e., on Windows, a line breaker is "\r\n" (CRLF).  On the
    ///    * others, it is "\n" (LF).  If you want TextInputProcessor to treat line
    ///    * breakers on Windows as XP line breakers (LF), please file a bug with
    ///    * the reason why you need the behavior.
    ///    */
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change" or
    ///    * "notify-selection-change".
    ///    * This is offset of the start of modified text range if type is
    ///    * "notify-text-change".  Or offset of start of selection if type is
    ///    * "notify-selection-change".
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long offset;`
    #[inline]
    pub unsafe fn GetOffset(&self, aOffset: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetOffset)(self, aOffset)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * This is selected text.  I.e., the length is selected length and
    ///    * it's empty if the selection is collapsed.
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
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * This is set to true when the selection is collapsed.  Otherwise, false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean collapsed;`
    #[inline]
    pub unsafe fn GetCollapsed(&self, aCollapsed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCollapsed)(self, aCollapsed)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * This is selected length.  I.e., if this is 0, collapsed is set to true.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * When selection is created from latter point to former point, this is
    ///    * set to true.  Otherwise, false.
    ///    * I.e., if this is true, offset + length is the anchor of selection.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean reversed;`
    #[inline]
    pub unsafe fn GetReversed(&self, aReversed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetReversed)(self, aReversed)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * This indicates the start of the selection's writing mode.
    ///    * The value can be "horizontal-tb", "vertical-rl" or "vertical-lr".
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString writingMode;`
    #[inline]
    pub unsafe fn GetWritingMode(&self, aWritingMode: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetWritingMode)(self, aWritingMode)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * If the selection change was caused by composition, this is set to true.
    ///    * Otherwise, false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean causedByComposition;`
    #[inline]
    pub unsafe fn GetCausedByComposition(&self, aCausedByComposition: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCausedByComposition)(self, aCausedByComposition)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * If the selection change was caused by selection event, this is set to true.
    ///    * Otherwise, false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean causedBySelectionEvent;`
    #[inline]
    pub unsafe fn GetCausedBySelectionEvent(&self, aCausedBySelectionEvent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCausedBySelectionEvent)(self, aCausedBySelectionEvent)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-selection-change".
    ///    * If the selection change occurred during composition, this is set to true.
    ///    * Otherwise, false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean occurredDuringComposition;`
    #[inline]
    pub unsafe fn GetOccurredDuringComposition(&self, aOccurredDuringComposition: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOccurredDuringComposition)(self, aOccurredDuringComposition)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change".
    ///    * This is removed text length by the change(s).  If this is empty, new text
    ///    * was just inserted.  Otherwise, the text is replaced with new text.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long removedLength;`
    #[inline]
    pub unsafe fn GetRemovedLength(&self, aRemovedLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRemovedLength)(self, aRemovedLength)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change".
    ///    * This is added text length by the change(s).  If this is empty, old text
    ///    * was just deleted.  Otherwise, the text replaces the old text.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long addedLength;`
    #[inline]
    pub unsafe fn GetAddedLength(&self, aAddedLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetAddedLength)(self, aAddedLength)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change".
    ///    * If the text change(s) was caused only by composition, this is set to true.
    ///    * Otherwise, false.  I.e., if one of text changes are caused by JS or
    ///    * modifying without composition, this is set to false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean causedOnlyByComposition;`
    #[inline]
    pub unsafe fn GetCausedOnlyByComposition(&self, aCausedOnlyByComposition: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCausedOnlyByComposition)(self, aCausedOnlyByComposition)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change".
    ///    * If at least one text change not caused by composition occurred during
    ///    * composition, this is set to true.  Otherwise, false.
    ///    * Note that this is set to false when new change is caused by neither
    ///    * composition nor occurred during composition because it's outdated for
    ///    * new composition.
    ///    * In other words, when text changes not caused by composition occurred
    ///    * during composition and it may cause committing composition, this is
    ///    * set to true.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean includingChangesDuringComposition;`
    #[inline]
    pub unsafe fn GetIncludingChangesDuringComposition(&self, aIncludingChangesDuringComposition: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIncludingChangesDuringComposition)(self, aIncludingChangesDuringComposition)
    }


    /// ```text
    /// /**
    ///    * This attribute has a valid value when type is "notify-text-change".
    ///    * If at least one text change occurred when there was no composition, this
    ///    * is set to true.  Otherwise, false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean includingChangesWithoutComposition;`
    #[inline]
    pub unsafe fn GetIncludingChangesWithoutComposition(&self, aIncludingChangesWithoutComposition: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIncludingChangesWithoutComposition)(self, aIncludingChangesWithoutComposition)
    }


}


/// `interface nsITextInputProcessorCallback : nsISupports`
///

/// ```text
/// /**
///  * nsITextInputProcessorCallback is a callback interface for JS to implement
///  * IME.  IME implemented by JS can implement onNotify() function and must send
///  * it to nsITextInputProcessor at initializing.  Then, onNotify() will be
///  * called with nsITextInputProcessorNotification instance.
///  * The reason why onNotify() uses string simply is that if we will support
///  * other notifications such as text changes and selection changes, we need to
///  * notify IME of some other information.  Then, only changing
///  * nsITextInputProcessorNotification interface is better for compatibility.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITextInputProcessorCallback {
    vtable: *const nsITextInputProcessorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITextInputProcessorCallback.
unsafe impl XpCom for nsITextInputProcessorCallback {
    const IID: nsIID = nsID(0x23d5f242, 0xadb5, 0x46f1,
        [0x87, 0x66, 0x90, 0xd1, 0xbf, 0x03, 0x83, 0xdf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITextInputProcessorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITextInputProcessorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITextInputProcessorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsITextInputProcessorCallback`.
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITextInputProcessorCallbackCoerce for nsITextInputProcessorCallback {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self {
        v
    }
}

impl nsITextInputProcessorCallback {
    /// Cast this `nsITextInputProcessorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITextInputProcessorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITextInputProcessorCallback {
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
impl<T: nsISupportsCoerce> nsITextInputProcessorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITextInputProcessorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITextInputProcessorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITextInputProcessorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification); */
    pub OnNotify: unsafe extern "system" fn (this: *const nsITextInputProcessorCallback, aTextInputProcessor: *const nsITextInputProcessor, aNotification: *const nsITextInputProcessorNotification, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITextInputProcessorCallback {

    /// ```text
    /// /**
    ///    * When Gecko notifies IME of something or requests something to IME,
    ///    * this is called.
    ///    *
    ///    * @param aTextInputProcessor Reference to the nsITextInputProcessor service
    ///    *                            which is the original receiver of the request
    ///    *                            or notification.
    ///    * @param aNotification       Stores type of notifications and additional
    ///    *                            information.
    ///    * @return                    Return true if it succeeded or does nothing.
    ///    *                            Otherwise, return false.
    ///    *
    ///    * Example #1 The simplest implementation of nsITextInputProcessorCallback is:
    ///    *
    ///    *   function simpleCallback(aTIP, aNotification)
    ///    *   {
        ///    *     try {
            ///    *       switch (aNotification.type) {
                ///    *         case "request-to-commit":
                ///    *           aTIP.commitComposition();
                ///    *           break;
                ///    *         case "request-to-cancel":
                ///    *           aTIP.cancelComposition();
                ///    *           break;
                ///    *       }
            ///    *     } catch (e) {
            ///    *       return false;
            ///    *     }
        ///    *     return true;
        ///    *   }
    ///    *
    ///    *   var TIP = Components.classes["@mozilla.org/text-input-processor;1"].
    ///    *               createInstance(Components.interfaces.nsITextInputProcessor);
    ///    *   if (!TIP.init(window, simpleCallback)) {
        ///    *     return;
        ///    *   }
    ///    */
    /// ```
    ///

    /// `boolean onNotify (in nsITextInputProcessor aTextInputProcessor, in nsITextInputProcessorNotification aNotification);`
    #[inline]
    pub unsafe fn OnNotify(&self, aTextInputProcessor: *const nsITextInputProcessor, aNotification: *const nsITextInputProcessorNotification, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnNotify)(self, aTextInputProcessor, aNotification, _retval)
    }


}


