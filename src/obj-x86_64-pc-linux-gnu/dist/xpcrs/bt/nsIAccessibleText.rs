//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleText",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute long caretOffset; */
                    Method {
                        name: "GetCaretOffset",
                        params: &[Param { name: "aCaretOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCaretOffset",
                        params: &[Param { name: "aCaretOffset", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long characterCount; */
                    Method {
                        name: "GetCharacterCount",
                        params: &[Param { name: "aCharacterCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long selectionCount; */
                    Method {
                        name: "GetSelectionCount",
                        params: &[Param { name: "aSelectionCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getText (in long startOffset, in long endOffset); */
                    Method {
                        name: "GetText",
                        params: &[Param { name: "startOffset", ty: "i32" }, Param { name: "endOffset", ty: "i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getTextAfterOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "GetTextAfterOffset",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut i32" }, Param { name: "endOffset", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getTextAtOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "GetTextAtOffset",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut i32" }, Param { name: "endOffset", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getTextBeforeOffset (in long offset, in AccessibleTextBoundary boundaryType, out long startOffset, out long endOffset); */
                    Method {
                        name: "GetTextBeforeOffset",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "boundaryType", ty: "AccessibleTextBoundary" }, Param { name: "startOffset", ty: "*mut i32" }, Param { name: "endOffset", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* wchar getCharacterAtOffset (in long offset); */
                    Method {
                        name: "GetCharacterAtOffset",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPersistentProperties getTextAttributes (in boolean includeDefAttrs, in long offset, out long rangeStartOffset, out long rangeEndOffset); */
                    Method {
                        name: "GetTextAttributes",
                        params: &[Param { name: "includeDefAttrs", ty: "bool" }, Param { name: "offset", ty: "i32" }, Param { name: "rangeStartOffset", ty: "*mut i32" }, Param { name: "rangeEndOffset", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut*const nsIPersistentProperties" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPersistentProperties defaultTextAttributes; */
                    Method {
                        name: "GetDefaultTextAttributes",
                        params: &[Param { name: "aDefaultTextAttributes", ty: "*mut*const nsIPersistentProperties" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getCharacterExtents (in long offset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
                    Method {
                        name: "GetCharacterExtents",
                        params: &[Param { name: "offset", ty: "i32" }, Param { name: "x", ty: "*mut i32" }, Param { name: "y", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }, Param { name: "coordType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getRangeExtents (in long startOffset, in long endOffset, out long x, out long y, out long width, out long height, in unsigned long coordType); */
                    Method {
                        name: "GetRangeExtents",
                        params: &[Param { name: "startOffset", ty: "i32" }, Param { name: "endOffset", ty: "i32" }, Param { name: "x", ty: "*mut i32" }, Param { name: "y", ty: "*mut i32" }, Param { name: "width", ty: "*mut i32" }, Param { name: "height", ty: "*mut i32" }, Param { name: "coordType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getOffsetAtPoint (in long x, in long y, in unsigned long coordType); */
                    Method {
                        name: "GetOffsetAtPoint",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "coordType", ty: "u32" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getSelectionBounds (in long selectionNum, out long startOffset, out long endOffset); */
                    Method {
                        name: "GetSelectionBounds",
                        params: &[Param { name: "selectionNum", ty: "i32" }, Param { name: "startOffset", ty: "*mut i32" }, Param { name: "endOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSelectionBounds (in long selectionNum, in long startOffset, in long endOffset); */
                    Method {
                        name: "SetSelectionBounds",
                        params: &[Param { name: "selectionNum", ty: "i32" }, Param { name: "startOffset", ty: "i32" }, Param { name: "endOffset", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addSelection (in long startOffset, in long endOffset); */
                    Method {
                        name: "AddSelection",
                        params: &[Param { name: "startOffset", ty: "i32" }, Param { name: "endOffset", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeSelection (in long selectionNum); */
                    Method {
                        name: "RemoveSelection",
                        params: &[Param { name: "selectionNum", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void scrollSubstringTo (in long startIndex, in long endIndex, in unsigned long scrollType); */
                    Method {
                        name: "ScrollSubstringTo",
                        params: &[Param { name: "startIndex", ty: "i32" }, Param { name: "endIndex", ty: "i32" }, Param { name: "scrollType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void scrollSubstringToPoint (in long startIndex, in long endIndex, in unsigned long coordinateType, in long x, in long y); */
                    Method {
                        name: "ScrollSubstringToPoint",
                        params: &[Param { name: "startIndex", ty: "i32" }, Param { name: "endIndex", ty: "i32" }, Param { name: "coordinateType", ty: "u32" }, Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleTextRange enclosingRange; */
                    Method {
                        name: "GetEnclosingRange",
                        params: &[Param { name: "aEnclosingRange", ty: "*mut*const nsIAccessibleTextRange" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray selectionRanges; */
                    Method {
                        name: "GetSelectionRanges",
                        params: &[Param { name: "aSelectionRanges", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIArray visibleRanges; */
                    Method {
                        name: "GetVisibleRanges",
                        params: &[Param { name: "aVisibleRanges", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleTextRange getRangeByChild (in nsIAccessible child); */
                    Method {
                        name: "GetRangeByChild",
                        params: &[Param { name: "child", ty: "*const nsIAccessible" }, Param { name: "_retval", ty: "*mut*const nsIAccessibleTextRange" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessibleTextRange getRangeAtPoint (in long x, in long y); */
                    Method {
                        name: "GetRangeAtPoint",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIAccessibleTextRange" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

