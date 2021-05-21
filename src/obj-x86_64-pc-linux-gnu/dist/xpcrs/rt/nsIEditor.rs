//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditor.idl
//


/// `interface nsIEditor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEditor {
    vtable: *const nsIEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEditor.
unsafe impl XpCom for nsIEditor {
    const IID: nsIID = nsID(0x094be624, 0xf0bf, 0x400f,
        [0x89, 0xe2, 0x6a, 0x84, 0xba, 0xab, 0x94, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEditor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEditor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEditorCoerce {
    /// Cheaply cast a value of this type from a `nsIEditor`.
    fn coerce_from(v: &nsIEditor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEditorCoerce for nsIEditor {
    #[inline]
    fn coerce_from(v: &nsIEditor) -> &Self {
        v
    }
}

impl nsIEditor {
    /// Cast this `nsIEditor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEditor {
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
impl<T: nsISupportsCoerce> nsIEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEditor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEditorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Selection selection; */
    pub GetSelection: unsafe extern "system" fn (this: *const nsIEditor, aSelection: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void setAttributeOrEquivalent (in Element element, in AString sourceAttrName, in AString sourceAttrValue, in boolean aSuppressTransaction); */
    pub SetAttributeOrEquivalent: unsafe extern "system" fn (this: *const nsIEditor, element: *const libc::c_void, sourceAttrName: *const ::nsstring::nsAString, sourceAttrValue: *const ::nsstring::nsAString, aSuppressTransaction: bool) -> ::nserror::nsresult,

    /* [can_run_script] void removeAttributeOrEquivalent (in Element element, in AString sourceAttrName, in boolean aSuppressTransaction); */
    pub RemoveAttributeOrEquivalent: unsafe extern "system" fn (this: *const nsIEditor, element: *const libc::c_void, sourceAttrName: *const ::nsstring::nsAString, aSuppressTransaction: bool) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute unsigned long flags; */
    pub GetFlags: unsafe extern "system" fn (this: *const nsIEditor, aFlags: *mut u32) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute unsigned long flags; */
    pub SetFlags: unsafe extern "system" fn (this: *const nsIEditor, aFlags: u32) -> ::nserror::nsresult,

    /* attribute AString contentsMIMEType; */
    pub GetContentsMIMEType: unsafe extern "system" fn (this: *const nsIEditor, aContentsMIMEType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString contentsMIMEType; */
    pub SetContentsMIMEType: unsafe extern "system" fn (this: *const nsIEditor, aContentsMIMEType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean isDocumentEditable; */
    pub GetIsDocumentEditable: unsafe extern "system" fn (this: *const nsIEditor, aIsDocumentEditable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isSelectionEditable; */
    pub GetIsSelectionEditable: unsafe extern "system" fn (this: *const nsIEditor, aIsSelectionEditable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute Document document; */
    pub GetDocument: unsafe extern "system" fn (this: *const nsIEditor, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute Element rootElement; */
    pub GetRootElement: unsafe extern "system" fn (this: *const nsIEditor, aRootElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsISelectionController selectionController; */
    pub GetSelectionController: unsafe extern "system" fn (this: *const nsIEditor, aSelectionController: *mut*const nsISelectionController) -> ::nserror::nsresult,

    /* [can_run_script] void deleteSelection (in short action, in short stripWrappers); */
    pub DeleteSelection: unsafe extern "system" fn (this: *const nsIEditor, action: i16, stripWrappers: i16) -> ::nserror::nsresult,

    /* readonly attribute boolean documentIsEmpty; */
    pub GetDocumentIsEmpty: unsafe extern "system" fn (this: *const nsIEditor, aDocumentIsEmpty: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean documentModified; */
    pub GetDocumentModified: unsafe extern "system" fn (this: *const nsIEditor, aDocumentModified: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] attribute ACString documentCharacterSet; */
    pub GetDocumentCharacterSet: unsafe extern "system" fn (this: *const nsIEditor, aDocumentCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [can_run_script] attribute ACString documentCharacterSet; */
    pub SetDocumentCharacterSet: unsafe extern "system" fn (this: *const nsIEditor, aDocumentCharacterSet: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [can_run_script] void resetModificationCount (); */
    pub ResetModificationCount: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* long getModificationCount (); */
    pub GetModificationCount: unsafe extern "system" fn (this: *const nsIEditor, _retval: *mut i32) -> ::nserror::nsresult,

    /* [can_run_script] void incrementModificationCount (in long aModCount); */
    pub IncrementModificationCount: unsafe extern "system" fn (this: *const nsIEditor, aModCount: i32) -> ::nserror::nsresult,

    /* readonly attribute nsITransactionManager transactionManager; */
    pub GetTransactionManager: unsafe extern "system" fn (this: *const nsIEditor, aTransactionManager: *mut*const nsITransactionManager) -> ::nserror::nsresult,

    /* [can_run_script] void doTransaction (in nsITransaction txn); */
    pub DoTransaction: unsafe extern "system" fn (this: *const nsIEditor, txn: *const nsITransaction) -> ::nserror::nsresult,

    /* void enableUndo (in boolean enable); */
    pub EnableUndo: unsafe extern "system" fn (this: *const nsIEditor, enable: bool) -> ::nserror::nsresult,

    /* [can_run_script] void undo (in unsigned long count); */
    pub Undo: unsafe extern "system" fn (this: *const nsIEditor, count: u32) -> ::nserror::nsresult,

    /* void canUndo (out boolean isEnabled, out boolean canUndo); */
    pub CanUndo: unsafe extern "system" fn (this: *const nsIEditor, isEnabled: *mut bool, canUndo: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void redo (in unsigned long count); */
    pub Redo: unsafe extern "system" fn (this: *const nsIEditor, count: u32) -> ::nserror::nsresult,

    /* void canRedo (out boolean isEnabled, out boolean canRedo); */
    pub CanRedo: unsafe extern "system" fn (this: *const nsIEditor, isEnabled: *mut bool, canRedo: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void beginTransaction (); */
    pub BeginTransaction: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* [can_run_script] void endTransaction (); */
    pub EndTransaction: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* void setShouldTxnSetSelection (in boolean should); */
    pub SetShouldTxnSetSelection: unsafe extern "system" fn (this: *const nsIEditor, should: bool) -> ::nserror::nsresult,

    /* nsIInlineSpellChecker getInlineSpellChecker (in boolean autoCreate); */
    pub GetInlineSpellChecker: unsafe extern "system" fn (this: *const nsIEditor, autoCreate: bool, _retval: *mut*const nsIInlineSpellChecker) -> ::nserror::nsresult,

    /* void setSpellcheckUserOverride (in boolean enable); */
    pub SetSpellcheckUserOverride: unsafe extern "system" fn (this: *const nsIEditor, enable: bool) -> ::nserror::nsresult,

    /* [can_run_script] void cut (); */
    pub Cut: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* boolean canCut (); */
    pub CanCut: unsafe extern "system" fn (this: *const nsIEditor, _retval: *mut bool) -> ::nserror::nsresult,

    /* void copy (); */
    pub Copy: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* boolean canCopy (); */
    pub CanCopy: unsafe extern "system" fn (this: *const nsIEditor, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void paste (in long aClipboardType); */
    pub Paste: unsafe extern "system" fn (this: *const nsIEditor, aClipboardType: i32) -> ::nserror::nsresult,

    /* [can_run_script] void pasteTransferable (in nsITransferable aTransferable); */
    pub PasteTransferable: unsafe extern "system" fn (this: *const nsIEditor, aTransferable: *const nsITransferable) -> ::nserror::nsresult,

    /* boolean canPaste (in long aClipboardType); */
    pub CanPaste: unsafe extern "system" fn (this: *const nsIEditor, aClipboardType: i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void selectAll (); */
    pub SelectAll: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* void beginningOfDocument (); */
    pub BeginningOfDocument: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* void endOfDocument (); */
    pub EndOfDocument: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* [can_run_script] void setAttribute (in Element aElement, in AString attributestr, in AString attvalue); */
    pub SetAttribute: unsafe extern "system" fn (this: *const nsIEditor, aElement: *const libc::c_void, attributestr: *const ::nsstring::nsAString, attvalue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void removeAttribute (in Element aElement, in AString aAttribute); */
    pub RemoveAttribute: unsafe extern "system" fn (this: *const nsIEditor, aElement: *const libc::c_void, aAttribute: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void cloneAttributes (in Element aDestElement, in Element aSourceElement); */
    pub CloneAttributes: unsafe extern "system" fn (this: *const nsIEditor, aDestElement: *const libc::c_void, aSourceElement: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void insertNode (in Node node, in Node parent, in long aPosition); */
    pub InsertNode: unsafe extern "system" fn (this: *const nsIEditor, node: *const libc::c_void, parent: *const libc::c_void, aPosition: i32) -> ::nserror::nsresult,

    /* [can_run_script] void deleteNode (in Node child); */
    pub DeleteNode: unsafe extern "system" fn (this: *const nsIEditor, child: *const libc::c_void) -> ::nserror::nsresult,

    /* AString outputToString (in AString formatType, in unsigned long flags); */
    pub OutputToString: unsafe extern "system" fn (this: *const nsIEditor, formatType: *const ::nsstring::nsAString, flags: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addEditorObserver (in nsIEditorObserver observer); */
    pub AddEditorObserver: unsafe extern "system" fn (this: *const nsIEditor, observer: *const nsIEditorObserver) -> ::nserror::nsresult,

    /* void addEditActionListener (in nsIEditActionListener listener); */
    pub AddEditActionListener: unsafe extern "system" fn (this: *const nsIEditor, listener: *const nsIEditActionListener) -> ::nserror::nsresult,

    /* void removeEditActionListener (in nsIEditActionListener listener); */
    pub RemoveEditActionListener: unsafe extern "system" fn (this: *const nsIEditor, listener: *const nsIEditActionListener) -> ::nserror::nsresult,

    /* void addDocumentStateListener (in nsIDocumentStateListener listener); */
    pub AddDocumentStateListener: unsafe extern "system" fn (this: *const nsIEditor, listener: *const nsIDocumentStateListener) -> ::nserror::nsresult,

    /* void removeDocumentStateListener (in nsIDocumentStateListener listener); */
    pub RemoveDocumentStateListener: unsafe extern "system" fn (this: *const nsIEditor, listener: *const nsIDocumentStateListener) -> ::nserror::nsresult,

    /* void forceCompositionEnd (); */
    pub ForceCompositionEnd: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* readonly attribute boolean composing; */
    pub GetComposing: unsafe extern "system" fn (this: *const nsIEditor, aComposing: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script,optional_argc] void unmask ([optional] in unsigned long aStart, [optional] in long long aEnd, [optional] in unsigned long aTimeout); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub Unmask: *const ::libc::c_void,

    /* [can_run_script] void mask (); */
    pub Mask: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,

    /* readonly attribute unsigned long unmaskedStart; */
    pub GetUnmaskedStart: unsafe extern "system" fn (this: *const nsIEditor, aUnmaskedStart: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long unmaskedEnd; */
    pub GetUnmaskedEnd: unsafe extern "system" fn (this: *const nsIEditor, aUnmaskedEnd: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean autoMaskingEnabled; */
    pub GetAutoMaskingEnabled: unsafe extern "system" fn (this: *const nsIEditor, aAutoMaskingEnabled: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString passwordMask; */
    pub GetPasswordMask: unsafe extern "system" fn (this: *const nsIEditor, aPasswordMask: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute long textLength; */
    pub GetTextLength: unsafe extern "system" fn (this: *const nsIEditor, aTextLength: *mut i32) -> ::nserror::nsresult,

    /* attribute long wrapWidth; */
    pub GetWrapWidth: unsafe extern "system" fn (this: *const nsIEditor, aWrapWidth: *mut i32) -> ::nserror::nsresult,

    /* attribute long wrapWidth; */
    pub SetWrapWidth: unsafe extern "system" fn (this: *const nsIEditor, aWrapWidth: i32) -> ::nserror::nsresult,

    /* attribute long newlineHandling; */
    pub GetNewlineHandling: unsafe extern "system" fn (this: *const nsIEditor, aNewlineHandling: *mut i32) -> ::nserror::nsresult,

    /* attribute long newlineHandling; */
    pub SetNewlineHandling: unsafe extern "system" fn (this: *const nsIEditor, aNewlineHandling: i32) -> ::nserror::nsresult,

    /* [can_run_script] void insertText (in AString aStringToInsert); */
    pub InsertText: unsafe extern "system" fn (this: *const nsIEditor, aStringToInsert: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void insertLineBreak (); */
    pub InsertLineBreak: unsafe extern "system" fn (this: *const nsIEditor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEditor {

    pub const eNone: i64 = 0;


    pub const eNext: i64 = 1;


    pub const ePrevious: i64 = 2;


    pub const eNextWord: i64 = 3;


    pub const ePreviousWord: i64 = 4;


    pub const eToBeginningOfLine: i64 = 5;


    pub const eToEndOfLine: i64 = 6;


    pub const eStrip: i64 = 0;


    pub const eNoStrip: i64 = 1;


    pub const eEditorPlaintextMask: i64 = 1;


    pub const eEditorSingleLineMask: i64 = 2;


    pub const eEditorPasswordMask: i64 = 4;


    pub const eEditorReadonlyMask: i64 = 8;


    pub const eEditorFilterInputMask: i64 = 16;


    pub const eEditorMailMask: i64 = 32;


    pub const eEditorEnableWrapHackMask: i64 = 64;


    pub const eEditorWidgetMask: i64 = 128;


    pub const eEditorNoCSSMask: i64 = 256;


    pub const eEditorAllowInteraction: i64 = 512;


    pub const eEditorDontEchoPassword: i64 = 1024;


    pub const eEditorRightToLeft: i64 = 2048;


    pub const eEditorLeftToRight: i64 = 4096;


    pub const eEditorSkipSpellCheck: i64 = 8192;


    pub const eNewlinesPasteIntact: i64 = 0;


    pub const eNewlinesPasteToFirst: i64 = 1;


    pub const eNewlinesReplaceWithSpaces: i64 = 2;


    pub const eNewlinesStrip: i64 = 3;


    pub const eNewlinesReplaceWithCommas: i64 = 4;


    pub const eNewlinesStripSurroundingWhitespace: i64 = 5;


    /// `readonly attribute Selection selection;`
    #[inline]
    pub unsafe fn GetSelection(&self, aSelection: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSelection)(self, aSelection)
    }



    /// `[can_run_script] void setAttributeOrEquivalent (in Element element, in AString sourceAttrName, in AString sourceAttrValue, in boolean aSuppressTransaction);`
    #[inline]
    pub unsafe fn SetAttributeOrEquivalent(&self, element: *const libc::c_void, sourceAttrName: *const ::nsstring::nsAString, sourceAttrValue: *const ::nsstring::nsAString, aSuppressTransaction: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAttributeOrEquivalent)(self, element, sourceAttrName, sourceAttrValue, aSuppressTransaction)
    }



    /// `[can_run_script] void removeAttributeOrEquivalent (in Element element, in AString sourceAttrName, in boolean aSuppressTransaction);`
    #[inline]
    pub unsafe fn RemoveAttributeOrEquivalent(&self, element: *const libc::c_void, sourceAttrName: *const ::nsstring::nsAString, aSuppressTransaction: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAttributeOrEquivalent)(self, element, sourceAttrName, aSuppressTransaction)
    }


    /// ```text
    /// /** edit flags for this editor.  May be set at any time. */
    /// ```
    ///

    /// `[setter_can_run_script] attribute unsigned long flags;`
    #[inline]
    pub unsafe fn GetFlags(&self, aFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFlags)(self, aFlags)
    }


    /// ```text
    /// /** edit flags for this editor.  May be set at any time. */
    /// ```
    ///

    /// `[setter_can_run_script] attribute unsigned long flags;`
    #[inline]
    pub unsafe fn SetFlags(&self, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetFlags)(self, aFlags)
    }


    /// ```text
    /// /**
    ///    * the MimeType of the document
    ///    */
    /// ```
    ///

    /// `attribute AString contentsMIMEType;`
    #[inline]
    pub unsafe fn GetContentsMIMEType(&self, aContentsMIMEType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetContentsMIMEType)(self, aContentsMIMEType)
    }


    /// ```text
    /// /**
    ///    * the MimeType of the document
    ///    */
    /// ```
    ///

    /// `attribute AString contentsMIMEType;`
    #[inline]
    pub unsafe fn SetContentsMIMEType(&self, aContentsMIMEType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetContentsMIMEType)(self, aContentsMIMEType)
    }


    /// ```text
    /// /** Returns true if we have a document that is not marked read-only */
    /// ```
    ///

    /// `readonly attribute boolean isDocumentEditable;`
    #[inline]
    pub unsafe fn GetIsDocumentEditable(&self, aIsDocumentEditable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDocumentEditable)(self, aIsDocumentEditable)
    }


    /// ```text
    /// /** Returns true if the current selection anchor is editable */
    /// ```
    ///

    /// `readonly attribute boolean isSelectionEditable;`
    #[inline]
    pub unsafe fn GetIsSelectionEditable(&self, aIsSelectionEditable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSelectionEditable)(self, aIsSelectionEditable)
    }


    /// ```text
    /// /**
    ///    * the DOM Document this editor is associated with, refcounted.
    ///    */
    /// ```
    ///

    /// `readonly attribute Document document;`
    #[inline]
    pub unsafe fn GetDocument(&self, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDocument)(self, aDocument)
    }


    /// ```text
    /// /** the body element, i.e. the root of the editable document.
    ///    */
    /// ```
    ///

    /// `readonly attribute Element rootElement;`
    #[inline]
    pub unsafe fn GetRootElement(&self, aRootElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetRootElement)(self, aRootElement)
    }


    /// ```text
    /// /**
    ///    * the selection controller for the current presentation, refcounted.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISelectionController selectionController;`
    #[inline]
    pub unsafe fn GetSelectionController(&self, aSelectionController: *mut*const nsISelectionController) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionController)(self, aSelectionController)
    }


    /// ```text
    /// /**
    ///    * DeleteSelection removes all nodes in the current selection.
    ///    * @param aDir  if eNext, delete to the right (for example, the DEL key)
    ///    *              if ePrevious, delete to the left (for example, the BACKSPACE key)
    ///    * @param stripWrappers If eStrip, strip any empty inline elements left
    ///    *                      behind after the deletion; if eNoStrip, don't.  If in
    ///    *                      doubt, pass eStrip -- eNoStrip is only for if you're
    ///    *                      about to insert text or similar right after.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteSelection (in short action, in short stripWrappers);`
    #[inline]
    pub unsafe fn DeleteSelection(&self, action: i16, stripWrappers: i16) -> ::nserror::nsresult {
        ((*self.vtable).DeleteSelection)(self, action, stripWrappers)
    }


    /// ```text
    /// /** Returns true if the document has no *meaningful* content */
    /// ```
    ///

    /// `readonly attribute boolean documentIsEmpty;`
    #[inline]
    pub unsafe fn GetDocumentIsEmpty(&self, aDocumentIsEmpty: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentIsEmpty)(self, aDocumentIsEmpty)
    }


    /// ```text
    /// /** Returns true if the document is modifed and needs saving */
    /// ```
    ///

    /// `readonly attribute boolean documentModified;`
    #[inline]
    pub unsafe fn GetDocumentModified(&self, aDocumentModified: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentModified)(self, aDocumentModified)
    }


    /// ```text
    /// /** Sets the current 'Save' document character set */
    /// ```
    ///

    /// `[can_run_script] attribute ACString documentCharacterSet;`
    #[inline]
    pub unsafe fn GetDocumentCharacterSet(&self, aDocumentCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocumentCharacterSet)(self, aDocumentCharacterSet)
    }


    /// ```text
    /// /** Sets the current 'Save' document character set */
    /// ```
    ///

    /// `[can_run_script] attribute ACString documentCharacterSet;`
    #[inline]
    pub unsafe fn SetDocumentCharacterSet(&self, aDocumentCharacterSet: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDocumentCharacterSet)(self, aDocumentCharacterSet)
    }


    /// ```text
    /// /** to be used ONLY when we need to override the doc's modification
    ///     * state (such as when it's saved).
    ///     */
    /// ```
    ///

    /// `[can_run_script] void resetModificationCount ();`
    #[inline]
    pub unsafe fn ResetModificationCount(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResetModificationCount)(self, )
    }


    /// ```text
    /// /** Gets the modification count of the document we are editing.
    ///     * @return the modification count of the document being edited.
    ///     *         Zero means unchanged.
    ///     */
    /// ```
    ///

    /// `long getModificationCount ();`
    #[inline]
    pub unsafe fn GetModificationCount(&self, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetModificationCount)(self, _retval)
    }


    /// ```text
    /// /** called each time we modify the document.
    ///     * Increments the modification count of the document.
    ///     * @param  aModCount  the number of modifications by which
    ///     *                    to increase or decrease the count
    ///     */
    /// ```
    ///

    /// `[can_run_script] void incrementModificationCount (in long aModCount);`
    #[inline]
    pub unsafe fn IncrementModificationCount(&self, aModCount: i32) -> ::nserror::nsresult {
        ((*self.vtable).IncrementModificationCount)(self, aModCount)
    }


    /// ```text
    /// /** transactionManager Get the transaction manager the editor is using.
    ///     */
    /// ```
    ///

    /// `readonly attribute nsITransactionManager transactionManager;`
    #[inline]
    pub unsafe fn GetTransactionManager(&self, aTransactionManager: *mut*const nsITransactionManager) -> ::nserror::nsresult {
        ((*self.vtable).GetTransactionManager)(self, aTransactionManager)
    }


    /// ```text
    /// /** doTransaction() fires a transaction.
    ///     * It is provided here so clients can create their own transactions.
    ///     * If a transaction manager is present, it is used.
    ///     * Otherwise, the transaction is just executed directly.
    ///     *
    ///     * @param aTxn the transaction to execute
    ///     */
    /// ```
    ///

    /// `[can_run_script] void doTransaction (in nsITransaction txn);`
    #[inline]
    pub unsafe fn DoTransaction(&self, txn: *const nsITransaction) -> ::nserror::nsresult {
        ((*self.vtable).DoTransaction)(self, txn)
    }


    /// ```text
    /// /** turn the undo system on or off
    ///     * @param aEnable  if PR_TRUE, the undo system is turned on if available
    ///     *                 if PR_FALSE the undo system is turned off if it
    ///     *                 was previously on
    ///     * @return         if aEnable is PR_TRUE, returns NS_OK if
    ///     *                 the undo system could be initialized properly
    ///     *                 if aEnable is PR_FALSE, returns NS_OK.
    ///     */
    /// ```
    ///

    /// `void enableUndo (in boolean enable);`
    #[inline]
    pub unsafe fn EnableUndo(&self, enable: bool) -> ::nserror::nsresult {
        ((*self.vtable).EnableUndo)(self, enable)
    }


    /// ```text
    /// /** undo reverses the effects of the last Do operation,
    ///     * if Undo is enabled in the editor.
    ///     * It is provided here so clients need no knowledge of whether
    ///     * the editor has a transaction manager or not.
    ///     * If a transaction manager is present, it is told to undo,
    ///     * and the result of that undo is returned.
    ///     * Otherwise, the Undo request is ignored and an
    ///     * error NS_ERROR_NOT_AVAILABLE is returned.
    ///     *
    ///     */
    /// ```
    ///

    /// `[can_run_script] void undo (in unsigned long count);`
    #[inline]
    pub unsafe fn Undo(&self, count: u32) -> ::nserror::nsresult {
        ((*self.vtable).Undo)(self, count)
    }


    /// ```text
    /// /** returns state information about the undo system.
    ///     * @param aIsEnabled [OUT] PR_TRUE if undo is enabled
    ///     * @param aCanUndo   [OUT] PR_TRUE if at least one transaction is
    ///     *                         currently ready to be undone.
    ///     */
    /// ```
    ///

    /// `void canUndo (out boolean isEnabled, out boolean canUndo);`
    #[inline]
    pub unsafe fn CanUndo(&self, isEnabled: *mut bool, canUndo: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanUndo)(self, isEnabled, canUndo)
    }


    /// ```text
    /// /** redo reverses the effects of the last Undo operation
    ///     * It is provided here so clients need no knowledge of whether
    ///     * the editor has a transaction manager or not.
    ///     * If a transaction manager is present, it is told to redo and the
    ///     * result of the previously undone transaction is reapplied to the document.
    ///     * If no transaction is available for Redo, or if the document
    ///     * has no transaction manager, the Redo request is ignored and an
    ///     * error NS_ERROR_NOT_AVAILABLE is returned.
    ///     *
    ///     */
    /// ```
    ///

    /// `[can_run_script] void redo (in unsigned long count);`
    #[inline]
    pub unsafe fn Redo(&self, count: u32) -> ::nserror::nsresult {
        ((*self.vtable).Redo)(self, count)
    }


    /// ```text
    /// /** returns state information about the redo system.
    ///     * @param aIsEnabled [OUT] PR_TRUE if redo is enabled
    ///     * @param aCanRedo   [OUT] PR_TRUE if at least one transaction is
    ///                               currently ready to be redone.
    ///     */
    /// ```
    ///

    /// `void canRedo (out boolean isEnabled, out boolean canRedo);`
    #[inline]
    pub unsafe fn CanRedo(&self, isEnabled: *mut bool, canRedo: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanRedo)(self, isEnabled, canRedo)
    }


    /// ```text
    /// /** beginTransaction is a signal from the caller to the editor that
    ///     * the caller will execute multiple updates to the content tree
    ///     * that should be treated as a single logical operation,
    ///     * in the most efficient way possible.<br>
    ///     * All transactions executed between a call to beginTransaction and
    ///     * endTransaction will be undoable as an atomic action.<br>
    ///     * endTransaction must be called after beginTransaction.<br>
    ///     * Calls to beginTransaction can be nested, as long as endTransaction
    ///     * is called once per beginUpdate.
    ///     */
    /// ```
    ///

    /// `[can_run_script] void beginTransaction ();`
    #[inline]
    pub unsafe fn BeginTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginTransaction)(self, )
    }


    /// ```text
    /// /** endTransaction is a signal to the editor that the caller is
    ///     * finished updating the content model.<br>
    ///     * beginUpdate must be called before endTransaction is called.<br>
    ///     * Calls to beginTransaction can be nested, as long as endTransaction
    ///     * is called once per beginTransaction.
    ///     */
    /// ```
    ///

    /// `[can_run_script] void endTransaction ();`
    #[inline]
    pub unsafe fn EndTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * While setting the flag with this method to false, CreateElementTransaction,
    ///    * DeleteRangeTransaction, DeleteTextTransaction, InsertNodeTransaction,
    ///    * InsertTextTransaction and SplitNodeTransaction won't change Selection
    ///    * after modifying the DOM tree.
    ///    * Note that calling this with false does not guarantee that Selection won't
    ///    * be changed because other transaction may ignore this flag, editor itself
    ///    * may change selection, and current selection may become invalid after
    ///    * changing the DOM tree, etc.
    ///    * After calling this method with true, the caller should guarantee that
    ///    * Selection should be positioned where user expects.
    ///    *
    ///    * @param should    false if you don't want above transactions to modify
    ///    *                  Selection automatically after modifying the DOM tree.
    ///    *                  Note that calling this with false does not guarantee
    ///    *                  that Selection is never changed.
    ///    */
    /// ```
    ///

    /// `void setShouldTxnSetSelection (in boolean should);`
    #[inline]
    pub unsafe fn SetShouldTxnSetSelection(&self, should: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShouldTxnSetSelection)(self, should)
    }


    /// ```text
    /// /** Returns the inline spell checker associated with this object. The spell
    ///     * checker is lazily created, so this function may create the object for
    ///     * you during this call.
    ///     * @param  autoCreate  If true, this will create a spell checker object
    ///     *                     if one does not exist yet for this editor. If false
    ///     *                     and the object has not been created, this function
    ///     *                     WILL RETURN NULL.
    ///     */
    /// ```
    ///

    /// `nsIInlineSpellChecker getInlineSpellChecker (in boolean autoCreate);`
    #[inline]
    pub unsafe fn GetInlineSpellChecker(&self, autoCreate: bool, _retval: *mut*const nsIInlineSpellChecker) -> ::nserror::nsresult {
        ((*self.vtable).GetInlineSpellChecker)(self, autoCreate, _retval)
    }


    /// ```text
    /// /** Called when the user manually overrides the spellchecking state for this
    ///     * editor.
    ///     * @param  enable  The new state of spellchecking in this editor, as
    ///     *                 requested by the user.
    ///     */
    /// ```
    ///

    /// `void setSpellcheckUserOverride (in boolean enable);`
    #[inline]
    pub unsafe fn SetSpellcheckUserOverride(&self, enable: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSpellcheckUserOverride)(self, enable)
    }


    /// ```text
    /// /** cut the currently selected text, putting it into the OS clipboard
    ///     * What if no text is selected?
    ///     * What about mixed selections?
    ///     * What are the clipboard formats?
    ///     */
    /// ```
    ///

    /// `[can_run_script] void cut ();`
    #[inline]
    pub unsafe fn Cut(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Cut)(self, )
    }


    /// ```text
    /// /**
    ///    * canCut() returns true if selected content is allowed to be copied to the
    ///    * clipboard and to be removed.
    ///    * Note that this always returns true if the editor is in a non-chrome
    ///    * HTML/XHTML document.
    ///    * FYI: Current user in script is only BlueGriffon.
    ///    */
    /// ```
    ///

    /// `boolean canCut ();`
    #[inline]
    pub unsafe fn CanCut(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanCut)(self, _retval)
    }


    /// ```text
    /// /** copy the currently selected text, putting it into the OS clipboard
    ///     * What if no text is selected?
    ///     * What about mixed selections?
    ///     * What are the clipboard formats?
    ///     */
    /// ```
    ///

    /// `void copy ();`
    #[inline]
    pub unsafe fn Copy(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Copy)(self, )
    }


    /// ```text
    /// /**
    ///    * canCopy() returns true if selected content is allowed to be copied to
    ///    * the clipboard.
    ///    * Note that this always returns true if the editor is in a non-chrome
    ///    * HTML/XHTML document.
    ///    * FYI: Current user in script is only BlueGriffon.
    ///    */
    /// ```
    ///

    /// `boolean canCopy ();`
    #[inline]
    pub unsafe fn CanCopy(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanCopy)(self, _retval)
    }


    /// ```text
    /// /** paste the text in the OS clipboard at the cursor position, replacing
    ///     * the selected text (if any)
    ///     */
    /// ```
    ///

    /// `[can_run_script] void paste (in long aClipboardType);`
    #[inline]
    pub unsafe fn Paste(&self, aClipboardType: i32) -> ::nserror::nsresult {
        ((*self.vtable).Paste)(self, aClipboardType)
    }


    /// ```text
    /// /** Paste the text in |aTransferable| at the cursor position, replacing the
    ///     * selected text (if any).
    ///     */
    /// ```
    ///

    /// `[can_run_script] void pasteTransferable (in nsITransferable aTransferable);`
    #[inline]
    pub unsafe fn PasteTransferable(&self, aTransferable: *const nsITransferable) -> ::nserror::nsresult {
        ((*self.vtable).PasteTransferable)(self, aTransferable)
    }


    /// ```text
    /// /** Can we paste? True if the doc is modifiable, and we have
    ///     * pasteable data in the clipboard.
    ///     */
    /// ```
    ///

    /// `boolean canPaste (in long aClipboardType);`
    #[inline]
    pub unsafe fn CanPaste(&self, aClipboardType: i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanPaste)(self, aClipboardType, _retval)
    }


