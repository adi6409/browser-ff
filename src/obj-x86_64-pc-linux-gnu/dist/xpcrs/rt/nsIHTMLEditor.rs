//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLEditor.idl
//


/// `interface nsIHTMLEditor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHTMLEditor {
    vtable: *const nsIHTMLEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHTMLEditor.
unsafe impl XpCom for nsIHTMLEditor {
    const IID: nsIID = nsID(0x87ee993e, 0x985f, 0x4a43,
        [0xa9, 0x74, 0x0d, 0x95, 0x12, 0xda, 0x2f, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHTMLEditor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHTMLEditor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHTMLEditorCoerce {
    /// Cheaply cast a value of this type from a `nsIHTMLEditor`.
    fn coerce_from(v: &nsIHTMLEditor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHTMLEditorCoerce for nsIHTMLEditor {
    #[inline]
    fn coerce_from(v: &nsIHTMLEditor) -> &Self {
        v
    }
}

impl nsIHTMLEditor {
    /// Cast this `nsIHTMLEditor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHTMLEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHTMLEditor {
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
impl<T: nsISupportsCoerce> nsIHTMLEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLEditor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHTMLEditor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHTMLEditorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void setInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue); */
    pub SetInlineProperty: unsafe extern "system" fn (this: *const nsIHTMLEditor, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void getInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    pub GetInlineProperty: unsafe extern "system" fn (this: *const nsIHTMLEditor, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] AString getInlinePropertyWithAttrValue (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
    pub GetInlinePropertyWithAttrValue: unsafe extern "system" fn (this: *const nsIHTMLEditor, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void removeInlineProperty (in AString aProperty, in AString aAttribute); */
    pub RemoveInlineProperty: unsafe extern "system" fn (this: *const nsIHTMLEditor, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean nodeIsBlock (in Node node); */
    pub NodeIsBlock: unsafe extern "system" fn (this: *const nsIHTMLEditor, node: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void insertHTML (in AString aInputString); */
    pub InsertHTML: unsafe extern "system" fn (this: *const nsIHTMLEditor, aInputString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void pasteNoFormatting (in long aSelectionType); */
    pub PasteNoFormatting: unsafe extern "system" fn (this: *const nsIHTMLEditor, aSelectionType: i32) -> ::nserror::nsresult,

    /* [can_run_script] void rebuildDocumentFromSource (in AString aSourceString); */
    pub RebuildDocumentFromSource: unsafe extern "system" fn (this: *const nsIHTMLEditor, aSourceString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void insertElementAtSelection (in Element aElement, in boolean aDeleteSelection); */
    pub InsertElementAtSelection: unsafe extern "system" fn (this: *const nsIHTMLEditor, aElement: *const libc::c_void, aDeleteSelection: bool) -> ::nserror::nsresult,

    /* void updateBaseURL (); */
    pub UpdateBaseURL: unsafe extern "system" fn (this: *const nsIHTMLEditor) -> ::nserror::nsresult,

    /* [can_run_script] void selectElement (in Element aElement); */
    pub SelectElement: unsafe extern "system" fn (this: *const nsIHTMLEditor, aElement: *const libc::c_void) -> ::nserror::nsresult,

    /* void setCaretAfterElement (in Element aElement); */
    pub SetCaretAfterElement: unsafe extern "system" fn (this: *const nsIHTMLEditor, aElement: *const libc::c_void) -> ::nserror::nsresult,

    /* AString getParagraphState (out boolean aMixed); */
    pub GetParagraphState: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] AString getFontFaceState (out boolean aMixed); */
    pub GetFontFaceState: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] AString getHighlightColorState (out boolean aMixed); */
    pub GetHighlightColorState: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL); */
    pub GetListState: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aOL: *mut bool, aUL: *mut bool, aDL: *mut bool) -> ::nserror::nsresult,

    /* void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD); */
    pub GetListItemState: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aLI: *mut bool, aDT: *mut bool, aDD: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void getAlignment (out boolean aMixed, out short aAlign); */
    pub GetAlignment: unsafe extern "system" fn (this: *const nsIHTMLEditor, aMixed: *mut bool, aAlign: *mut i16) -> ::nserror::nsresult,

    /* [can_run_script] void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType); */
    pub MakeOrChangeList: unsafe extern "system" fn (this: *const nsIHTMLEditor, aListType: *const ::nsstring::nsAString, entireList: bool, aBulletType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [can_run_script] void removeList (in AString aListType); */
    pub RemoveList: unsafe extern "system" fn (this: *const nsIHTMLEditor, aListType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* Element getElementOrParentByTagName (in AString aTagName, in Node aNode); */
    pub GetElementOrParentByTagName: unsafe extern "system" fn (this: *const nsIHTMLEditor, aTagName: *const ::nsstring::nsAString, aNode: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* nsISupports getSelectedElement (in AString aTagName); */
    pub GetSelectedElement: unsafe extern "system" fn (this: *const nsIHTMLEditor, aTagName: *const ::nsstring::nsAString, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [can_run_script] Element createElementWithDefaults (in AString aTagName); */
    pub CreateElementWithDefaults: unsafe extern "system" fn (this: *const nsIHTMLEditor, aTagName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void insertLinkAroundSelection (in Element aAnchorElement); */
    pub InsertLinkAroundSelection: unsafe extern "system" fn (this: *const nsIHTMLEditor, aAnchorElement: *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void setBackgroundColor (in AString aColor); */
    pub SetBackgroundColor: unsafe extern "system" fn (this: *const nsIHTMLEditor, aColor: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute boolean isCSSEnabled; */
    pub GetIsCSSEnabled: unsafe extern "system" fn (this: *const nsIHTMLEditor, aIsCSSEnabled: *mut bool) -> ::nserror::nsresult,

    /* [setter_can_run_script] attribute boolean isCSSEnabled; */
    pub SetIsCSSEnabled: unsafe extern "system" fn (this: *const nsIHTMLEditor, aIsCSSEnabled: bool) -> ::nserror::nsresult,

    /* [can_run_script] void checkSelectionStateForAnonymousButtons (); */
    pub CheckSelectionStateForAnonymousButtons: unsafe extern "system" fn (this: *const nsIHTMLEditor) -> ::nserror::nsresult,

    /* boolean isAnonymousElement (in Element aElement); */
    pub IsAnonymousElement: unsafe extern "system" fn (this: *const nsIHTMLEditor, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean returnInParagraphCreatesNewParagraph; */
    pub GetReturnInParagraphCreatesNewParagraph: unsafe extern "system" fn (this: *const nsIHTMLEditor, aReturnInParagraphCreatesNewParagraph: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean returnInParagraphCreatesNewParagraph; */
    pub SetReturnInParagraphCreatesNewParagraph: unsafe extern "system" fn (this: *const nsIHTMLEditor, aReturnInParagraphCreatesNewParagraph: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHTMLEditor {

    pub const eLeft: i64 = 0;


    pub const eCenter: i64 = 1;


    pub const eRight: i64 = 2;


    pub const eJustify: i64 = 3;

    /// ```text
    /// /**
    ///    * SetInlineProperty() sets the aggregate properties on the current selection
    ///    *
    ///    * @param aProperty   the property to set on the selection
    ///    * @param aAttribute  the attribute of the property, if applicable.
    ///    *                    May be null.
    ///    *                    Example: aProperty="font", aAttribute="color"
    ///    * @param aValue      if aAttribute is not null, the value of the attribute.
    ///    *                    May be null.
    ///    *                    Example: aProperty="font", aAttribute="color",
    ///    *                             aValue="0x00FFFF"
    ///    */
    /// ```
    ///

    /// `[can_run_script] void setInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue);`
    #[inline]
    pub unsafe fn SetInlineProperty(&self, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetInlineProperty)(self, aProperty, aAttribute, aValue)
    }


    /// ```text
    /// /**
    ///    * getInlineProperty() gets aggregate properties of the current selection.
    ///    * All object in the current selection are scanned and their attributes are
    ///    * represented in a list of Property object.
    ///    *
    ///    * @param aProperty   the property to get on the selection
    ///    * @param aAttribute  the attribute of the property, if applicable.
    ///    *                    May be null.
    ///    *                    Example: aProperty="font", aAttribute="color"
    ///    * @param aValue      if aAttribute is not null, the value of the attribute.
    ///    *                    May be null.
    ///    *                    Example: aProperty="font", aAttribute="color",
    ///    *                             aValue="0x00FFFF"
    ///    * @param aFirst      [OUT] PR_TRUE if the first text node in the
    ///    *                          selection has the property
    ///    * @param aAny        [OUT] PR_TRUE if any of the text nodes in the
    ///    *                          selection have the property
    ///    * @param aAll        [OUT] PR_TRUE if all of the text nodes in the
    ///    *                          selection have the property
    ///    */
    /// ```
    ///

    /// `[can_run_script] void getInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll);`
    #[inline]
    pub unsafe fn GetInlineProperty(&self, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInlineProperty)(self, aProperty, aAttribute, aValue, aFirst, aAny, aAll)
    }



    /// `[can_run_script] AString getInlinePropertyWithAttrValue (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll);`
    #[inline]
    pub unsafe fn GetInlinePropertyWithAttrValue(&self, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString, aValue: *const ::nsstring::nsAString, aFirst: *mut bool, aAny: *mut bool, aAll: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetInlinePropertyWithAttrValue)(self, aProperty, aAttribute, aValue, aFirst, aAny, aAll, _retval)
    }


    /// ```text
    /// /**
    ///    * removeInlineProperty() removes a property which changes inline style of
    ///    * text.  E.g., bold, italic, super and sub.
    ///    *
    ///    * @param aProperty   Tag name whcih represents the inline style you want to
    ///    *                    remove.  E.g., "strong", "b", etc.
    ///    *                    If "href", <a> element which has href attribute will be
    ///    *                    removed.
    ///    *                    If "name", <a> element which has non-empty name
    ///    *                    attribute will be removed.
    ///    * @param aAttribute  If aProperty is "font", aAttribute should be "face",
    ///    *                    "size", "color" or "bgcolor".
    ///    */
    /// ```
    ///

    /// `[can_run_script] void removeInlineProperty (in AString aProperty, in AString aAttribute);`
    #[inline]
    pub unsafe fn RemoveInlineProperty(&self, aProperty: *const ::nsstring::nsAString, aAttribute: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveInlineProperty)(self, aProperty, aAttribute)
    }


    /// ```text
    /// /**
    ///    * Tests if a node is a BLOCK element according the the HTML 4.0 DTD.
    ///    *   This does NOT consider CSS effect on display type
    ///    *
    ///    * @param aNode      the node to test
    ///    */
    /// ```
    ///

    /// `boolean nodeIsBlock (in Node node);`
    #[inline]
    pub unsafe fn NodeIsBlock(&self, node: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).NodeIsBlock)(self, node, _retval)
    }


    /// ```text
    /// /**
    ///    * Insert some HTML source at the current location
    ///    *
    ///    * @param aInputString   the string to be inserted
    ///    */
    /// ```
    ///

    /// `[can_run_script] void insertHTML (in AString aInputString);`
    #[inline]
    pub unsafe fn InsertHTML(&self, aInputString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InsertHTML)(self, aInputString)
    }


    /// ```text
    /// /**
    ///     * Paste the text in the OS clipboard at the cursor position, replacing
    ///     * the selected text (if any), but strip out any HTML styles and formatting
    ///     */
    /// ```
    ///

    /// `[can_run_script] void pasteNoFormatting (in long aSelectionType);`
    #[inline]
    pub unsafe fn PasteNoFormatting(&self, aSelectionType: i32) -> ::nserror::nsresult {
        ((*self.vtable).PasteNoFormatting)(self, aSelectionType)
    }


    /// ```text
    /// /**
    ///    *  Rebuild the entire document from source HTML
    ///    *  Needed to be able to edit HEAD and other outside-of-BODY content
    ///    *
    ///    *  @param aSourceString   HTML source string of the entire new document
    ///    */
    /// ```
    ///

    /// `[can_run_script] void rebuildDocumentFromSource (in AString aSourceString);`
    #[inline]
    pub unsafe fn RebuildDocumentFromSource(&self, aSourceString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RebuildDocumentFromSource)(self, aSourceString)
    }


    /// ```text
    /// /**
    ///     * Insert an element, which may have child nodes, at the selection
    ///     * Used primarily to insert a new element for various insert element dialogs,
    ///     *   but it enforces the HTML 4.0 DTD "CanContain" rules, so it should
    ///     *   be useful for other elements.
    ///     *
    ///     * @param aElement           The element to insert
    ///     * @param aDeleteSelection   Delete the selection before inserting
    ///     *     If aDeleteSelection is PR_FALSE, then the element is inserted
    ///     *     after the end of the selection for all element except
    ///     *     Named Anchors, which insert before the selection
    ///     */
    /// ```
    ///

    /// `[can_run_script] void insertElementAtSelection (in Element aElement, in boolean aDeleteSelection);`
    #[inline]
    pub unsafe fn InsertElementAtSelection(&self, aElement: *const libc::c_void, aDeleteSelection: bool) -> ::nserror::nsresult {
        ((*self.vtable).InsertElementAtSelection)(self, aElement, aDeleteSelection)
    }


    /// ```text
    /// /**
    ///    *   Set the BaseURL for the document to the current URL
    ///    *     but only if the page doesn't have a <base> tag
    ///    *   This should be done after the document URL has changed,
    ///    *     such as after saving a file
    ///    *   This is used as base for relativizing link and image urls
    ///    */
    /// ```
    ///

    /// `void updateBaseURL ();`
    #[inline]
    pub unsafe fn UpdateBaseURL(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateBaseURL)(self, )
    }


    /// ```text
    /// /**
    ///     * Set the selection at the suppled element
    ///     *
    ///     * @param aElement   An element in the document
    ///     */
    /// ```
    ///

    /// `[can_run_script] void selectElement (in Element aElement);`
    #[inline]
    pub unsafe fn SelectElement(&self, aElement: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SelectElement)(self, aElement)
    }


    /// ```text
    /// /**
    ///     * Create a collapsed selection just after aElement
    ///     *
    ///     * XXX could we parameterize SelectElement(before/select/after>?
        ///     *
        ///     * The selection is set to parent-of-aElement with an
        ///     *   offset 1 greater than aElement's offset
        ///     *   but it enforces the HTML 4.0 DTD "CanContain" rules, so it should
        ///     *   be useful for other elements.
        ///     *
        ///     * @param aElement  An element in the document
        ///     */
        /// ```
        ///

        /// `void setCaretAfterElement (in Element aElement);`
        #[inline]
        pub unsafe fn SetCaretAfterElement(&self, aElement: *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).SetCaretAfterElement)(self, aElement)
        }


        /// ```text
        /// /**
        ///    * getParagraphState returns what block tag paragraph format is in
        ///    * the selection.
        ///    * @param aMixed     True if there is more than one format
        ///    * @return           Name of block tag. "" is returned for none.
        ///    */
        /// ```
        ///

        /// `AString getParagraphState (out boolean aMixed);`
        #[inline]
        pub unsafe fn GetParagraphState(&self, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetParagraphState)(self, aMixed, _retval)
        }


        /// ```text
        /// /**
        ///    * getFontFaceState returns what font face is in the selection.
        ///    * @param aMixed    True if there is more than one font face
        ///    * @return          Name of face.  Note: "tt" is returned for
        ///    *                  tt tag.  "" is returned for none.
        ///    */
        /// ```
        ///

        /// `[can_run_script] AString getFontFaceState (out boolean aMixed);`
        #[inline]
        pub unsafe fn GetFontFaceState(&self, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetFontFaceState)(self, aMixed, _retval)
        }


        /// ```text
        /// /**
        ///    * getHighlightColorState returns what the highlight color of the selection.
        ///    * @param aMixed     True if there is more than one font color
        ///    * @return           Color string. "" is returned for none.
        ///    */
        /// ```
        ///

        /// `[can_run_script] AString getHighlightColorState (out boolean aMixed);`
        #[inline]
        pub unsafe fn GetHighlightColorState(&self, aMixed: *mut bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).GetHighlightColorState)(self, aMixed, _retval)
        }


        /// ```text
        /// /**
        ///    * getListState returns what list type is in the selection.
        ///    * @param aMixed    True if there is more than one type of list, or
        ///    *                  if there is some list and non-list
        ///    * @param aOL       The company that employs me.  No, really, it's
        ///    *                  true if an "ol" list is selected.
        ///    * @param aUL       true if an "ul" list is selected.
        ///    * @param aDL       true if a "dl" list is selected.
        ///    */
        /// ```
        ///

        /// `void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL);`
        #[inline]
        pub unsafe fn GetListState(&self, aMixed: *mut bool, aOL: *mut bool, aUL: *mut bool, aDL: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetListState)(self, aMixed, aOL, aUL, aDL)
        }


        /// ```text
        /// /**
        ///    * getListItemState returns what list item type is in the selection.
        ///    * @param aMixed    True if there is more than one type of list item, or
        ///    *                  if there is some list and non-list
        ///    *                  XXX This ignores `<li>` element selected state.
        ///    *                      For example, even if `<li>` and `<dt>` are selected,
        ///    *                      this is set to false.
        ///    * @param aLI       true if "li" list items are selected.
        ///    * @param aDT       true if "dt" list items are selected.
        ///    * @param aDD       true if "dd" list items are selected.
        ///    */
        /// ```
        ///

        /// `void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD);`
        #[inline]
        pub unsafe fn GetListItemState(&self, aMixed: *mut bool, aLI: *mut bool, aDT: *mut bool, aDD: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetListItemState)(self, aMixed, aLI, aDT, aDD)
        }


