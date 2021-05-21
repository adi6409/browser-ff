//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleScrollingEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleScrollingEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute unsigned long scrollX; */
                    Method {
                        name: "GetScrollX",
                        params: &[Param { name: "aScrollX", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long scrollY; */
                    Method {
                        name: "GetScrollY",
                        params: &[Param { name: "aScrollY", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maxScrollX; */
                    Method {
                        name: "GetMaxScrollX",
                        params: &[Param { name: "aMaxScrollX", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maxScrollY; */
                    Method {
                        name: "GetMaxScrollY",
                        params: &[Param { name: "aMaxScrollY", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

