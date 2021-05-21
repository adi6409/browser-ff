//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleTextChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute long start; */
                    Method {
                        name: "GetStart",
                        params: &[Param { name: "aStart", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isInserted; */
                    Method {
                        name: "GetIsInserted",
                        params: &[Param { name: "aIsInserted", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString modifiedText; */
                    Method {
                        name: "GetModifiedText",
                        params: &[Param { name: "aModifiedText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

