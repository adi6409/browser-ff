//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextSelectionChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTextSelectionChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute nsIArray selectionRanges; */
                    Method {
                        name: "GetSelectionRanges",
                        params: &[Param { name: "aSelectionRanges", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

