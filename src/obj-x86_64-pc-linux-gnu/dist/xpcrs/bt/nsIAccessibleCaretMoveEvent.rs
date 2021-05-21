//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleCaretMoveEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleCaretMoveEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute long caretOffset; */
                    Method {
                        name: "GetCaretOffset",
                        params: &[Param { name: "aCaretOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isSelectionCollapsed; */
                    Method {
                        name: "GetIsSelectionCollapsed",
                        params: &[Param { name: "aIsSelectionCollapsed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

