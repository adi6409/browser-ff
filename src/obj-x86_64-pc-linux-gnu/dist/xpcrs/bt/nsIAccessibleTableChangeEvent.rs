//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTableChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTableChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute long rowOrColIndex; */
                    Method {
                        name: "GetRowOrColIndex",
                        params: &[Param { name: "aRowOrColIndex", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long RowsOrColsCount; */
                    Method {
                        name: "GetRowsOrColsCount",
                        params: &[Param { name: "aRowsOrColsCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

