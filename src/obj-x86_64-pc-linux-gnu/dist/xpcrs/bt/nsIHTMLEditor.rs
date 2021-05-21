//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLEditor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHTMLEditor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void setInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue); */
                    Method {
                        name: "SetInlineProperty",
                        params: &[Param { name: "aProperty", ty: "*const ::nsstring::nsAString" }, Param { name: "aAttribute", ty: "*const ::nsstring::nsAString" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void getInlineProperty (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
                    Method {
                        name: "GetInlineProperty",
                        params: &[Param { name: "aProperty", ty: "*const ::nsstring::nsAString" }, Param { name: "aAttribute", ty: "*const ::nsstring::nsAString" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aFirst", ty: "*mut bool" }, Param { name: "aAny", ty: "*mut bool" }, Param { name: "aAll", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] AString getInlinePropertyWithAttrValue (in AString aProperty, in AString aAttribute, in AString aValue, out boolean aFirst, out boolean aAny, out boolean aAll); */
                    Method {
                        name: "GetInlinePropertyWithAttrValue",
                        params: &[Param { name: "aProperty", ty: "*const ::nsstring::nsAString" }, Param { name: "aAttribute", ty: "*const ::nsstring::nsAString" }, Param { name: "aValue", ty: "*const ::nsstring::nsAString" }, Param { name: "aFirst", ty: "*mut bool" }, Param { name: "aAny", ty: "*mut bool" }, Param { name: "aAll", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void removeInlineProperty (in AString aProperty, in AString aAttribute); */
                    Method {
                        name: "RemoveInlineProperty",
                        params: &[Param { name: "aProperty", ty: "*const ::nsstring::nsAString" }, Param { name: "aAttribute", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean nodeIsBlock (in Node node); */
                    Method {
                        name: "NodeIsBlock",
                        params: &[Param { name: "node", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void insertHTML (in AString aInputString); */
                    Method {
                        name: "InsertHTML",
                        params: &[Param { name: "aInputString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void pasteNoFormatting (in long aSelectionType); */
                    Method {
                        name: "PasteNoFormatting",
                        params: &[Param { name: "aSelectionType", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void rebuildDocumentFromSource (in AString aSourceString); */
                    Method {
                        name: "RebuildDocumentFromSource",
                        params: &[Param { name: "aSourceString", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void insertElementAtSelection (in Element aElement, in boolean aDeleteSelection); */
                    Method {
                        name: "InsertElementAtSelection",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aDeleteSelection", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void updateBaseURL (); */
                    Method {
                        name: "UpdateBaseURL",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void selectElement (in Element aElement); */
                    Method {
                        name: "SelectElement",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setCaretAfterElement (in Element aElement); */
                    Method {
                        name: "SetCaretAfterElement",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getParagraphState (out boolean aMixed); */
                    Method {
                        name: "GetParagraphState",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] AString getFontFaceState (out boolean aMixed); */
                    Method {
                        name: "GetFontFaceState",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] AString getHighlightColorState (out boolean aMixed); */
                    Method {
                        name: "GetHighlightColorState",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getListState (out boolean aMixed, out boolean aOL, out boolean aUL, out boolean aDL); */
                    Method {
                        name: "GetListState",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "aOL", ty: "*mut bool" }, Param { name: "aUL", ty: "*mut bool" }, Param { name: "aDL", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getListItemState (out boolean aMixed, out boolean aLI, out boolean aDT, out boolean aDD); */
                    Method {
                        name: "GetListItemState",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "aLI", ty: "*mut bool" }, Param { name: "aDT", ty: "*mut bool" }, Param { name: "aDD", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void getAlignment (out boolean aMixed, out short aAlign); */
                    Method {
                        name: "GetAlignment",
                        params: &[Param { name: "aMixed", ty: "*mut bool" }, Param { name: "aAlign", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void makeOrChangeList (in AString aListType, in boolean entireList, in AString aBulletType); */
                    Method {
                        name: "MakeOrChangeList",
                        params: &[Param { name: "aListType", ty: "*const ::nsstring::nsAString" }, Param { name: "entireList", ty: "bool" }, Param { name: "aBulletType", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void removeList (in AString aListType); */
                    Method {
                        name: "RemoveList",
                        params: &[Param { name: "aListType", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element getElementOrParentByTagName (in AString aTagName, in Node aNode); */
                    Method {
                        name: "GetElementOrParentByTagName",
                        params: &[Param { name: "aTagName", ty: "*const ::nsstring::nsAString" }, Param { name: "aNode", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getSelectedElement (in AString aTagName); */
                    Method {
                        name: "GetSelectedElement",
                        params: &[Param { name: "aTagName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] Element createElementWithDefaults (in AString aTagName); */
                    Method {
                        name: "CreateElementWithDefaults",
                        params: &[Param { name: "aTagName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void insertLinkAroundSelection (in Element aAnchorElement); */
                    Method {
                        name: "InsertLinkAroundSelection",
                        params: &[Param { name: "aAnchorElement", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void setBackgroundColor (in AString aColor); */
                    Method {
                        name: "SetBackgroundColor",
                        params: &[Param { name: "aColor", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [setter_can_run_script] attribute boolean isCSSEnabled; */
                    Method {
                        name: "GetIsCSSEnabled",
                        params: &[Param { name: "aIsCSSEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIsCSSEnabled",
                        params: &[Param { name: "aIsCSSEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void checkSelectionStateForAnonymousButtons (); */
                    Method {
                        name: "CheckSelectionStateForAnonymousButtons",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isAnonymousElement (in Element aElement); */
                    Method {
                        name: "IsAnonymousElement",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean returnInParagraphCreatesNewParagraph; */
                    Method {
                        name: "GetReturnInParagraphCreatesNewParagraph",
                        params: &[Param { name: "aReturnInParagraphCreatesNewParagraph", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetReturnInParagraphCreatesNewParagraph",
                        params: &[Param { name: "aReturnInParagraphCreatesNewParagraph", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

