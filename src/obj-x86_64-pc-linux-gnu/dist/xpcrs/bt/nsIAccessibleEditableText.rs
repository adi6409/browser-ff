//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleEditableText.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleEditableText",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setTextContents (in AString text); */
                    Method {
                        name: "SetTextContents",
                        params: &[Param { name: "text", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void insertText (in AString text, in long position); */
                    Method {
                        name: "InsertText",
                        params: &[Param { name: "text", ty: "*const ::nsstring::nsAString" }, Param { name: "position", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void copyText (in long startPos, in long endPos); */
                    Method {
                        name: "CopyText",
                        params: &[Param { name: "startPos", ty: "i32" }, Param { name: "endPos", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cutText (in long startPos, in long endPos); */
                    Method {
                        name: "CutText",
                        params: &[Param { name: "startPos", ty: "i32" }, Param { name: "endPos", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void deleteText (in long startPos, in long endPos); */
                    Method {
                        name: "DeleteText",
                        params: &[Param { name: "startPos", ty: "i32" }, Param { name: "endPos", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void pasteText (in long position); */
                    Method {
                        name: "PasteText",
                        params: &[Param { name: "position", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

