//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleValue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleValue",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute double maximumValue; */
                    Method {
                        name: "GetMaximumValue",
                        params: &[Param { name: "aMaximumValue", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double minimumValue; */
                    Method {
                        name: "GetMinimumValue",
                        params: &[Param { name: "aMinimumValue", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute double currentValue; */
                    Method {
                        name: "GetCurrentValue",
                        params: &[Param { name: "aCurrentValue", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentValue",
                        params: &[Param { name: "aCurrentValue", ty: "libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double minimumIncrement; */
                    Method {
                        name: "GetMinimumIncrement",
                        params: &[Param { name: "aMinimumIncrement", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