    /// ```text
    /// /** sets the document selection to the entire contents of the document */
    /// ```
    ///

    /// `[can_run_script] void selectAll ();`
    #[inline]
    pub unsafe fn SelectAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SelectAll)(self, )
    }


    /// ```text
    /// /**
    ///    * Collapses selection at start of the document.  If it's an HTML editor,
    ///    * collapses selection at start of current editing host (<body> element if
        ///    * it's in designMode) instead.  If there is a non-editable node before any
    ///    * editable text nodes or inline elements which can have text nodes as their
    ///    * children, collapses selection at start of the editing host.  If there is
    ///    * an editable text node which is not collapsed, collapses selection at
    ///    * start of the text node.  If there is an editable inline element which
    ///    * cannot have text nodes as its child, collapses selection at before the
    ///    * element node.  Otherwise, collapses selection at start of the editing
    ///    * host.
    ///    */
    /// ```
    ///

    /// `void beginningOfDocument ();`
    #[inline]
    pub unsafe fn BeginningOfDocument(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginningOfDocument)(self, )
    }


    /// ```text
    /// /** sets the document selection to the end of the document */
    /// ```
    ///

    /// `void endOfDocument ();`
    #[inline]
    pub unsafe fn EndOfDocument(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndOfDocument)(self, )
    }


    /// ```text
    /// /**
    ///    * setAttribute() sets the attribute of aElement.
    ///    * No checking is done to see if aAttribute is a legal attribute of the node,
    ///    * or if aValue is a legal value of aAttribute.
    ///    *
    ///    * @param aElement    the content element to operate on
    ///    * @param aAttribute  the string representation of the attribute to set
    ///    * @param aValue      the value to set aAttribute to
    ///    */
    /// ```
    ///

    /// `[can_run_script] void setAttribute (in Element aElement, in AString attributestr, in AString attvalue);`
    #[inline]
    pub unsafe fn SetAttribute(&self, aElement: *const libc::c_void, attributestr: *const ::nsstring::nsAString, attvalue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetAttribute)(self, aElement, attributestr, attvalue)
    }


    /// ```text
    /// /**
    ///    * removeAttribute() deletes aAttribute from the attribute list of aElement.
    ///    * If aAttribute is not an attribute of aElement, nothing is done.
    ///    *
    ///    * @param aElement      the content element to operate on
    ///    * @param aAttribute    the string representation of the attribute to get
    ///    */
    /// ```
    ///

    /// `[can_run_script] void removeAttribute (in Element aElement, in AString aAttribute);`
    #[inline]
    pub unsafe fn RemoveAttribute(&self, aElement: *const libc::c_void, aAttribute: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAttribute)(self, aElement, aAttribute)
    }


    /// ```text
    /// /**
    ///    * cloneAttributes() is similar to Node::cloneNode(),
    ///    *   it assures the attribute nodes of the destination are identical
    ///    *   with the source node by copying all existing attributes from the
    ///    *   source and deleting those not in the source.
    ///    *   This is used when the destination element already exists
    ///    *
    ///    * @param aDestNode     the destination element to operate on
    ///    * @param aSourceNode   the source element to copy attributes from
    ///    */
    /// ```
    ///

    /// `[can_run_script] void cloneAttributes (in Element aDestElement, in Element aSourceElement);`
    #[inline]
    pub unsafe fn CloneAttributes(&self, aDestElement: *const libc::c_void, aSourceElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CloneAttributes)(self, aDestElement, aSourceElement)
    }


    /// ```text
    /// /**
    ///    * insertNode inserts aNode into aParent at aPosition.
    ///    * No checking is done to verify the legality of the insertion.
    ///    * That is the responsibility of the caller.
    ///    * @param aNode     The DOM Node to insert.
    ///    * @param aParent   The node to insert the new object into
    ///    * @param aPosition The place in aParent to insert the new node
    ///    *                  0=first child, 1=second child, etc.
    ///    *                  any number > number of current children = last child
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertNode (in Node node, in Node parent, in long aPosition);`
    #[inline]
    pub unsafe fn InsertNode(&self, node: *const libc::c_void, parent: *const libc::c_void, aPosition: i32) -> ::nserror::nsresult {
        ((*self.vtable).InsertNode)(self, node, parent, aPosition)
    }


    /// ```text
    /// /**
    ///    * deleteNode removes aChild from aParent.
    ///    * @param aChild    The node to delete
    ///    */
    /// ```
    ///

    /// `[can_run_script] void deleteNode (in Node child);`
    #[inline]
    pub unsafe fn DeleteNode(&self, child: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).DeleteNode)(self, child)
    }


    /// ```text
    /// /**
    ///    * Output methods:
    ///    * aFormatType is a mime type, like text/plain.
    ///    */
    /// ```
    ///

    /// `AString outputToString (in AString formatType, in unsigned long flags);`
    #[inline]
    pub unsafe fn OutputToString(&self, formatType: *const ::nsstring::nsAString, flags: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OutputToString)(self, formatType, flags, _retval)
    }


    /// ```text
    /// /** add an EditorObserver to the editors list of observers. */
    /// ```
    ///

    /// `void addEditorObserver (in nsIEditorObserver observer);`
    #[inline]
    pub unsafe fn AddEditorObserver(&self, observer: *const nsIEditorObserver) -> ::nserror::nsresult {
        ((*self.vtable).AddEditorObserver)(self, observer)
    }


    /// ```text
    /// /** add an EditActionListener to the editors list of listeners. */
    /// ```
    ///

    /// `void addEditActionListener (in nsIEditActionListener listener);`
    #[inline]
    pub unsafe fn AddEditActionListener(&self, listener: *const nsIEditActionListener) -> ::nserror::nsresult {
        ((*self.vtable).AddEditActionListener)(self, listener)
    }


    /// ```text
    /// /** Remove an EditActionListener from the editor's list of listeners. */
    /// ```
    ///

    /// `void removeEditActionListener (in nsIEditActionListener listener);`
    #[inline]
    pub unsafe fn RemoveEditActionListener(&self, listener: *const nsIEditActionListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveEditActionListener)(self, listener)
    }


    /// ```text
    /// /** Add a DocumentStateListener to the editors list of doc state listeners. */
    /// ```
    ///

    /// `void addDocumentStateListener (in nsIDocumentStateListener listener);`
    #[inline]
    pub unsafe fn AddDocumentStateListener(&self, listener: *const nsIDocumentStateListener) -> ::nserror::nsresult {
        ((*self.vtable).AddDocumentStateListener)(self, listener)
    }


    /// ```text
    /// /** Remove a DocumentStateListener to the editors list of doc state listeners. */
    /// ```
    ///

    /// `void removeDocumentStateListener (in nsIDocumentStateListener listener);`
    #[inline]
    pub unsafe fn RemoveDocumentStateListener(&self, listener: *const nsIDocumentStateListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDocumentStateListener)(self, listener)
    }


    /// ```text
    /// /**
    ///    * forceCompositionEnd() force the composition end
    ///    */
    /// ```
    ///

    /// `void forceCompositionEnd ();`
    #[inline]
    pub unsafe fn ForceCompositionEnd(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ForceCompositionEnd)(self, )
    }


    /// ```text
    /// /**
    ///    * whether this editor has active IME transaction
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean composing;`
    #[inline]
    pub unsafe fn GetComposing(&self, aComposing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetComposing)(self, aComposing)
    }


    /// ```text
    /// /**
    ///    * unmask() is available only when the editor is a passwrod field.  This
    ///    * unmasks characters in specified by aStart and aEnd.  If there have
    ///    * already unmasked characters, they are masked when this is called.
    ///    * Note that if you calls this without non-zero `aTimeout`, you bear
    ///    * responsibility for masking password with calling `mask()`.  I.e.,
    ///    * user inputting password won't be masked automacitally.  If user types
    ///    * a new character and echo is enabled, unmasked range is expanded to
    ///    * including it.
    ///    *
    ///    * @param aStart      Optional, first index to show the character.  If you
    ///    *                    specify middle of a surrogate pair, this expands the
    ///    *                    range to include the prceding high surrogate
    ///    *                    automatically.
    ///    *                    If omitted, it means that all characters of the
    ///    *                    password becomes unmasked.
    ///    * @param aEnd        Optional, next index of last unmasked character.  If
    ///    *                    you specify middle of a surrogate pair, the expands
    ///    *                    the range to include the following low surrogate.
    ///    *                    If omitted or negative value, it means unmasking all
    ///    *                    characters after aStart.  Specifying same index
    ///    *                    throws an exception.
    ///    * @param aTimeout    Optional, specify milliseconds to hide the unmasked
    ///    *                    characters if you want to show them temporarily.
    ///    *                    If omitted or 0, it means this won't mask the characters
    ///    *                    automatically.
    ///    */
    /// ```
    ///

    /// `[can_run_script,optional_argc] void unmask ([optional] in unsigned long aStart, [optional] in long long aEnd, [optional] in unsigned long aTimeout);`
    const _Unmask: () = ();

    /// ```text
    /// /**
    ///    * mask() is available only when the editor is a password field.  This masks
    ///    * all unmasked characters immediately.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void mask ();`
    #[inline]
    pub unsafe fn Mask(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Mask)(self, )
    }


    /// ```text
    /// /**
    ///    * These attributes are available only when the editor is a password field.
    ///    * unmaskedStart is first unmasked character index, or 0 if there is no
    ///    * unmasked characters.
    ///    * unmaskedEnd is next index of the last unmasked character.  0 means there
    ///    * is no unmasked characters.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long unmaskedStart;`
    #[inline]
    pub unsafe fn GetUnmaskedStart(&self, aUnmaskedStart: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetUnmaskedStart)(self, aUnmaskedStart)
    }



    /// `readonly attribute unsigned long unmaskedEnd;`
    #[inline]
    pub unsafe fn GetUnmaskedEnd(&self, aUnmaskedEnd: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetUnmaskedEnd)(self, aUnmaskedEnd)
    }


    /// ```text
    /// /**
    ///    * autoMaskingEnabled is true if unmasked range and newly inputted characters
    ///    * are masked automatically.  That's the default state.  If false, until
    ///    * `mask()` is called, unmasked range and newly inputted characters are
    ///    * unmasked.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean autoMaskingEnabled;`
    #[inline]
    pub unsafe fn GetAutoMaskingEnabled(&self, aAutoMaskingEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAutoMaskingEnabled)(self, aAutoMaskingEnabled)
    }


    /// ```text
    /// /**
    ///    * passwordMask attribute is a mask character which is used to mask password.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString passwordMask;`
    #[inline]
    pub unsafe fn GetPasswordMask(&self, aPasswordMask: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPasswordMask)(self, aPasswordMask)
    }


    /// ```text
    /// /**
    ///     * The length of the contents in characters.
    ///     * XXX change this type to 'unsigned long'
    ///     */
    /// ```
    ///

    /// `readonly attribute long textLength;`
    #[inline]
    pub unsafe fn GetTextLength(&self, aTextLength: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetTextLength)(self, aTextLength)
    }


    /// ```text
    /// /** Get and set the body wrap width.
    ///     *
    ///     * Special values:
    ///     *    0 = wrap to window width
    ///     *   -1 = no wrap at all
    ///     */
    /// ```
    ///

    /// `attribute long wrapWidth;`
    #[inline]
    pub unsafe fn GetWrapWidth(&self, aWrapWidth: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetWrapWidth)(self, aWrapWidth)
    }


    /// ```text
    /// /** Get and set the body wrap width.
    ///     *
    ///     * Special values:
    ///     *    0 = wrap to window width
    ///     *   -1 = no wrap at all
    ///     */
    /// ```
    ///

    /// `attribute long wrapWidth;`
    #[inline]
    pub unsafe fn SetWrapWidth(&self, aWrapWidth: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetWrapWidth)(self, aWrapWidth)
    }


    /// ```text
    /// /** Get and set newline handling.
    ///    *
    ///    *  Values are the constants defined above.
    ///    */
    /// ```
    ///

    /// `attribute long newlineHandling;`
    #[inline]
    pub unsafe fn GetNewlineHandling(&self, aNewlineHandling: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNewlineHandling)(self, aNewlineHandling)
    }


    /// ```text
    /// /** Get and set newline handling.
    ///    *
    ///    *  Values are the constants defined above.
    ///    */
    /// ```
    ///

    /// `attribute long newlineHandling;`
    #[inline]
    pub unsafe fn SetNewlineHandling(&self, aNewlineHandling: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetNewlineHandling)(self, aNewlineHandling)
    }


    /// ```text
    /// /**
    ///    * Inserts a string at the current location,
    ///    * given by the selection.
    ///    * If the selection is not collapsed, the selection is deleted
    ///    * and the insertion takes place at the resulting collapsed selection.
    ///    *
    ///    * @param aString   the string to be inserted
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertText (in AString aStringToInsert);`
    #[inline]
    pub unsafe fn InsertText(&self, aStringToInsert: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InsertText)(self, aStringToInsert)
    }


    /// ```text
    /// /**
    ///    * Insert a line break into the content model.
    ///    * The interpretation of a break is up to the implementation:
    ///    * it may enter a character, split a node in the tree, etc.
    ///    * This may be more efficient than calling InsertText with a newline.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertLineBreak ();`
    #[inline]
    pub unsafe fn InsertLineBreak(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InsertLineBreak)(self, )
    }


}


