//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleRelation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAccessibleRelation",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long relationType; */
                    Method {
                        name: "GetRelationType",
                        params: &[Param { name: "aRelationType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long targetsCount; */
                    Method {
                        name: "GetTargetsCount",
                        params: &[Param { name: "aTargetsCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAccessible getTarget (in unsigned long index); */
                    Method {
                        name: "GetTarget",
                        params: &[Param { name: "index", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIAccessible" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIArray getTargets (); */
                    Method {
                        name: "GetTargets",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