        /// ```text
        /// /**
        ///    * getAlignment     returns what alignment is in the selection.
        ///    * @param aMixed    Always returns false.
        ///    * @param aAlign    enum value for first encountered alignment
        ///    *                  (left/center/right)
        ///    */
        /// ```
        ///

        /// `[can_run_script] void getAlignment (out boolean aMixed, out short aAlign);`
        #[inline]
        pub unsafe fn GetAlignment(&self, aMixed: *mut bool, aAlign: *mut i16) -> ::nserror::nsresult {
            ((*self.vtable).GetAlignment)(self, aMixed, aAlign)
        }


        /// ```text
        /// /**
        ///    * Document me!
        ///    *
        ///    */
        /// ```
        ///

        /// `[can_run_script] void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType);`
        #[inline]
        pub unsafe fn MakeOrChangeList(&self, aListType: *const ::nsstring::nsAString, entireList: bool, aBulletType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).MakeOrChangeList)(self, aListType, entireList, aBulletType)
        }


        /// ```text
        /// /**
        ///    * removeList removes list items (<li>, <dd>, and <dt>) and list structures
        ///    * (<ul>, <ol>, and <dl>).
        ///    *
        ///    * @param aListType  Unused.
        ///    */
        /// ```
        ///

        /// `[can_run_script] void removeList (in AString aListType);`
        #[inline]
        pub unsafe fn RemoveList(&self, aListType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).RemoveList)(self, aListType)
        }


        /// ```text
        /// /**
        ///    * GetElementOrParentByTagName() returns an inclusive ancestor element whose
        ///    * name matches aTagName from aNode or anchor node of Selection to <body>
        ///    * element or null if there is no element matching with aTagName.
        ///    *
        ///    * @param aTagName        The tag name which you want to look for.
        ///    *                        Must not be empty string.
        ///    *                        If "list", the result may be <ul>, <ol> or <dl>
        ///    *                        element.
        ///    *                        If "td", the result may be <td> or <th>.
        ///    *                        If "href", the result may be <a> element
        ///    *                        which has "href" attribute with non-empty value.
        ///    *                        If "anchor", the result may be <a> which has
        ///    *                        "name" attribute with non-empty value.
        ///    * @param aNode           If non-null, this starts to look for the result
        ///    *                        from it.  Otherwise, i.e., null, starts from
        ///    *                        anchor node of Selection.
        ///    * @return                If an element which matches aTagName, returns
        ///    *                        an Element.  Otherwise, nullptr.
        ///    */
        /// ```
        ///

        /// `Element getElementOrParentByTagName (in AString aTagName, in Node aNode);`
        #[inline]
        pub unsafe fn GetElementOrParentByTagName(&self, aTagName: *const ::nsstring::nsAString, aNode: *const libc::c_void, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).GetElementOrParentByTagName)(self, aTagName, aNode, _retval)
        }


        /// ```text
        /// /**
        ///    * getSelectedElement() returns a "selected" element node.  "selected" means:
        ///    * - there is only one selection range
        ///    * - the range starts from an element node or in an element
        ///    * - the range ends at immediately after same element
        ///    * - and the range does not include any other element nodes.
        ///    * Additionally, only when aTagName is "href", this thinks that an <a>
        ///    * element which has non-empty "href" attribute includes the range, the
        ///    * <a> element is selected.
        ///    *
        ///    * @param aTagName    Case-insensitive element name.
        ///    *                    If empty string, this returns any element node or null.
        ///    *                    If "href", this returns an <a> element which has
        ///    *                    non-empty "href" attribute or null.
        ///    *                    If "anchor", this returns an <a> element which has
        ///    *                    non-empty "name" attribute or null.
        ///    *                    Otherwise, returns an element node whose name is
        ///    *                    same as aTagName or null.
        ///    * @return            A "selected" element.
        ///    */
        /// ```
        ///

        /// `nsISupports getSelectedElement (in AString aTagName);`
        #[inline]
        pub unsafe fn GetSelectedElement(&self, aTagName: *const ::nsstring::nsAString, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
            ((*self.vtable).GetSelectedElement)(self, aTagName, _retval)
        }


        /// ```text
        /// /**
        ///    * Return a new element with default attribute values
        ///    *
        ///    * This does not rely on the selection, and is not sensitive to context.
        ///    *
        ///    * Used primarily to supply new element for various insert element dialogs
        ///    *  (Image, Link, NamedAnchor, Table, and HorizontalRule
            ///    *   are the only returned elements as of 7/25/99)
        ///    *
        ///    * @param aTagName  The HTML tagname
        ///    *    Special input values for Links and Named anchors:
        ///    *    Use "href" to get a link node
        ///    *      (an "A" tag with the "href" attribute set)
        ///    *    Use "anchor" or "namedanchor" to get a named anchor node
        ///    *      (an "A" tag with the "name" attribute set)
        ///    * @return          The new element created.
        ///    */
        /// ```
        ///

        /// `[can_run_script] Element createElementWithDefaults (in AString aTagName);`
        #[inline]
        pub unsafe fn CreateElementWithDefaults(&self, aTagName: *const ::nsstring::nsAString, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).CreateElementWithDefaults)(self, aTagName, _retval)
        }


        /// ```text
        /// /**
        ///    * Insert an link element as the parent of the current selection
        ///    *
        ///    * @param aElement   An "A" element with a non-empty "href" attribute
        ///    */
        /// ```
        ///

        /// `[can_run_script] void insertLinkAroundSelection (in Element aAnchorElement);`
        #[inline]
        pub unsafe fn InsertLinkAroundSelection(&self, aAnchorElement: *const libc::c_void) -> ::nserror::nsresult {
            ((*self.vtable).InsertLinkAroundSelection)(self, aAnchorElement)
        }


        /// ```text
        /// /**
        ///    * Set the value of the "bgcolor" attribute on the document's <body> element
        ///    *
        ///    * @param aColor  The HTML color string, such as "#ffccff" or "yellow"
        ///    */
        /// ```
        ///

        /// `[can_run_script] void setBackgroundColor (in AString aColor);`
        #[inline]
        pub unsafe fn SetBackgroundColor(&self, aColor: *const ::nsstring::nsAString) -> ::nserror::nsresult {
            ((*self.vtable).SetBackgroundColor)(self, aColor)
        }


        /// ```text
        /// /**
        ///    * A boolean which is true is the HTMLEditor has been instantiated
        ///    * with CSS knowledge and if the CSS pref is currently checked
        ///    *
        ///    * @return    true if CSS handled and enabled
        ///    */
        /// ```
        ///

        /// `[setter_can_run_script] attribute boolean isCSSEnabled;`
        #[inline]
        pub unsafe fn GetIsCSSEnabled(&self, aIsCSSEnabled: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetIsCSSEnabled)(self, aIsCSSEnabled)
        }


        /// ```text
        /// /**
        ///    * A boolean which is true is the HTMLEditor has been instantiated
        ///    * with CSS knowledge and if the CSS pref is currently checked
        ///    *
        ///    * @return    true if CSS handled and enabled
        ///    */
        /// ```
        ///

        /// `[setter_can_run_script] attribute boolean isCSSEnabled;`
        #[inline]
        pub unsafe fn SetIsCSSEnabled(&self, aIsCSSEnabled: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetIsCSSEnabled)(self, aIsCSSEnabled)
        }


        /// ```text
        /// /**
        ///    * checkSelectionStateForAnonymousButtons() may refresh editing UI such as
        ///    * resizers, inline-table-editing UI, absolute positioning UI for current
        ///    * Selection and focus state.  When this method shows or hides UI, the
        ///    * editor (and/or its document/window) could be broken by mutation observers.
        ///    * FYI: Current user in script is only BlueGriffon.
        ///    */
        /// ```
        ///

        /// `[can_run_script] void checkSelectionStateForAnonymousButtons ();`
        #[inline]
        pub unsafe fn CheckSelectionStateForAnonymousButtons(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).CheckSelectionStateForAnonymousButtons)(self, )
        }



        /// `boolean isAnonymousElement (in Element aElement);`
        #[inline]
        pub unsafe fn IsAnonymousElement(&self, aElement: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).IsAnonymousElement)(self, aElement, _retval)
        }


        /// ```text
        /// /**
        ///    * A boolean indicating if a return key pressed in a paragraph creates
        ///    * another paragraph or just inserts a <br> at the caret
        ///    *
        ///    * @return    true if CR in a paragraph creates a new paragraph
        ///    */
        /// ```
        ///

        /// `attribute boolean returnInParagraphCreatesNewParagraph;`
        #[inline]
        pub unsafe fn GetReturnInParagraphCreatesNewParagraph(&self, aReturnInParagraphCreatesNewParagraph: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetReturnInParagraphCreatesNewParagraph)(self, aReturnInParagraphCreatesNewParagraph)
        }


        /// ```text
        /// /**
        ///    * A boolean indicating if a return key pressed in a paragraph creates
        ///    * another paragraph or just inserts a <br> at the caret
        ///    *
        ///    * @return    true if CR in a paragraph creates a new paragraph
        ///    */
        /// ```
        ///

        /// `attribute boolean returnInParagraphCreatesNewParagraph;`
        #[inline]
        pub unsafe fn SetReturnInParagraphCreatesNewParagraph(&self, aReturnInParagraphCreatesNewParagraph: bool) -> ::nserror::nsresult {
            ((*self.vtable).SetReturnInParagraphCreatesNewParagraph)(self, aReturnInParagraphCreatesNewParagraph)
        }


    }


