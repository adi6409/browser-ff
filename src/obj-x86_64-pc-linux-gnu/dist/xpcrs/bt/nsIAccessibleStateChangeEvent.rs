//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleStateChangeEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleStateChangeEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute unsigned long state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isExtraState; */
                    Method {
                        name: "GetIsExtraState",
                        params: &[Param { name: "aIsExtraState", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isEnabled; */
                    Method {
                        name: "GetIsEnabled",
                        params: &[Param { name: "aIsEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

