//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHideEvent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleHideEvent",
            base: Some("nsIAccessibleEvent"),
            methods: Ok(&[
                    /* readonly attribute nsIAccessible targetParent; */
                    Method {
                        name: "GetTargetParent",
                        params: &[Param { name: "aTargetParent", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible targetNextSibling; */
                    Method {
                        name: "GetTargetNextSibling",
                        params: &[Param { name: "aTargetNextSibling", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIAccessible targetPrevSibling; */
                    Method {
                        name: "GetTargetPrevSibling",
                        params: &[Param { name: "aTargetPrevSibling", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

