//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleEvent",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long eventType; */
                    Method {
                        name: "GetEventType",
                        params: &[Param { name: "aEventType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible accessible; */
                    Method {
                        name: "GetAccessible",
                        params: &[Param { name: "aAccessible", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessibleDocument accessibleDocument; */
                    Method {
                        name: "GetAccessibleDocument",
                        params: &[Param { name: "aAccessibleDocument", ty: "*mut*const nsIAccessibleDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Node DOMNode; */
                    Method {
                        name: "GetDOMNode",
                        params: &[Param { name: "aDOMNode", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isFromUserInput; */
                    Method {
                        name: "GetIsFromUserInput",
                        params: &[Param { name: "aIsFromUserInput", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

