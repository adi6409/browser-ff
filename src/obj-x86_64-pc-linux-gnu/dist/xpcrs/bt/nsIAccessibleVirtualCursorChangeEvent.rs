//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleVirtualCursorChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleVirtualCursorChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessible oldAccessible; */
                    Method {
                        name: "GetOldAccessible",
                        params: &[Param { name: "aOldAccessible", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long oldStartOffset; */
                    Method {
                        name: "GetOldStartOffset",
                        params: &[Param { name: "aOldStartOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long oldEndOffset; */
                    Method {
                        name: "GetOldEndOffset",
                        params: &[Param { name: "aOldEndOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible newAccessible; */
                    Method {
                        name: "GetNewAccessible",
                        params: &[Param { name: "aNewAccessible", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long newStartOffset; */
                    Method {
                        name: "GetNewStartOffset",
                        params: &[Param { name: "aNewStartOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long newEndOffset; */
                    Method {
                        name: "GetNewEndOffset",
                        params: &[Param { name: "aNewEndOffset", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short reason; */
                    Method {
                        name: "GetReason",
                        params: &[Param { name: "aReason", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short boundaryType; */
                    Method {
                        name: "GetBoundaryType",
                        params: &[Param { name: "aBoundaryType", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

